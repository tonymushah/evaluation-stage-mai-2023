use actix_web::web::block;
use async_graphql::{ComplexObject, Context};

use crate::{
    generate_admin_object,
    graphql::get_pool,
    models::{devis::Devis, materiel::Materiel, type_chantier::TypeChantier},
};

use super::{materiels::AdminMateriel, type_chantier::AdminTypeChantier};

generate_admin_object!(AdminDevis, Devis);

#[ComplexObject]
impl AdminDevis {
    pub async fn type_chantier(&self, ctx: &Context<'_>) -> crate::Result<AdminTypeChantier> {
        let mut pool = get_pool(ctx)?;
        let id = self.type_chantier_id;
        block(move || -> crate::Result<AdminTypeChantier> {
            use crate::schema::type_chantier::dsl::*;
            use diesel::prelude::*;
            type_chantier
                .filter(id_type_chantier.eq(id))
                .limit(1)
                .select(TypeChantier::as_select())
                .load(&mut pool)?
                .first()
                .cloned()
                .map(|i| i.into())
                .ok_or(crate::Error::Diesel(diesel::result::Error::NotFound))
        })
        .await?
    }
    pub async fn materiel(&self, ctx: &Context<'_>) -> crate::Result<AdminMateriel> {
        let mut pool = get_pool(ctx)?;
        let id = self.materiel_id.clone();
        block(move || -> crate::Result<AdminMateriel> {
            use crate::schema::materiels::dsl::*;
            use diesel::prelude::*;
            materiels
                .filter(code.eq(id))
                .limit(1)
                .select(Materiel::as_select())
                .load(&mut pool)?
                .first()
                .cloned()
                .map(|i| i.into())
                .ok_or(crate::Error::Diesel(diesel::result::Error::NotFound))
        })
        .await?
    }
}
