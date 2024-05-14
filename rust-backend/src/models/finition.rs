use crate::{schema::finition, DbPoolConnection};

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
#[diesel(primary_key(id_finition))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Finition {
    pub id_finition: Uuid,
    pub designation: String,
    pub prix: BigDecimal,
    pub duree: BigDecimal,
    pub is_standard: Option<bool>,
}

impl Finition {
    pub fn get_first_standard(con: &mut DbPoolConnection) -> crate::Result<Self> {
        use crate::schema::finition::dsl::*;
        finition
            .filter(is_standard.eq(true))
            .limit(1)
            .select(Self::as_select())
            .load(con)?
            .first()
            .cloned()
            .ok_or(crate::Error::StandardFinitionNotFound)
    }
}
