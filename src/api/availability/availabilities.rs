use axum::{extract::State, Json};
use chrono::{DateTime, Utc};
use serde::Serialize;
use souvenir::Id;

use crate::{ApiResult, Application};

#[derive(Serialize)]
pub struct ApiEntry {
    id: Id,
    name: Option<String>,
    created_at: DateTime<Utc>,
}

pub async fn availabilities(State(state): State<Application>) -> ApiResult<Vec<ApiEntry>> {
    let result = sqlx::query_as!(
        ApiEntry,
        r#"
            SELECT id AS "id: Id", name, created_at FROM availability 
                ORDER BY created_at;
        "#
    )
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(result))
}
