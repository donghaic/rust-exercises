use std::fmt::format;
use std::time::Duration;

use anyhow::Result;
use async_trait::async_trait;
use bb8_redis::{
    bb8,
    redis::{AsyncCommands, cmd},
    RedisConnectionManager,
};
use bb8_redis::redis::RedisResult;

use crate::backend::Backend;
use crate::config;

pub type Pool = bb8::Pool<RedisConnectionManager>;


pub struct Redis {
    pool: Pool,
}


impl Redis {
    pub async fn from(redis: &config::Redis) -> Result<Redis> {
        let manager = RedisConnectionManager::new(format!("redis://{}:{}", redis.host, redis.port)).unwrap();
        let pool = Pool::builder()
            .connection_timeout(Duration::from_millis(5000))
            .max_size(10).build(manager)
            .await?;

        Ok(Redis {
            pool,
        })
    }
}

#[async_trait]
impl Backend for Redis {
    async fn put(&mut self, key: &str, value: &[u8], ttl: u32) -> Result<()> {
        let mut conn = self.pool.get().await.unwrap();
        conn.set_ex(key, value, ttl as usize).await.map_err(|e| e.into())
    }

    async fn get(&self, key: &str) -> Result<Option<Vec<u8>>> {
        let mut conn = self.pool.get().await?;
        let res: RedisResult<Option<Vec<u8>>> = conn.get(key).await;
        res.map_err(|e| e.into())
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::*;

    #[tokio::test]
    async fn test_redis() -> Result<()> {
        let mut redis = Redis::from(&config::Redis {
            host: "localhost".to_string(),
            port: 6379,
            password: "".to_string(),
            db: 0,
            expiration: 0,
        }).await?;

        redis.put("foo", "bar".as_bytes(), 100).await?;
        let x = redis.get("foo").await?;
        println!("{:?}", x);
        assert_eq!(String::from_utf8(x.unwrap_or_default()).unwrap(), "bar".to_string());

        let notexist = redis.get("no exist").await?;
        println!("{:?}", notexist);
        assert!(notexist.is_none());
        Ok(())
    }
}