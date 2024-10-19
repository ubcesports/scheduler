use crate::Subject;
use futures::future::BoxFuture;
use futures::FutureExt;
use souvenir::{Id, Identifiable, Type};
use sqlx::{Executor, Sqlite, SqliteConnection};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Slot {
    pub id: Id<Slot>,
    pub w2m_id: i64,
}

impl Slot {
    pub fn new(id: Id<Slot>, w2m_id: i64) -> Self {
        Self { id, w2m_id }
    }

    pub fn from_sql_row(id: String, w2m_id: i64) -> Self {
        Self {
            id: id.parse().unwrap(),
            w2m_id,
        }
    }

    pub fn to_sql_row(&self) -> (String, i64) {
        (self.id.to_string(), self.w2m_id)
    }

    pub async fn all_slots(
        tx: impl Executor<'_, Database = Sqlite>,
    ) -> Result<Vec<Self>, sqlx::Error> {
        Ok(sqlx::query!("SELECT * FROM slot ORDER BY w2m_id;")
            .fetch_all(tx)
            .await?
            .into_iter()
            .map(|record| Self {
                id: record.id.parse().unwrap(),
                w2m_id: record.w2m_id.unwrap(),
            })
            .collect())
    }
}

impl Type for Slot {
    const PREFIX: &'static str = "sl";
}

impl Identifiable for Slot {
    type Output = Slot;

    fn id(&self) -> Id<Self> {
        self.id
    }
}

#[derive(Debug)]
pub struct Schedule {
    pub id: Id<Schedule>,
    pub parent: Option<Id<Schedule>>,
}

impl Schedule {
    pub fn new(parent: Option<Id<Schedule>>) -> Self {
        Self::from(Id::random(), parent)
    }

    pub fn from(id: Id<Schedule>, parent: Option<Id<Schedule>>) -> Self {
        Self { id, parent }
    }

    pub async fn resolve(id: Id<Schedule>, tx: &mut SqliteConnection) -> Result<Self, sqlx::Error> {
        let data = sqlx::query!("SELECT * FROM schedule WHERE id = $1;", id)
            .fetch_one(tx)
            .await?;

        Ok(Self::from(
            Id::parse(&data.id).unwrap(),
            data.parent_id.map(|id| Id::parse(&id).unwrap()),
        ))
    }

    pub async fn fetch_current(tx: &mut SqliteConnection) -> Result<Self, sqlx::Error> {
        let data =
            sqlx::query!("SELECT * FROM schedule WHERE id = (SELECT schedule FROM parameters);")
                .fetch_one(tx)
                .await?;

        Ok(Self::from(
            Id::parse(&data.id).unwrap(),
            data.parent_id.map(|id| Id::parse(&id).unwrap()),
        ))
    }

    pub async fn upsert(&mut self, tx: &mut SqliteConnection) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "
            INSERT INTO schedule (id, parent_id) VALUES ($1, $2)
                ON CONFLICT DO UPDATE SET parent_id = $2;
            ",
            self.id,
            self.parent
        )
        .execute(tx)
        .await?;

        Ok(())
    }

    pub async fn count(
        &self,
        subject: impl Into<Id<Subject>>,
        tx: &mut SqliteConnection,
    ) -> Result<u64, sqlx::Error> {
        let subject_id = subject.into();

        Ok(sqlx::query!(
            "
            SELECT COUNT(*) AS count FROM schedule_assignment
                WHERE schedule_id = $1 AND subject_id = $2;
            ",
            self.id,
            subject_id,
        )
        .fetch_one(tx)
        .await?
        .count as u64)
    }

    pub async fn count_total(
        &self,
        subject: impl Into<Id<Subject>>,
        tx: &mut SqliteConnection,
    ) -> Result<u64, sqlx::Error> {
        fn count_total<'a>(
            this: &'a Schedule,
            subject: Id<Subject>,
            tx: &'a mut SqliteConnection,
        ) -> BoxFuture<'a, Result<u64, sqlx::Error>> {
            async move {
                let parent_count = if let Some(parent) = this.parent {
                    Schedule::resolve(parent, tx)
                        .await?
                        .count_total(subject, tx)
                        .await?
                } else {
                    0
                };

                Ok(parent_count + this.count(subject, tx).await?)
            }
            .boxed()
        }

        count_total(self, subject.into(), tx).await
    }

    pub async fn last_scheduled(
        &self,
        subject: impl Into<Id<Subject>>,
        tx: &mut SqliteConnection,
    ) -> Result<Option<u64>, sqlx::Error> {
        fn last_scheduled<'a>(
            this: &'a Schedule,
            subject: Id<Subject>,
            tx: &'a mut SqliteConnection,
        ) -> BoxFuture<'a, Result<Option<u64>, sqlx::Error>> {
            async move {
                if this.count(subject, tx).await? > 0 {
                    return Ok(Some(0));
                }

                if let Some(parent) = this.parent {
                    Ok(Schedule::resolve(parent, tx)
                        .await?
                        .last_scheduled(subject, tx)
                        .await?
                        .map(|c| c + 1))
                } else {
                    Ok(None)
                }
            }
            .boxed()
        }

        last_scheduled(self, subject.into(), tx).await
    }

    pub async fn add(
        &self,
        slot: impl Into<Id<Slot>>,
        subject: impl Into<Id<Subject>>,
        tx: impl Executor<'_, Database = Sqlite>,
    ) -> Result<(), sqlx::Error> {
        let slot_id = slot.into();
        let subject_id = subject.into();

        sqlx::query!(
            "
            INSERT INTO schedule_assignment (schedule_id, subject_id, slot_id)
                VALUES ($1, $2, $3);
            ",
            self.id,
            subject_id,
            slot_id,
        )
        .execute(tx)
        .await?;

        Ok(())
    }

    pub async fn get_slot(
        &self,
        slot: impl Into<Id<Slot>>,
        tx: impl Executor<'_, Database = Sqlite>,
    ) -> Result<Vec<Id<Subject>>, sqlx::Error> {
        let slot_id = slot.into();

        Ok(sqlx::query!(
            "
            SELECT subject_id FROM schedule_assignment
                WHERE schedule_id = $1 AND slot_id = $2;
            ",
            self.id,
            slot_id,
        )
        .fetch_all(tx)
        .await?
        .into_iter()
        .map(|record| Id::parse(&record.subject_id).unwrap())
        .collect())
    }
}

impl Type for Schedule {
    const PREFIX: &'static str = "sc";
}

impl Identifiable for Schedule {
    type Output = Schedule;

    fn id(&self) -> Id<Self> {
        self.id
    }
}
