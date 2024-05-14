use async_graphql::InputObject;
use bigdecimal::BigDecimal;
use uuid::Uuid;

use crate::models::devis::Devis;

#[derive(Debug, Clone, InputObject)]
pub struct DevisInput {
    pub id: Option<Uuid>,
    pub type_chantier_id: Uuid,
    pub materiel_id: String,
    pub quantite: BigDecimal,
}

impl From<DevisInput> for Devis {
    fn from(value: DevisInput) -> Self {
        Self {
            id_devis: value.id.unwrap_or(Uuid::new_v4()),
            type_chantier_id: value.type_chantier_id,
            materiel_id: value.materiel_id,
            quantite: value.quantite,
        }
    }
}
