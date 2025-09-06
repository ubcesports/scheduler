pub mod commands;
mod model;

pub use model::*;

use sqlx::{Acquire, PgPool, Postgres};

pub struct Context {
    pub db: PgPool,
}

pub trait Tx<'a>
where
    Self: Acquire<'a, Database = Postgres>,
{
}

impl<'a, T: Acquire<'a, Database = Postgres>> Tx<'a> for T {}
