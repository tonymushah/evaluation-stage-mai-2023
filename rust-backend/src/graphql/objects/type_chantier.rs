use async_graphql::InputObject;
use bigdecimal::BigDecimal;
use uuid::Uuid;

use crate::models::type_chantier::TypeChantier;

#[derive(Debug, Clone, InputObject)]
pub struct TypeChantierInput {
    pub id: Option<Uuid>,
    pub nom: String,
    pub description: String,
    pub duree: BigDecimal,
}

impl From<TypeChantierInput> for TypeChantier {
    fn from(value: TypeChantierInput) -> Self {
        Self {
            id_type_chantier: value.id.unwrap_or(Uuid::new_v4()),
            nom: value.nom,
            description: value.description,
            duree: value.duree,
        }
    }
}
