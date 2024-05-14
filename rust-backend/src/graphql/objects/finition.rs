use async_graphql::InputObject;
use bigdecimal::BigDecimal;
use uuid::Uuid;

use crate::models::finition::Finition;

#[derive(Debug, Clone, InputObject)]
pub struct FinitionInput {
    pub id_finition: Option<Uuid>,
    pub designation: String,
    pub prix: BigDecimal,
    pub duree: BigDecimal,
    pub is_standard: Option<bool>,
}

impl From<FinitionInput> for Finition {
    fn from(value: FinitionInput) -> Self {
        Self {
            id_finition: value.id_finition.unwrap_or(Uuid::new_v4()),
            designation: value.designation,
            prix: value.prix,
            duree: value.duree,
            is_standard: value.is_standard,
        }
    }
}
