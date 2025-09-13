use std::collections::HashMap;

use axum::{
    extract::{Path, State},
    Json,
};
use chrono::{DateTime, Utc};
use serde::Serialize;
use souvenir::Id;

use crate::{ApiResult, Application};

#[derive(Serialize)]
pub struct ApiResponse {
    id: Id,
    name: Option<String>,
    created_at: DateTime<Utc>,
    entries: HashMap<Id, Vec<ApiSubject>>,
}

#[derive(Serialize)]
pub struct ApiSubject {
    id: Id,
    tag: String,
    name: Option<String>,
}

pub async fn availability(
    State(state): State<Application>,
    Path(id): Path<String>,
) -> ApiResult<ApiResponse> {
    let metadata = sqlx::query!(
        r#"
            SELECT id AS "id: Id", name, created_at FROM availability 
                WHERE id = $1
                LIMIT 1;
        "#,
        Id::parse(&id)? as Id,
    )
    .fetch_one(&state.pool)
    .await?;

    let mut map: HashMap<Id, Vec<ApiSubject>> = HashMap::new();

    sqlx::query!(
        r#"
            SELECT 
                slot_id AS "slot: Id", 
                subject_id AS "subject: Id", 
                subject.tag,
                subject.name
            FROM availability_entry
                INNER JOIN subject ON subject_id = subject.id 
                WHERE availability_id = $1;
        "#,
        metadata.id as Id,
    )
    .fetch_all(&state.pool)
    .await?
    .into_iter()
    .for_each(|e| {
        map.entry(e.slot).or_default().push(ApiSubject {
            id: e.subject,
            name: e.name,
            tag: e.tag,
        })
    });

    Ok(Json(ApiResponse {
        id: metadata.id,
        name: metadata.name,
        created_at: metadata.created_at,
        entries: map,
    }))
}
