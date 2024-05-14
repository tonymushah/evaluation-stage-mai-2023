use crate::schema::type_chantier;
use async_graphql::SimpleObject;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    SimpleObject,
    Identifiable,
    Selectable,
    Insertable,
    Deserialize,
    Serialize,
    Queryable,
)]
#[diesel(table_name = type_chantier)]
#[primary_key(id_type_chantier)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TypeChantier {
    pub id_type_chantier: Uuid,
    pub nom: String,
    pub description: String,
}
