use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use serde::Serialize;
use souvenir::Id;

use crate::{ApiResult, Application, Slot};

#[derive(Clone, Serialize)]
struct ApiSlot {
    id: Id,
    w2m_id: Option<i32>,
}

async fn slots(State(state): State<Application>) -> ApiResult<Vec<ApiSlot>> {
    let mut conn = state.pool.acquire().await?;
    let result = Slot::all_slots(&mut conn).await?;

    Ok(Json(
        result
            .into_iter()
            .map(|Slot { id, w2m_id }| ApiSlot {
                id,
                w2m_id: Some(w2m_id),
            })
            .collect(),
    ))
}

async fn slot(State(state): State<Application>, Path(id): Path<String>) -> ApiResult<ApiSlot> {
    let id = Id::parse(&id)?;
    let result = sqlx::query_as!(
        ApiSlot,
        r#"
            SELECT id as "id: Id", w2m_id FROM slot
                WHERE id = $1;
        "#,
        id as Id,
    )
    .fetch_one(&state.pool)
    .await?;

    Ok(Json(result))
}

pub fn create_router() -> Router<Application> {
    Router::new()
        .route("/slots", get(slots))
        .route("/slot/{id}", get(slot))
}
