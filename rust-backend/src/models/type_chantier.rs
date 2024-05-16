use crate::{graphql::get_pool, models::v_devis_materiel::VDevisMateriel, schema::type_chantier};
use actix_web::web::block;
use async_graphql::{ComplexObject, Context, SimpleObject};
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
#[diesel(table_name = type_chantier)]
#[diesel(primary_key(id_type_chantier))]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[graphql(complex)]
pub struct TypeChantier {
    pub id_type_chantier: Uuid,
    pub nom: String,
    pub description: String,
    pub duree: BigDecimal,
}

#[ComplexObject]
impl TypeChantier {
    pub async fn prix_total(&self, ctx: &Context<'_>) -> crate::Result<BigDecimal> {
        let mut pool = get_pool(ctx)?;
        let id = self.id_type_chantier;
        block(move || -> crate::Result<BigDecimal> {
            use crate::views::v_devis_materiel::dsl::*;
            v_devis_materiel
                .filter(type_chantier.eq(id))
                .select(VDevisMateriel::as_select())
                .load(&mut pool)?
                .into_iter()
                .map(|v| v.prix_total)
                .reduce(|acc, i| acc + i)
                .ok_or(crate::Error::Diesel(diesel::result::Error::NotFound))
        })
        .await?
    }
}
