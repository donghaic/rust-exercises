use std::net::SocketAddr;
use std::sync::Arc;

use anyhow::{Context, Result};
use axum::{routing::get, Router};
use axum_prom::{PrometheusMetrics, PrometheusMetricsBuilder, PrometheusMetricsRegistry};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

use prebid_cache_rs::backend::{self, Backend};
use prebid_cache_rs::config::config::Configuration;
use prebid_cache_rs::endpoints;
use prebid_cache_rs::metrics;

#[tokio::main]
async fn main() -> Result<()> {
    let env_filter = EnvFilter::new(
        std::env::var("RUST_LOG").unwrap_or_else(|_| "prebid_cache=debug,tower_http=debug".into()),
    );

    tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt::layer())
        .init();

    tracing::info!("starting prebid cache server");
    let (prometheus, prometheus_registry) = PrometheusMetricsBuilder::new("prebid_cache")
        .pair()
        .unwrap();

    let cfg = Configuration::load()?;
    let metrics = Arc::new(metrics::Metrics::new(prometheus_registry.clone()));
    let backend = backend::build(&cfg, metrics.clone())?;

    let _ = start_prometheus_server(&cfg, prometheus.clone(), prometheus_registry.clone());

    serve(&cfg, backend, prometheus.clone()).await?;

    Ok(())
}

fn start_prometheus_server(
    cfg: &Configuration,
    prometheus_layer: PrometheusMetrics,
    metric_handle: PrometheusMetricsRegistry,
) -> Result<()> {
    tracing::info!("start_prometheus_server");
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route(
            axum_prom::DEFAULT_ENDPOINT,
            get(|| async move { metric_handle.metrics() }),
        )
        .layer(prometheus_layer);

    // TODO select a unused port for metrics

    let addr: SocketAddr = format!("0.0.0.0:{}", cfg.admin_port).parse()?;
    tokio::spawn(async move {
        // run it with http on localhost:18080
        tracing::info!("admin server listening on {}", &addr);

        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
    });

    Ok(())
}

async fn serve(
    cfg: &Configuration,
    backend: Box<dyn Backend>,
    prometheus_layer: PrometheusMetrics,
) -> Result<()> {
    let app = endpoints::api_routes(backend).layer(prometheus_layer);

    tracing::info!("api server listening on {}", cfg.port);
    axum::Server::bind(&format!("0.0.0.0:{}", cfg.port).parse()?)
        .serve(app.into_make_service())
        .await
        .context("error running HTTP server")
}
