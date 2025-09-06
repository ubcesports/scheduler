use std::collections::HashMap;

use souvenir::{Id, Identifiable, Tagged};
use sqlx::{Executor, Postgres};

use crate::Tx;

#[derive(Debug, Identifiable, Tagged)]
#[souvenir(tag = "av")]
pub struct Availability {
    #[souvenir(id)]
    pub id: Id,
}

impl Availability {
    pub fn new(id: Id) -> Self {
        Self { id }
    }

    pub async fn upsert(&mut self, tx: impl Tx<'_>) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
                INSERT INTO availability (id)
                    VALUES ($1)
                    ON CONFLICT DO NOTHING;
            "#,
            self.id as Id,
        )
        .execute(&mut *tx.acquire().await?)
        .await?;

        Ok(())
    }

    pub async fn fetch_current(tx: impl Executor<'_, Database = Postgres>) -> anyhow::Result<Self> {
        Ok(sqlx::query_as!(
            Availability,
            r#"
                SELECT id as "id: _" FROM availability 
                    WHERE id = (SELECT availability FROM parameters);
            "#
        )
        .fetch_one(tx)
        .await?)
    }

    pub async fn insert(
        &self,
        slot: impl Identifiable,
        subject: impl Identifiable,
        tx: impl Tx<'_>,
    ) -> anyhow::Result<()> {
        sqlx::query!(
            "
                INSERT INTO availability_entry (availability_id, slot_id, subject_id)
                    VALUES ($1, $2, $3)
            ",
            self.id as Id,
            slot.id() as Id,
            subject.id() as Id,
        )
        .execute(&mut *tx.acquire().await?)
        .await?;

        Ok(())
    }

    pub async fn for_slot(
        &self,
        slot: impl Identifiable,
        tx: impl Tx<'_>,
    ) -> anyhow::Result<Vec<Id>> {
        Ok(sqlx::query!(
            r#"
                SELECT subject_id as "id: Id" FROM availability_entry
                    WHERE availability_id = $1 AND slot_id = $2;
            "#,
            self.id as Id,
            slot.id() as Id,
        )
        .fetch_all(&mut *tx.acquire().await?)
        .await?
        .into_iter()
        .map(|record| record.id)
        .collect())
    }

    pub async fn for_subject(
        &self,
        subject: impl Identifiable,
        tx: impl Tx<'_>,
    ) -> anyhow::Result<Vec<Id>> {
        Ok(sqlx::query!(
            r#"
                SELECT slot_id as "id: Id" FROM availability_entry
                    WHERE availability_id = $1 AND subject_id = $2;
            "#,
            self.id as Id,
            subject.id() as Id,
        )
        .fetch_all(&mut *tx.acquire().await?)
        .await?
        .into_iter()
        .map(|record| record.id)
        .collect())
    }

    pub async fn sorted_by_flexibility(
        &self,
        tx: impl Tx<'_>,
    ) -> anyhow::Result<Vec<(Id, Vec<Id>)>> {
        let mut map: HashMap<_, Vec<_>> = HashMap::new();

        sqlx::query!(
            r#"
                SELECT slot_id AS "slot: Id", subject_id AS "subject: Id"
                    FROM availability_entry 
                    WHERE availability_id = $1;
            "#,
            self.id as Id
        )
        .fetch_all(&mut *tx.acquire().await?)
        .await?
        .into_iter()
        .map(|record| (record.slot, record.subject))
        .for_each(|(slot, subject)| map.entry(slot).or_default().push(subject));

        let mut list = Vec::from_iter(map.into_iter());
        list.sort_by(|a, b| a.1.len().cmp(&b.1.len()));

        Ok(list)
    }
}
