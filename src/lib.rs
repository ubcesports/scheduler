mod api;
mod config;
mod model;

pub use api::*;
pub use config::*;
pub use model::*;

use sqlx::{Acquire, Postgres};

/// Trait alias for database connection, transaction, or pool
/// for function signatures.
pub trait Tx<'a>
where
    Self: Acquire<'a, Database = Postgres> + Send,
{
}

impl<'a, T: Acquire<'a, Database = Postgres> + Send> Tx<'a> for T {}
