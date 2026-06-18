use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

use serde_json::Value;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct SectionCache {
    inner: Arc<RwLock<HashMap<String, CacheEntry>>>,
    ttl: Duration,
}

#[derive(Clone)]
struct CacheEntry {
    value: Value,
    stored_at: Instant,
}

impl SectionCache {
    pub fn new(ttl: Duration) -> Self {
        Self {
            inner: Arc::new(RwLock::new(HashMap::new())),
            ttl,
        }
    }

    pub async fn get(&self, key: &str) -> Option<Value> {
        let guard = self.inner.read().await;
        let entry = guard.get(key)?;
        if entry.stored_at.elapsed() < self.ttl {
            Some(entry.value.clone())
        } else {
            tracing::debug!(key, "cache entry expired");
            None
        }
    }

    pub async fn insert(&self, key: String, value: Value) {
        let mut guard = self.inner.write().await;
        let size = guard.len() + 1;
        guard.insert(
            key.clone(),
            CacheEntry {
                value,
                stored_at: Instant::now(),
            },
        );
        tracing::debug!(key = %key, size, "cache entry stored");
    }
}
