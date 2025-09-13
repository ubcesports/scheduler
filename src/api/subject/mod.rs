use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use souvenir::Id;

use crate::{ApiResult, Application, Subject};

#[derive(Clone, Serialize)]
struct ApiSubject {
    id: Id,
    tag: String,
    name: Option<String>,
}

async fn subjects(State(state): State<Application>) -> ApiResult<Vec<ApiSubject>> {
    let mut conn = state.pool.acquire().await?;
    let result = Subject::all_subjects(&mut conn).await?;

    Ok(Json(
        result
            .into_iter()
            .map(|Subject { id, tag, name }| ApiSubject { id, tag, name })
            .collect(),
    ))
}

async fn subject(
    State(state): State<Application>,
    Path(id): Path<String>,
) -> ApiResult<ApiSubject> {
    let id = Id::parse(&id)?;

    let mut conn = state.pool.acquire().await?;
    Ok(Json(Subject::find(id, &mut conn).await.map(
        |Subject { id, tag, name }| ApiSubject { id, tag, name },
    )?))
}

#[derive(Clone, Deserialize)]
struct AssociationEntry {
    tag: String,
    name: String,
}

async fn associate(
    State(state): State<Application>,
    Json(body): Json<Vec<AssociationEntry>>,
) -> ApiResult<Value> {
    for entry in body {
        sqlx::query!(
            "UPDATE subject SET name = $1 WHERE tag = $2;",
            entry.name,
            entry.tag
        )
        .execute(&state.pool)
        .await?;
    }

    Ok(Json(json!({ "status": "ok" })))
}

pub fn create_router() -> Router<Application> {
    Router::new()
        .route("/subjects", get(subjects))
        .route("/subject/{id}", get(subject))
        .route("/subjects/associate", post(associate))
}
