use async_graphql::InputObject;
use bigdecimal::BigDecimal;
use uuid::Uuid;

use crate::models::materiel::Materiel;

#[derive(Debug, InputObject)]
pub struct MaterielInput {
    pub code: String,
    pub designation: String,
    pub prix_unitaire: BigDecimal,
    pub unite_id: Uuid,
}

impl From<MaterielInput> for Materiel {
    fn from(value: MaterielInput) -> Self {
        Self {
            code: value.code,
            designation: value.designation,
            prix_unitaire: value.prix_unitaire,
            unite_id: value.unite_id,
        }
    }
}
