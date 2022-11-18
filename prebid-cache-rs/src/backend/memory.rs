use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use anyhow::anyhow;
use async_trait::async_trait;

use crate::backend::backend::Backend;

type MemoryInner = Arc<RwLock<HashMap<String, Vec<u8>>>>;

pub(crate) struct Memory {
    memory: MemoryInner,
}

impl Memory {
    pub(crate) fn new() -> Memory {
        Self {
            memory: Default::default(),
        }
    }
}

#[async_trait]
impl Backend for Memory {
    async fn put(&mut self, key: &str, value: &[u8], _ttl: u32) -> anyhow::Result<()> {
        self.memory.write().unwrap().insert(key.to_string(), value.to_vec());
        Ok(())
    }

    async fn get(&self, key: &str) -> anyhow::Result<Option<Vec<u8>>> {
        let res = self.memory.read().unwrap().get(key).map(|v| v.clone());
        Ok(res)
    }
}


#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::*;

    #[tokio::test]
    async fn test_memory() -> Result<()> {
        let mut memory = Memory::new();
        memory.put("hello", "world".as_bytes(), 10).await?;

        let x = memory.get("hello").await?;
        println!("{:?}", x);

        assert_eq!(String::from_utf8(x.unwrap_or_default()).unwrap(), "world".to_string());

        Ok(())
    }
}