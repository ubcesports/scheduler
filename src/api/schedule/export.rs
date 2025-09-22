use crate::{ApiError, Application};
use axum::extract::{Path, Query, State};
use axum::http::{header, HeaderMap};
use axum::response::IntoResponse;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use souvenir::Id;
use std::fmt::Write;

#[derive(Deserialize)]
pub struct ExportQuery {
    format: Option<String>,
}

#[derive(Serialize)]
pub struct SlotAssignment {
    pub slot_w2m_id: i32,
    pub subject_name: String,
}

pub async fn export(
    State(state): State<Application>,
    Path(id): Path<String>,
    Query(query): Query<ExportQuery>,
) -> Result<impl IntoResponse, crate::api::ApiError> {
    let records = sqlx::query!(
        r#"
            SELECT slot.w2m_id, subject.name, subject.tag FROM schedule_assignment
                INNER JOIN slot ON schedule_assignment.slot_id = slot.id
                INNER JOIN subject ON schedule_assignment.subject_id = subject.id
                WHERE schedule_assignment.schedule_id = $1
                ORDER BY slot.w2m_id ASC, subject.name ASC;
        "#,
        Id::parse(&id)? as Id,
    )
    .fetch_all(&state.pool)
    .await?;

    let assignments: Vec<SlotAssignment> = records
        .into_iter()
        .map(|row| SlotAssignment {
            slot_w2m_id: row.w2m_id.unwrap_or_default(),
            subject_name: row.name.unwrap_or_else(|| row.tag),
        })
        .collect();

    let format_type = query.format.unwrap_or_else(|| "default".to_string());
    let formatted_assignments = match format_type.as_str() {
        "default" => build_csv(&assignments, false),
        "sheets-export" => build_csv(&assignments, true),
        _ => {
            return Err(ApiError {
                status_code: StatusCode::BAD_REQUEST,
                error: anyhow::anyhow!("Bad request: Invalid format parameter"),
            })
        }
    };

    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        "text/csv; charset=utf-8".parse().unwrap(),
    );
    headers.insert(
        header::CONTENT_DISPOSITION,
        format!("attachment; filename=\"schedule-{id}.csv\"")
            .parse()
            .unwrap(),
    );

    Ok((headers, formatted_assignments))
}

fn build_csv(assignments: &[SlotAssignment], sheets_export: bool) -> String {
    const SLOTS_PER_DAY: usize = 5;

    let mut days: Vec<Vec<(&str, &str)>> = vec![Vec::new(); 5];

    for (i, chunk) in assignments.chunks_exact(2).enumerate() {
        let day = i / SLOTS_PER_DAY;
        days[day].push((&chunk[0].subject_name, &chunk[1].subject_name));
    }

    let mut rows: Vec<Vec<&str>> = Vec::new();

    for slot_idx in 0..SLOTS_PER_DAY {
        let mut row1 = Vec::with_capacity(5);
        let mut row2 = Vec::with_capacity(5);

        for day in 0..5 {
            let (s1, s2) = days[day][slot_idx];
            row1.push(s1);
            row2.push(s2);
        }

        rows.push(row1);
        rows.push(row2);

        if sheets_export {
            rows.push(rows[rows.len() - 2].clone());
            rows.push(rows[rows.len() - 2].clone());
        }
    }

    let mut output = String::with_capacity(assignments.len() * 32);
    for row in rows {
        let _ = writeln!(output, "{}", row.join(","));
    }
    output
}
