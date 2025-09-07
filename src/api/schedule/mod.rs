use axum::{routing::get, Router};

use crate::Application;

mod schedule;
mod schedules;

pub fn create_router() -> Router<Application> {
    Router::new()
        .route("/schedules", get(schedules::schedules))
        .route("/schedule/{id}", get(schedule::schedule))
}
