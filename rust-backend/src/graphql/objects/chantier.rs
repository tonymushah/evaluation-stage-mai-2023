use async_graphql::InputObject;
use time::Date;
use uuid::Uuid;

use crate::models::chantier::Chantier;

#[derive(Clone, InputObject)]
pub struct ChantierInput {
    pub id_chantier: Uuid,
    pub client: String,
    pub type_chantier_id: Uuid,
    pub type_finition_id: Uuid,
    pub date_debut: Option<Date>,
}

impl From<ChantierInput> for Chantier {
    fn from(value: ChantierInput) -> Self {
        Self {
            id_chantier: value.id_chantier,
            client: value.client,
            type_chantier_id: value.type_chantier_id,
            type_finition_id: value.type_finition_id,
            date_debut: value.date_debut,
        }
    }
}
