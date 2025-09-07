use axum::{routing::get, Router};

use crate::Application;

mod availabilities;
mod availability;

pub fn create_router() -> Router<Application> {
    Router::new().route("/availabilities", get(availabilities::availabilities))
}
