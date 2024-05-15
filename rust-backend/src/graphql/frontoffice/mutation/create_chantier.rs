use async_graphql::InputObject;
use time::Date;
use uuid::Uuid;

use crate::graphql::objects::chantier::ChantierInput;

#[derive(Debug, Clone, Copy, InputObject)]
pub struct ClientCreateChantier {
    pub type_chantier: Uuid,
    pub date_debut: Option<Date>,
    pub type_finition_id: Option<Uuid>,
}

impl ClientCreateChantier {
    pub fn into_chantier_input(self, finition: Uuid, client: String) -> ChantierInput {
        ChantierInput {
            id: None,
            client,
            type_chantier_id: self.type_chantier,
            type_finition_id: finition,
            date_debut: self.date_debut,
        }
    }
}
