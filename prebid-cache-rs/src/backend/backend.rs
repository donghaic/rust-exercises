use anyhow::Result;
use async_trait::async_trait;
use enum_dispatch::*;

use crate::backend::redis::Redis;

#[async_trait]
pub trait Backend: Send + Sync {
    async fn put(&mut self, key: &str, value: &[u8], ttl: u32) -> Result<()>;

    async fn get(&self, key: &str) -> Result<Option<Vec<u8>>>;
}
