use bigdecimal::BigDecimal;
use diesel::prelude::*;
use time::Date;
use uuid::Uuid;

use crate::views::v_chantier_finition;

#[derive(Debug, Queryable, Identifiable, Selectable)]
#[diesel(table_name = v_chantier_finition)]
#[primary_key(id_chantier, client, id_finition)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct VChantierFinition {
    pub id_chantier: Uuid,
    pub client: String,
    pub date_debut: Option<Date>,
    pub finition_prix: BigDecimal,
    pub finition_duree: BigDecimal,
    pub finition: String,
    pub id_finition: Uuid,
}