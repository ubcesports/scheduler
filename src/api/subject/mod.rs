use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use serde::Serialize;
use souvenir::Id;

use crate::{ApiResult, Application, Subject};

#[derive(Clone, Serialize)]
struct ApiSubject {
    id: Id,
    w2m_id: Option<i32>,
    name: String,
}

async fn subjects(State(state): State<Application>) -> ApiResult<Vec<ApiSubject>> {
    let result = Subject::all_subjects(&state.pool).await?;

    Ok(Json(
        result
            .into_iter()
            .map(|Subject { id, w2m_id, name }| ApiSubject { id, w2m_id, name })
            .collect(),
    ))
}

async fn subject(
    State(state): State<Application>,
    Path(id): Path<String>,
) -> ApiResult<ApiSubject> {
    let id = Id::parse(&id)?;

    Ok(Json(Subject::find(id, &state.pool).await.map(
        |Subject { id, w2m_id, name }| ApiSubject { id, w2m_id, name },
    )?))
}

pub fn create_router() -> Router<Application> {
    Router::new()
        .route("/subjects", get(subjects))
        .route("/subject/{id}", get(subject))
}
