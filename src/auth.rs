use jsonwebtoken::{decode, decode_header, Algorithm, Validation};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::jwk_cache::{JwksCache, JwksCacheError};

#[derive(Debug, Deserialize, Serialize)]
pub struct CognitoClaims {
    pub sub: String,
    pub iss: String,
    pub exp: usize,
    pub iat: usize,
    pub token_use: String,
    pub aud: Option<String>,
    pub client_id: Option<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum VerifyError {
    #[error("invalid jwt header")]
    InvalidHeader,
    #[error("missing kid in jwt header")]
    MissingKid,
    #[error("unsupported alg (expected RS256)")]
    UnsupportedAlg,
    #[error("jwks cache error: {0}")]
    Jwks(#[from] JwksCacheError),
    #[error("jwt verify failed: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),
    #[error("issuer mismatch")]
    IssuerMismatch,
    #[error("audience/client_id mismatch")]
    AudienceMismatch,
    #[error("token_use mismatch")]
    TokenUseMismatch,
}

pub async fn verify_cognito_jwt_cached(
    token: &str,
    cache: Arc<JwksCache>,
) -> Result<CognitoClaims, VerifyError> {
    let header = decode_header(token).map_err(|_| VerifyError::InvalidHeader)?;
    let kid = header.kid.ok_or(VerifyError::MissingKid)?;
    if header.alg != Algorithm::RS256 {
        return Err(VerifyError::UnsupportedAlg);
    }

    let region = &cache.region;
    let user_pool_id = &cache.user_pool_id;

    let decoding_key = cache.key_for_kid(&kid).await?;

    let issuer = format!(
        "https://cognito-idp.{region}.amazonaws.com/{user_pool_id}"
    );

    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_issuer(&[issuer.as_str()]);
    validation.validate_exp = true;

    let token_data = decode::<CognitoClaims>(token, &decoding_key, &validation)?;
    let claims = token_data.claims;

    if claims.iss != issuer {
        return Err(VerifyError::IssuerMismatch);
    }

    if claims.token_use != "access" {
        return Err(VerifyError::TokenUseMismatch);
    }

    if claims.client_id.as_deref() != Some(&cache.cognito_app_client_id) {
        return Err(VerifyError::AudienceMismatch);
    }

    Ok(claims)
}

pub async fn check_auth(
    req: &actix_web::HttpRequest,
    cache: Arc<JwksCache>,
) -> Result<CognitoClaims, VerifyError> {
    let auth_header = match req.headers().get("Authorization") {
        Some(header_value) => header_value.to_str().unwrap_or(""),
        None => "",
    };

    if auth_header.is_empty() {
        return Err(VerifyError::Jwt(jsonwebtoken::errors::Error::from(
            jsonwebtoken::errors::ErrorKind::InvalidToken,
        )));
    }

    let token = auth_header.strip_prefix("Bearer ").unwrap_or(auth_header);

    verify_cognito_jwt_cached(token, cache).await
}
