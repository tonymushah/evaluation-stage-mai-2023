use crate::schema::finition;

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
)]
#[diesel(table_name = finition)]
#[primary_key(id_finition)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Finition {
    pub id_finition: Uuid,
    pub designation: String,
    pub prix: BigDecimal,
    pub duree: BigDecimal,
    pub is_standard: Option<bool>,
}
