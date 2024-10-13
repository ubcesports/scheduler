use std::collections::HashMap;

use crate::{Slot, Subject};
use souvenir::{Id, Identifiable, Type};
use sqlx::{Executor, Sqlite, SqliteConnection};

#[derive(Debug)]
pub struct Availability {
    pub id: Id<Availability>,
}

impl Availability {
    pub fn new(id: Id<Availability>) -> Self {
        Self { id }
    }

    pub async fn upsert(
        &mut self,
        tx: impl Executor<'_, Database = Sqlite>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO availability (id) VALUES ($1) ON CONFLICT DO NOTHING;",
            self.id,
        )
        .execute(tx)
        .await?;

        Ok(())
    }

    pub async fn fetch_current(
        tx: impl Executor<'_, Database = Sqlite>,
    ) -> Result<Self, sqlx::Error> {
        let data = sqlx::query!(
            "SELECT * FROM availability WHERE id = (SELECT availability FROM parameters);"
        )
        .fetch_one(tx)
        .await?;

        Ok(Self::new(Id::from(data.id)))
    }

    pub async fn insert(
        &self,
        slot: impl Into<Id<Slot>>,
        subject: impl Into<Id<Subject>>,
        tx: impl Executor<'_, Database = Sqlite>,
    ) -> Result<(), sqlx::Error> {
        let slot_id = slot.into();
        let subject_id = subject.into();

        sqlx::query!(
            "
            INSERT INTO availability_entry (availability_id, slot_id, subject_id)
                VALUES ($1, $2, $3)
            ",
            self.id,
            slot_id,
            subject_id,
        )
        .execute(tx)
        .await?;

        Ok(())
    }

    pub async fn for_slot(
        &self,
        slot: impl Into<Id<Slot>>,
        tx: impl Executor<'_, Database = Sqlite>,
    ) -> Result<Vec<Id<Subject>>, sqlx::Error> {
        let slot_id = slot.into();

        Ok(sqlx::query!(
            "
            SELECT subject_id FROM availability_entry
                WHERE availability_id = $1 AND slot_id = $2;
            ",
            self.id,
            slot_id,
        )
        .fetch_all(tx)
        .await?
        .into_iter()
        .map(|record| Id::from(record.subject_id))
        .collect())
    }

    pub async fn for_subject(
        &self,
        subject: impl Into<Id<Subject>>,
        tx: &mut SqliteConnection,
    ) -> Result<Vec<Id<Slot>>, sqlx::Error> {
        let subject_id = subject.into();

        Ok(sqlx::query!(
            "
            SELECT slot_id FROM availability_entry
                WHERE availability_id = $1 AND subject_id = $2;
            ",
            self.id,
            subject_id,
        )
        .fetch_all(tx)
        .await?
        .into_iter()
        .map(|record| Id::from(record.slot_id))
        .collect())
    }

    pub async fn sorted_by_flexibility(
        &self,
        tx: impl Executor<'_, Database = Sqlite>,
    ) -> Result<Vec<(Id<Slot>, Vec<Id<Subject>>)>, sqlx::Error> {
        let mut map: HashMap<_, Vec<_>> = HashMap::new();

        sqlx::query!(
            "SELECT slot_id, subject_id FROM availability_entry WHERE availability_id = $1;",
            self.id
        )
        .fetch_all(tx)
        .await?
        .into_iter()
        .map(|record| {
            (
                Id::<Slot>::from(record.slot_id),
                Id::<Subject>::from(record.subject_id),
            )
        })
        .for_each(|(slot, subject)| map.entry(slot).or_default().push(subject));

        let mut list = Vec::from_iter(map.into_iter());
        list.sort_by(|a, b| a.1.len().cmp(&b.1.len()));
        Ok(list)
    }
}

impl Type for Availability {
    const PREFIX: &'static str = "av";
}

impl Identifiable for Availability {
    fn id(&self) -> Id<Self> {
        self.id
    }
}
