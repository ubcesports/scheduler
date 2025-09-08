use souvenir::{Id, Identifiable, Tagged};

use sqlx::PgConnection;

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

    pub async fn all_slots(tx: &mut PgConnection) -> anyhow::Result<Vec<Self>> {
        Ok(sqlx::query!(
            r#"
                SELECT id as "id: Id", w2m_id FROM slot 
                    ORDER BY w2m_id;
            "#
        )
        .fetch_all(tx)
        .await?
        .into_iter()
        .map(|record| {
            let w2m_id = record.w2m_id.ok_or_else(|| anyhow::anyhow!("w2m_id is NULL for slot id {}", record.id))?;
            Ok(Self {
                id: record.id,
                w2m_id,
            })
        })
        .collect::<Result<Vec<_>, _>>())
    }
}
