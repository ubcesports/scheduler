use axum::{
    routing::{get, post},
    Router,
};

use crate::Application;

mod generate;
mod get_schedule;
mod schedules;

pub fn create_router() -> Router<Application> {
    Router::new()
        .route("/schedules", get(schedules::schedules))
        .route("/schedule/generate", post(generate::generate))
        .route("/schedule/{id}", get(get_schedule::get_schedule))
}
