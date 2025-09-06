use crate::Tx;
use souvenir::{id, Id, Identifiable, Tagged};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Identifiable, Tagged)]
#[souvenir(tag = "slot")]
pub struct Slot {
    #[souvenir(id)]
    pub id: Id,
    pub w2m_id: i32,
}

impl Slot {
    pub fn new(id: Id, w2m_id: i32) -> Self {
        Self { id, w2m_id }
    }

    pub async fn all_slots(tx: impl Tx<'_>) -> anyhow::Result<Vec<Self>> {
        Ok(sqlx::query!(
            r#"
                SELECT id as "id: Id", w2m_id FROM slot 
                    ORDER BY w2m_id;
            "#
        )
        .fetch_all(&mut *tx.acquire().await?)
        .await?
        .into_iter()
        .map(|record| Self {
            id: record.id,
            w2m_id: record.w2m_id.unwrap(),
        })
        .collect())
    }
}

#[derive(Copy, Clone, Debug, Identifiable, Tagged)]
#[souvenir(tag = "sch")]
pub struct Schedule {
    #[souvenir(id)]
    pub id: Id,
    pub parent: Option<Id>,
}

impl Schedule {
    pub fn new(parent: Option<Id>) -> Self {
        Self::from(id!(Schedule), parent)
    }

    pub fn from(id: Id, parent: Option<Id>) -> Self {
        Self { id, parent }
    }

    pub async fn resolve(id: Id, tx: impl Tx<'_>) -> anyhow::Result<Self> {
        Ok(sqlx::query_as!(
            Schedule,
            r#"
                SELECT id AS "id: Id", parent_id AS "parent: Id" FROM schedule 
                    WHERE id = $1;
            "#,
            id as Id
        )
        .fetch_one(&mut *tx.acquire().await?)
        .await?)
    }

    pub async fn fetch_current(tx: impl Tx<'_>) -> anyhow::Result<Self> {
        Ok(sqlx::query_as!(
            Schedule,
            r#"
                SELECT id AS "id: Id", parent_id AS "parent: Id" FROM schedule 
                    WHERE id = (SELECT schedule FROM parameters);
            "#
        )
        .fetch_one(&mut *tx.acquire().await?)
        .await?)
    }

    pub async fn upsert(&mut self, tx: impl Tx<'_>) -> anyhow::Result<()> {
        sqlx::query!(
            "
            INSERT INTO schedule (id, parent_id) VALUES ($1, $2)
                ON CONFLICT (id) DO UPDATE SET parent_id = $2;
            ",
            self.id as Id,
            self.parent as Option<Id>
        )
        .execute(&mut *tx.acquire().await?)
        .await?;

        Ok(())
    }

    pub async fn count(&self, subject: impl Identifiable, tx: impl Tx<'_>) -> anyhow::Result<u32> {
        Ok(sqlx::query!(
            "
                SELECT COUNT(*) AS count FROM schedule_assignment
                    WHERE schedule_id = $1 AND subject_id = $2;
            ",
            self.id as Id,
            subject.id() as Id,
        )
        .fetch_one(&mut *tx.acquire().await?)
        .await?
        .count
        .unwrap_or(0) as u32)
    }

    pub async fn count_total(
        &self,
        subject: impl Identifiable,
        tx: impl Tx<'_>,
    ) -> anyhow::Result<u32> {
        let mut conn = tx.acquire().await?;
        let subject = subject.id();

        let mut schedule = *self;
        let mut count = 0;

        loop {
            count += schedule.count(subject, &mut *conn).await?;

            if let Some(parent) = schedule.parent {
                schedule = Schedule::resolve(parent, &mut *conn).await?;
            } else {
                break;
            }
        }

        Ok(count)
    }

    pub async fn last_scheduled(
        &self,
        subject: impl Identifiable,
        tx: impl Tx<'_>,
    ) -> anyhow::Result<Option<u64>> {
        let mut conn = tx.acquire().await?;
        let subject = subject.id();

        let mut schedule = *self;
        let mut count = 0;

        loop {
            if schedule.count(subject, &mut *conn).await? > 0 {
                return Ok(Some(count));
            }

            if let Some(parent) = schedule.parent {
                schedule = Schedule::resolve(parent, &mut *conn).await?;
                count += 1;
            } else {
                return Ok(None);
            }
        }
    }

    pub async fn add(
        &self,
        slot: impl Identifiable,
        subject: impl Identifiable,
        tx: impl Tx<'_>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "
                INSERT INTO schedule_assignment (schedule_id, subject_id, slot_id)
                    VALUES ($1, $2, $3);
            ",
            self.id as Id,
            subject.id() as Id,
            slot.id() as Id,
        )
        .execute(&mut *tx.acquire().await?)
        .await?;

        Ok(())
    }

    pub async fn get_slot(
        &self,
        slot: impl Identifiable,
        tx: impl Tx<'_>,
    ) -> Result<Vec<Id>, sqlx::Error> {
        Ok(sqlx::query!(
            r#"
            SELECT subject_id AS "subject: Id" FROM schedule_assignment
                WHERE schedule_id = $1 AND slot_id = $2;
            "#,
            self.id as Id,
            slot.id() as Id,
        )
        .fetch_all(&mut *tx.acquire().await?)
        .await?
        .into_iter()
        .map(|record| record.subject)
        .collect())
    }
}
