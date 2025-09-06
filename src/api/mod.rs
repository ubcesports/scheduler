use std::{ops::Deref, sync::Arc};

use axum::Router;
use sqlx::PgPool;

use crate::Config;

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
        &*self.0
    }
}

pub struct ApplicationData {
    pub config: Config,
    pub pool: PgPool,
}

pub fn create_router(app: ApplicationData) -> Router {
    Router::new().with_state(Application::new(app))
}
