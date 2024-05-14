use async_graphql::SimpleObject;
use bigdecimal::BigDecimal;
use time::Date;
use uuid::Uuid;

use crate::models::v_chantier_finition::VChantierFinition;

#[derive(Debug, SimpleObject, Clone)]
pub struct ClientChantier {
    pub id: Uuid,
    pub date_debut: Option<Date>,
    #[graphql(skip)]
    pub finition_prix: BigDecimal,
    pub finition_duree: BigDecimal,
    pub finition: String,
    pub id_finition: Uuid,
    #[graphql(skip)]
    pub type_chantier_id: Uuid,
}

impl From<VChantierFinition> for ClientChantier {
    fn from(value: VChantierFinition) -> Self {
        Self {
            id: value.id_chantier,
            date_debut: value.date_debut,
            finition_prix: value.finition_prix,
            finition_duree: value.finition_duree,
            finition: value.finition,
            id_finition: value.id_finition,
            type_chantier_id: value.type_chantier_id,
        }
    }
}
