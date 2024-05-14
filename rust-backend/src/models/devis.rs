use crate::schema::devis;

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
    AsChangeset,
)]
#[diesel(table_name = devis)]
#[diesel(primary_key(id_devis))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Devis {
    pub id_devis: Uuid,
    pub type_chantier_id: Uuid,
    #[graphql(skip)]
    pub materiel_id: String,
    pub quantite: BigDecimal,
}
