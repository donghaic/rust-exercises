use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;

pub use backend::*;

use crate::backend::memory::Memory;
use crate::config::config::Configuration;
use crate::metrics::Metrics;

pub mod backend;
mod decorates;
pub mod memory;
pub mod redis;

pub fn build(cfg: &Configuration, metrics: Arc<Metrics>) -> Result<Box<dyn Backend>> {
    Ok(Box::new(Memory::new()))
}
