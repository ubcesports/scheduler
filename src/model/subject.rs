use souvenir::{Id, Identifiable, Type};
use sqlx::{Executor, Sqlite};

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Subject {
    pub id: Id<Subject>,
    pub w2m_id: Option<i64>,
    pub name: String,
}

impl Subject {
    pub async fn find(
        id: Id<Subject>,
        tx: impl Executor<'_, Database = Sqlite>,
    ) -> Result<Self, sqlx::Error> {
        let data = sqlx::query!("SELECT * FROM subject WHERE id = $1 LIMIT 1;", id)
            .fetch_one(tx)
            .await?;

        Ok(Subject {
            id,
            w2m_id: data.w2m_id,
            name: data.name,
        })
    }

    pub async fn upsert(
        id: Id<Subject>,
        w2m_id: Option<i64>,
        name: impl Into<String>,
        tx: impl Executor<'_, Database = Sqlite>,
    ) -> Result<Self, sqlx::Error> {
        let name_str = name.into();

        sqlx::query!(
            "
            INSERT INTO subject (id, w2m_id, name)
                VALUES ($1, $2, $3)
                ON CONFLICT (w2m_id) DO UPDATE SET w2m_id = w2m_id
                RETURNING id, w2m_id, name;
            ",
            id,
            w2m_id,
            name_str,
        )
        .fetch_one(tx)
        .await
        .map(|result| Self {
            id: result.id.parse().unwrap(),
            w2m_id: result.w2m_id,
            name: result.name,
        })
    }

    pub async fn all_subjects(
        tx: impl Executor<'_, Database = Sqlite>,
    ) -> Result<Vec<Self>, sqlx::Error> {
        Ok(sqlx::query!("SELECT * FROM subject;")
            .fetch_all(tx)
            .await?
            .into_iter()
            .map(|record| Subject {
                id: record.id.parse().unwrap(),
                w2m_id: record.w2m_id,
                name: record.name,
            })
            .collect())
    }
}

impl Type for Subject {
    const PREFIX: &'static str = "sj";
}

impl Identifiable for Subject {
    type Output = Subject;

    fn id(&self) -> Id<Self> {
        self.id
    }
}
