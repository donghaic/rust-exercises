use std::net::SocketAddr;
use std::sync::Arc;

use anyhow::Context;
use anyhow::Result;

use crate::backend::Backend;
use crate::config::config::Configuration;
use crate::endpoints::handlers::api_routes;
use crate::metrics::Metrics;

mod handlers;

pub async fn serve(backend: Box<dyn Backend>, metrics: Arc<Metrics>) -> Result<()> {
    let app = api_routes(backend, metrics);
    axum::Server::bind(&"0.0.0.0:8080".parse()?)
        .serve(app.into_make_service())
        .await
        .context("error running HTTP server")
}