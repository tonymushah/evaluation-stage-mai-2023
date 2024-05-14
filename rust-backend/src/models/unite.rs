use crate::schema::unite;
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
    AsChangeset,
)]
#[diesel(table_name = unite)]
#[diesel(primary_key(id_unite))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Unite {
    pub id_unite: Uuid,
    pub designation: String,
}
