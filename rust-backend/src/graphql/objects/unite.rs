use async_graphql::InputObject;
use uuid::Uuid;

use crate::models::unite::Unite;

#[derive(Debug, Clone, InputObject)]
pub struct UniteInput {
    pub id: Option<Uuid>,
    pub designation: String,
}

impl From<UniteInput> for Unite {
    fn from(value: UniteInput) -> Self {
        Self {
            id_unite: value.id.unwrap_or(Uuid::new_v4()),
            designation: value.designation,
        }
    }
}
