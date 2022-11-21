use std::io;
use std::io::Read;
use std::io::Write;
use std::sync::Arc;

use anyhow::anyhow;
use async_trait::async_trait;

use crate::backend::Backend;
use crate::metrics::Metrics;

pub struct WithMetricsBackend<T> where T: Backend {
    delegate: T,
    metrics: Arc<Metrics>,

}

impl<T> WithMetricsBackend<T> where T: Backend {
    pub fn new(delegate: T, metrics: Arc<Metrics>) -> Self {
        Self { delegate, metrics }
    }
}

#[async_trait]
impl<T> Backend for WithMetricsBackend<T> where T: Backend {
    async fn put(&self, key: &str, value: &[u8], ttl: u32) -> anyhow::Result<()> {
        println!("WithMetricsBackend set ");
        self.metrics.record_get_backend_total();
        self.delegate.put(key, value, ttl).await
    }

    async fn get(&self, key: &str) -> anyhow::Result<Option<Vec<u8>>> {
        println!("WithMetricsBackend get");
        self.metrics.record_put_total();
        self.delegate.get(key).await
    }
}


pub struct SizeCappedBackend<T> where T: Backend {
    max_size: usize,
    delegate: T,
}

impl<T> SizeCappedBackend<T> where T: Backend {
    pub fn new(max_size: usize, delegate: T) -> Self {
        Self { max_size, delegate }
    }
}

#[async_trait]
impl<T> Backend for SizeCappedBackend<T> where T: Backend {
    async fn put(&self, key: &str, value: &[u8], ttl: u32) -> anyhow::Result<()> {
        println!("SizeCappedBackend set ");
        let len = value.len();
        anyhow::ensure!(len > self.max_size || len == 0, "SizeCappedBackend");
        self.delegate.put(key, value, ttl).await
    }

    async fn get(&self, key: &str) -> anyhow::Result<Option<Vec<u8>>> {
        self.delegate.get(key).await
    }
}


pub struct SnappyCompressorBackend<T> where T: Backend {
    delegate: T,
}

impl<T> SnappyCompressorBackend<T> where T: Backend {
    pub fn new(delegate: T) -> Self {
        Self { delegate }
    }
}

#[async_trait]
impl<T> Backend for SnappyCompressorBackend<T> where T: Backend {
    async fn put(&self, key: &str, value: &[u8], ttl: u32) -> anyhow::Result<()> {
        let compressed = frame_press(value)?;
        self.delegate.put(key, &compressed, ttl).await
    }

    async fn get(&self, key: &str) -> anyhow::Result<Option<Vec<u8>>> {
        let res = self.delegate.get(key).await?;
        match res {
            None => Err(anyhow!("No such key")),
            Some(buff) => {
                let result = frame_depress(buff.as_slice());
                result.and_then(|v| Ok(Some(v)))
            }
        }
    }
}

#[inline]
fn frame_press(bytes: &[u8]) -> anyhow::Result<Vec<u8>> {
    use snap::write;

    let mut wtr = write::FrameEncoder::new(vec![]);
    wtr.write_all(bytes)?;
    let result = wtr.into_inner();
    match result {
        Ok(buf) => Ok(buf),
        Err(e) => Err(anyhow!("Error encoding"))
    }
}

#[inline]
fn frame_depress(bytes: &[u8]) -> anyhow::Result<Vec<u8>> {
    use snap::read;

    let mut buf = vec![];
    let result = read::FrameDecoder::new(bytes).read_to_end(&mut buf);
    match result {
        Ok(_) => Ok(buf),
        Err(e) => Err(anyhow!("Error decoding"))
    }
}


#[cfg(test)]
mod tests {
    use anyhow::Result;

    use crate::backend::memory::Memory;

    use super::*;

    #[tokio::test]
    async fn test_with_metrics_backend() -> Result<()> {
        let mut backend = WithMetricsBackend::new(Memory::new(), Arc::new(Metrics {}));
        backend.put("hello", "world".as_bytes(), 10).await?;

        let x = backend.get("hello").await?;
        println!("{:?}", x);

        assert_eq!(String::from_utf8(x.unwrap_or_default()).unwrap(), "world".to_string());

        Ok(())
    }

    #[tokio::test]
    async fn test_snappy_compressor_backend() -> Result<()> {
        let mut backend = SnappyCompressorBackend::new(Memory::new());

        backend.put("hello", "world123asd12123131sa".as_bytes(), 10).await?;
        let x = backend.get("hello").await?;
        println!("{:?}", x);
        println!("{:?}", String::from_utf8_lossy(x.unwrap_or_default().as_slice()));

        Ok(())
    }

    #[test]
    fn test_frame_press() -> Result<()> {
        let data = "hello3hellohello123123hellohello123123hellohellossskdkd3".as_bytes();
        println!("org len {:?}", data.len());

        let a = frame_press(data)?;
        println!("compre len {:?}", a.len());

        println!("{:?}", a.len());
        let b = frame_depress(&a)?;
        println!("dec====={}", String::from_utf8(b).unwrap());
        println!("enc====={}", String::from_utf8_lossy(&a));

        Ok(())
    }
}
