use crate::schema::chantier;

use async_graphql::SimpleObject;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use time::Date;
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
#[diesel(table_name = chantier)]
#[diesel(primary_key(id_chantier))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Chantier {
    pub id_chantier: Uuid,
    #[graphql(skip)]
    pub client: String,
    #[graphql(skip)]
    pub type_chantier_id: Uuid,
    #[graphql(skip)]
    pub type_finition_id: Uuid,
    pub date_debut: Option<Date>,
}
