use bigdecimal::BigDecimal;
use diesel::prelude::*;
use uuid::Uuid;

use crate::views::v_devis_materiel;

#[derive(Debug, Queryable, Identifiable, Selectable)]
#[diesel(table_name = v_devis_materiel)]
#[diesel(primary_key(id_devis, unite_id, materiel_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct VDevisMateriel {
    pub id_devis: Uuid,
    pub type_chantier: Uuid,
    pub materiel_id: String,
    pub materiel: String,
    pub quantite: BigDecimal,
    pub prix_unitaire: BigDecimal,
    pub unite: String,
    pub unite_id: Uuid,
    pub prix_total: BigDecimal,
}
