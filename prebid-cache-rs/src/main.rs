use std::sync::Arc;

use anyhow::Result;

use prebid_cache_rs::backend;
use prebid_cache_rs::config::config::Configuration;
use prebid_cache_rs::endpoints;
use prebid_cache_rs::metrics;

#[tokio::main]
async fn main() -> Result<()> {

    println!("Hello, world!");
    let cfg = Configuration::new();
    let metrics = Arc::new(metrics::Metrics::new());
    let backend = backend::build(cfg, metrics.clone())?;

    endpoints::serve(backend, metrics.clone()).await?;

    Ok(())
}
