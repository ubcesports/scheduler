use axum::{
    routing::{get, post},
    Router,
};

use crate::Application;

mod availabilities;
mod availability;
mod import;

pub fn create_router() -> Router<Application> {
    Router::new()
        .route("/availabilities", get(availabilities::availabilities))
        .route("/availability/import", post(import::import))
        .route("/availability/{id}", get(availability::availability))
}
