use axum::extract::{Path, Query, State};
use axum::http::{HeaderMap, header};
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use souvenir::Id;
use crate::Application;
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
    Query(query): Query<ExportQuery>
) -> Result<impl IntoResponse, crate::api::ApiError> {
    let records = sqlx::query!(
        r#"
            SELECT slot.w2m_id, subject.name FROM schedule_assignment
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
        .filter_map(|row| {
            Some(SlotAssignment {
                slot_w2m_id: row.w2m_id?,
                subject_name: row.name,
            })
        })
        .collect();

    let format_type: Option<String> = query.format;
    let formatted_assignments = match format_type.as_deref() {
        Some("sheets-export") => build_csv(&assignments, true).await,
        _ => build_csv(&assignments, false).await,
    };

    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        "text/plain; charset=utf-8".parse().unwrap(),
    );
    headers.insert(
        header::CONTENT_DISPOSITION,
        format!("attachment; filename=\"schedule-{id}.txt\"")
            .parse()
            .unwrap(),
    );

    Ok((headers, formatted_assignments))
}

async fn build_csv(assignments: &[SlotAssignment], sheets_export: bool) -> String {
    let slots_per_day = 5;
    let mut output = String::with_capacity(assignments.len() * 32);

    let mut days: Vec<Vec<(&str, &str)>> = vec![Vec::new(); 5];

    for (i, chunk) in assignments.chunks_exact(2).enumerate() {
        let day = i / slots_per_day;
        days[day].push((&chunk[0].subject_name, &chunk[1].subject_name));
    }

    let mut rows: Vec<Vec<String>> = Vec::new();

    for slot_idx in 0..slots_per_day {
        let mut row1 = Vec::with_capacity(5);
        let mut row2 = Vec::with_capacity(5);

        for day in 0..5 {
            let (s1, s2) = days[day][slot_idx];
            row1.push(s1.to_string());
            row2.push(s2.to_string());
        }

        rows.push(row1);
        rows.push(row2);

        if sheets_export {
            rows.push(rows[rows.len() - 2].clone());
            rows.push(rows[rows.len() - 2].clone());
        }
    }

    for row in rows {
        let _ = writeln!(output, "{}", row.join(","));
    }
    output
    
}
