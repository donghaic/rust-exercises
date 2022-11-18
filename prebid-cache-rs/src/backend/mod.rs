use anyhow::Result;
use async_trait::async_trait;

pub use backend::*;

pub mod memory;
pub mod backend;
pub mod redis;
mod decorates;

