use souvenir::{Id, Identifiable, Tagged};

use crate::Tx;

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
