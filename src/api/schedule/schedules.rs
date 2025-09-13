use axum::{extract::State, Json};
use serde::Serialize;
use souvenir::Id;

use crate::{api::ApiResult, Application};

#[derive(Serialize)]
pub struct ApiResponse {
    pub id: Id,
    pub name: Option<String>,
    pub parent: Option<Id>,
}

pub async fn schedules(State(app): State<Application>) -> ApiResult<Vec<ApiResponse>> {
    let response = sqlx::query_as!(
        ApiResponse,
        r#"
            SELECT id AS "id: Id", name, parent_id AS "parent: Id" 
                FROM schedule;
        "#
    )
    .fetch_all(&app.pool)
    .await?;

    Ok(Json(response))
}
