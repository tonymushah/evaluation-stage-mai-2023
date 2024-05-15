use actix_web::web::block;
use async_graphql::{ComplexObject, Context};

use crate::{
    generate_admin_object,
    graphql::get_pool,
    models::{chantier::Chantier, finition::Finition, type_chantier::TypeChantier},
};

use super::{finition::AdminFinition, type_chantier::AdminTypeChantier};

generate_admin_object!(AdminChantier, Chantier);

#[ComplexObject]
impl AdminChantier {
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
    pub async fn finition(&self, ctx: &Context<'_>) -> crate::Result<AdminFinition> {
        let mut pool = get_pool(ctx)?;
        let id = self.type_finition_id;
        block(move || -> crate::Result<AdminFinition> {
            use crate::schema::finition::dsl::*;
            use diesel::prelude::*;
            finition
                .filter(id_finition.eq(id))
                .limit(1)
                .select(Finition::as_select())
                .load(&mut pool)?
                .first()
                .cloned()
                .map(|i| i.into())
                .ok_or(crate::Error::Diesel(diesel::result::Error::NotFound))
        })
        .await?
    }
}
