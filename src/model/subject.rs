use souvenir::{Id, Identifiable, Tagged};

use crate::Tx;

#[derive(Debug, Hash, PartialEq, Eq, Identifiable, Tagged)]
#[souvenir(tag = "sub")]
pub struct Subject {
    #[souvenir(id)]
    pub id: Id,
    pub w2m_id: Option<i32>,
    pub name: String,
}

impl Subject {
    pub async fn find(id: Id, tx: impl Tx<'_>) -> Result<Self, sqlx::Error> {
        let data = sqlx::query!("SELECT * FROM subject WHERE id = $1 LIMIT 1;", id as Id)
            .fetch_one(&mut *tx.acquire().await?)
            .await?;

        Ok(Subject {
            id,
            w2m_id: data.w2m_id,
            name: data.name,
        })
    }

    pub async fn upsert(
        id: Id,
        w2m_id: Option<i32>,
        name: impl Into<String>,
        tx: impl Tx<'_>,
    ) -> Result<Self, sqlx::Error> {
        let name_str = name.into();

        sqlx::query!(
            r#"
                INSERT INTO subject (id, w2m_id, name)
                    VALUES ($1, $2, $3)
                    ON CONFLICT (w2m_id) DO UPDATE SET w2m_id = $2
                    RETURNING id AS "id: Id", w2m_id, name;
            "#,
            id as Id,
            w2m_id,
            name_str,
        )
        .fetch_one(&mut *tx.acquire().await?)
        .await
        .map(|result| Self {
            id: result.id,
            w2m_id: result.w2m_id,
            name: result.name,
        })
    }

    pub async fn all_subjects(tx: impl Tx<'_>) -> Result<Vec<Self>, sqlx::Error> {
        Ok(
            sqlx::query!(r#"SELECT id AS "id: Id", w2m_id, name FROM subject;"#)
                .fetch_all(&mut *tx.acquire().await?)
                .await?
                .into_iter()
                .map(|record| Subject {
                    id: record.id,
                    w2m_id: record.w2m_id,
                    name: record.name,
                })
                .collect(),
        )
    }
}
