use axum::{extract::State, routing::get, Json, Router};
use reqwest::StatusCode;
use serde_json::{json, Value};

use crate::{Application, Config};

async fn health() -> (StatusCode, Json<Value>) {
    (
        StatusCode::OK,
        Json(json!({
            "status": "ok"
        })),
    )
}

async fn config(State(state): State<Application>) -> Json<Config> {
    Json(state.config.clone())
}

pub fn create_router() -> Router<Application> {
    Router::new()
        .route("/health", get(health))
        .route("/config", get(config))
}
