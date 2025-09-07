use std::{ops::Deref, sync::Arc};

use axum::{
    response::{IntoResponse, Response},
    Json, Router,
};
use reqwest::StatusCode;
use serde_json::json;
use sqlx::PgPool;

use crate::Config;

mod availability;
mod health;
mod parameters;
mod schedule;
mod slot;
mod subject;

pub type ApiResult<T> = Result<Json<T>, ApiError>;

pub struct ApiError {
    status_code: StatusCode,
    error: anyhow::Error,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (
            self.status_code,
            Json(json!({
                "error": self.error.to_string(),
            })),
        )
            .into_response()
    }
}

impl<E: Into<anyhow::Error>> From<E> for ApiError {
    fn from(value: E) -> Self {
        Self {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error: value.into(),
        }
    }
}

#[derive(Clone)]
pub struct Application(Arc<ApplicationData>);

impl Application {
    pub fn new(data: ApplicationData) -> Self {
        Self(Arc::new(data))
    }
}

impl Deref for Application {
    type Target = ApplicationData;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct ApplicationData {
    pub config: Config,
    pub pool: PgPool,
}

pub fn create_router(app: ApplicationData) -> Router {
    Router::new()
        .merge(availability::create_router())
        .merge(health::create_router())
        .merge(parameters::create_router())
        .merge(slot::create_router())
        .merge(subject::create_router())
        .with_state(Application::new(app))
}
