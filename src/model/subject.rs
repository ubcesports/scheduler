use souvenir::{Id, Identifiable};
use sqlx::{Executor, Sqlite};

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Subject {
    pub id: Id<Subject>,
    pub w2m_id: i64,
    pub name: String,
}

impl Subject {
    pub fn new(id: Id<Subject>, w2m_id: i64, name: impl Into<String>) -> Self {
        Self {
            id,
            w2m_id,
            name: name.into(),
        }
    }

    pub fn id(&self) -> Id<Subject> {
        self.id
    }

    pub async fn upsert(
        &mut self,
        tx: impl Executor<'_, Database = Sqlite>,
    ) -> Result<(), sqlx::Error> {
        let id = self.id.as_i64();

        let result = sqlx::query!(
            "
            INSERT INTO subject (id, w2m_id, name)
                VALUES ($1, $2, $3)
                ON CONFLICT (w2m_id) DO UPDATE SET w2m_id = w2m_id
                RETURNING id, w2m_id, name;
            ",
            id,
            self.w2m_id,
            self.name,
        )
        .fetch_one(tx)
        .await?;

        self.id = result.id.into();
        result.w2m_id.map(|id| self.w2m_id = id);
        self.name = result.name;

        Ok(())
    }

    pub async fn all_subjects(
        tx: impl Executor<'_, Database = Sqlite>,
    ) -> Result<Vec<Subject>, sqlx::Error> {
        Ok(sqlx::query!("SELECT * FROM subject;")
            .fetch_all(tx)
            .await?
            .into_iter()
            .map(|record| Subject::new(record.id.into(), record.w2m_id.unwrap(), record.name))
            .collect())
    }

    pub async fn resolve(
        id: Id<Subject>,
        tx: impl Executor<'_, Database = Sqlite>,
    ) -> Result<Subject, sqlx::Error> {
        let qid = id.as_i64();

        let data = sqlx::query!("SELECT * FROM subject WHERE id = $1 LIMIT 1;", qid)
            .fetch_one(tx)
            .await?;

        Ok(Subject::new(id, data.w2m_id.unwrap(), data.name))
    }
}

impl Identifiable for Subject {
    const PREFIX: &'static str = "sj";
}
