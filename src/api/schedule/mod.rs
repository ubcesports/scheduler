use axum::{
    routing::{get, post},
    Router,
};

use crate::Application;

mod export;
mod generate;
mod schedule;
mod schedules;

pub fn create_router() -> Router<Application> {
    Router::new()
        .route("/schedules", get(schedules::schedules))
        .route("/schedule/generate", post(generate::generate))
        .route("/schedule/{id}", get(schedule::schedule))
        .route("/schedule/{id}/export", get(export::export))
}
