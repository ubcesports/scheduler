use souvenir::{Id, Identifiable, Tagged};
use sqlx::PgConnection;

#[derive(Debug, Hash, PartialEq, Eq, Identifiable, Tagged)]
#[souvenir(tag = "sub")]
pub struct Subject {
    #[souvenir(id)]
    pub id: Id,
    pub tag: String,
    pub name: Option<String>,
}

impl Subject {
    pub async fn find(id: Id, tx: &mut PgConnection) -> Result<Self, sqlx::Error> {
        let data = sqlx::query!("SELECT * FROM subject WHERE id = $1 LIMIT 1;", id as Id)
            .fetch_one(tx)
            .await?;

        Ok(Subject {
            id,
            tag: data.tag,
            name: data.name,
        })
    }

    pub async fn upsert(
        id: Id,
        tag: &str,
        name: Option<&str>,
        tx: &mut PgConnection,
    ) -> Result<Self, sqlx::Error> {
        let current_name = sqlx::query!("SELECT name FROM subject WHERE id = $1;", id as Id)
            .fetch_optional(&mut *tx)
            .await?
            .map(|row| row.name)
            .flatten();

        let name = name.or(current_name.as_deref());

        sqlx::query!(
            r#"
                INSERT INTO subject (id, tag, name)
                    VALUES ($1, $2, $3)
                    ON CONFLICT (tag) DO UPDATE SET tag = $2
                    RETURNING id AS "id: Id", tag, name;
            "#,
            id as Id,
            tag,
            name,
        )
        .fetch_one(&mut *tx)
        .await
        .map(|result| Self {
            id: result.id,
            tag: result.tag,
            name: result.name,
        })
    }

    pub async fn all_subjects(tx: &mut PgConnection) -> Result<Vec<Self>, sqlx::Error> {
        Ok(
            sqlx::query!(r#"SELECT id AS "id: Id", tag, name FROM subject;"#)
                .fetch_all(tx)
                .await?
                .into_iter()
                .map(|record| Subject {
                    id: record.id,
                    tag: record.tag,
                    name: record.name,
                })
                .collect(),
        )
    }
}
