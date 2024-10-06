pub mod commands;
mod model;

pub use model::*;

use sqlx::{Pool, Sqlite};

pub struct Context {
    pub db: Pool<Sqlite>,
}
