use std::collections::HashMap;

use axum::{
    extract::{Path, State},
    Json,
};
use serde::Serialize;
use souvenir::Id;

use crate::{api::ApiResult, Application};

#[derive(Serialize)]
pub struct ApiResponse {
    pub id: Id,
    pub parent: Option<Id>,
    pub assignments: HashMap<Id, Vec<ApiAssignment>>,
}

#[derive(Serialize)]
pub struct ApiAssignment {
    pub id: Id,
    pub name: String,
}

pub async fn get_schedule(
    State(state): State<Application>,
    Path(id): Path<String>,
) -> ApiResult<ApiResponse> {
    let metadata = sqlx::query!(
        r#"
            SELECT id AS "id: Id", parent_id AS "parent: Id" FROM schedule 
                WHERE id = $1
                LIMIT 1;
        "#,
        Id::parse(&id)? as Id,
    )
    .fetch_one(&state.pool)
    .await?;

    let mut map: HashMap<Id, Vec<ApiAssignment>> = HashMap::new();

    sqlx::query!(
        r#"
            SELECT 
                slot_id AS "slot: Id", 
                subject_id AS "subject: Id", 
                subject.name
            FROM schedule_assignment
                INNER JOIN subject ON subject_id = subject.id 
                WHERE schedule_id = $1;
        "#,
        metadata.id as Id,
    )
    .fetch_all(&state.pool)
    .await?
    .into_iter()
    .for_each(|e| {
        map.entry(e.slot).or_default().push(ApiAssignment {
            id: e.subject,
            name: e.name,
        })
    });

    Ok(Json(ApiResponse {
        id: metadata.id,
        parent: metadata.parent,
        assignments: map,
    }))
}
