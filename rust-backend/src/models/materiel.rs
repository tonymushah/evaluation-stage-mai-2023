use crate::schema::materiels;

use async_graphql::SimpleObject;
use bigdecimal::BigDecimal;
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
    Associations,
)]
#[diesel(table_name = materiels)]
#[primary_key(code)]
#[diesel(belongs_to(super::unite::Unite))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Materiel {
    pub code: String,
    pub designation: String,
    pub prix_unitaire: BigDecimal,
    #[graphql(skip)]
    pub unite_id: Uuid,
}
