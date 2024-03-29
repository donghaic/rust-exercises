use std::sync::{Arc, RwLock};

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, put};
use axum::{Extension, Json, Router};

use crate::backend::memory::Memory;
use crate::backend::Backend;
use crate::metrics::Metrics;

type SharedState = Arc<Box<dyn Backend>>;

pub fn api_routes(backend: Box<dyn Backend>) -> Router {
    let db = SharedState::new(backend);
    Router::new()
        .route("/status", get(health_check))
        .route("/cache", get(get_data))
        .route("/cache1", get(post_data))
        .route("/version", get(health_check))
        .layer(Extension(db))
}

async fn post_data(Extension(state): Extension<SharedState>) -> impl IntoResponse {
    state.put("hello", "world".as_bytes(), 10).await;

    (StatusCode::OK, "Health check passed!".to_string())
}

async fn get_data(Extension(state): Extension<SharedState>) -> impl IntoResponse {
    let res = state.get("hello").await;
    println!("get_data ==={:?}", res);
    (StatusCode::OK, "Health check passed!".to_string())
}

async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "Health check passed!".to_string())
}
