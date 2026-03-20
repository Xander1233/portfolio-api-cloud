use jsonwebtoken::DecodingKey;
use serde::Deserialize;
use std::{collections::HashMap};
use thiserror::Error;
use time::{Duration, OffsetDateTime};
use tokio::sync::RwLock;

#[derive(Debug, Deserialize)]
struct JwkSet {
    keys: Vec<Jwk>,
}

#[derive(Debug, Deserialize)]
struct Jwk {
    kid: String,
    n: String,
    e: String,
}

#[derive(Debug, Error)]
pub enum JwksCacheError {
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("no matching jwk for kid")]
    NoMatchingKey,
    #[error("invalid jwk key material")]
    InvalidKey,
}

struct CachedJwks {
    keys: HashMap<String, DecodingKey>, // kid -> DecodingKey
    fetched_at: OffsetDateTime,
}

pub struct JwksCache {
    pub region: String,
    pub user_pool_id: String,
    pub cognito_app_client_id: String,

    client: reqwest::Client,
    jwks_url: String,
    ttl: Duration,
    inner: RwLock<Option<CachedJwks>>,
}

impl JwksCache {
    pub fn new(region: &str, user_pool_id: &str, ttl: Duration, app_client_id: impl Into<String>) -> Self {
        let jwks_url = cognito_jwks_url(&region, &user_pool_id);

        Self {
            region: region.into(),
            user_pool_id: user_pool_id.into(),
            cognito_app_client_id: app_client_id.into(),
            client: reqwest::Client::new(),
            jwks_url: jwks_url.into(),
            ttl,
            inner: RwLock::new(None),
        }
    }

    fn is_fresh(entry: &CachedJwks, ttl: Duration) -> bool {
        OffsetDateTime::now_utc() - entry.fetched_at < ttl
    }

    async fn fetch_jwks(&self) -> Result<CachedJwks, reqwest::Error> {
        let set: JwkSet = self
            .client
            .get(&self.jwks_url)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        let mut keys = HashMap::new();
        for k in set.keys {
            if let Ok(dk) = DecodingKey::from_rsa_components(&k.n, &k.e) {
                keys.insert(k.kid, dk);
            }
        }

        Ok(CachedJwks {
            keys,
            fetched_at: OffsetDateTime::now_utc(),
        })
    }

    /// Get a DecodingKey for a given kid.
    /// Strategy:
    /// 1) If cache fresh and contains kid -> return
    /// 2) If cache stale -> refresh, then return if found
    /// 3) If fresh but kid missing -> force refresh once (handles key rotation), then return if found
    pub async fn key_for_kid(&self, kid: &str) -> Result<DecodingKey, JwksCacheError> {
        {
            let guard = self.inner.read().await;
            if let Some(entry) = guard.as_ref() {
                if Self::is_fresh(entry, self.ttl) {
                    if let Some(key) = entry.keys.get(kid) {
                        return Ok(key.clone());
                    }
                }
            }
        }

        let mut guard = self.inner.write().await;

        if let Some(entry) = guard.as_ref() {
            if Self::is_fresh(entry, self.ttl) {
                if let Some(key) = entry.keys.get(kid) {
                    return Ok(key.clone());
                }
            }
        }

        let refreshed = self.fetch_jwks().await?;
        let key = refreshed
            .keys
            .get(kid)
            .cloned()
            .ok_or(JwksCacheError::NoMatchingKey)?;

        *guard = Some(refreshed);
        Ok(key)
    }
}

pub fn cognito_jwks_url(region: &str, user_pool_id: &str) -> String {
    format!(
        "https://cognito-idp.{region}.amazonaws.com/{user_pool_id}/.well-known/jwks.json"
    )
}
