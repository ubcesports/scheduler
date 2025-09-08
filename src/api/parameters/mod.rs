use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use souvenir::Id;

use crate::{ApiResult, Application};

#[derive(Clone, Serialize)]
struct ApiParameters {
    version: i32,
    availability: Option<Id>,
    schedule: Option<Id>,
}

async fn get_parameters(State(state): State<Application>) -> ApiResult<ApiParameters> {
    Ok(Json(
        sqlx::query_as!(
            ApiParameters,
            r#"
                SELECT 
                    version, 
                    availability AS "availability: _", 
                    schedule AS "schedule: _"
                FROM parameters;
            "#
        )
        .fetch_one(&state.pool)
        .await?,
    ))
}

#[derive(Clone, Deserialize)]
struct ApiBody {
    availability: Option<Id>,
    schedule: Option<Id>,
}

async fn post_parameters(
    State(state): State<Application>,
    Json(body): Json<ApiBody>,
) -> ApiResult<Value> {
    let mut conn = state.pool.begin().await?;

    if let Some(availability) = body.availability {
        sqlx::query!(
            "UPDATE parameters SET availability = $1",
            availability as Id
        )
        .execute(&mut *conn)
        .await?;
    }

    if let Some(schedule) = body.schedule {
        sqlx::query!("UPDATE parameters SET schedule = $1", schedule as Id)
            .execute(&mut *conn)
            .await?;
    }

    conn.commit().await?;
    Ok(Json(json!({ "result": "ok" })))
}

pub fn create_router() -> Router<Application> {
    Router::new()
        .route("/parameters", get(get_parameters))
        .route("/parameters", post(post_parameters))
}
