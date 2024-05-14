use crate::{
    generate_upserts, graphql::objects::type_chantier::TypeChantierInput,
    models::type_chantier::TypeChantier,
};

use async_graphql::{Context, Object};
use diesel::prelude::*;

pub struct AdminTypeChantierMutations;

#[Object]
impl AdminTypeChantierMutations {
    generate_upserts!(
        TypeChantierInput,
        TypeChantier,
        type_chantier,
        id_type_chantier,
        crate::schema::type_chantier::dsl
    );
    pub async fn upsert(
        &self,
        ctx: &Context<'_>,
        input: TypeChantierInput,
    ) -> crate::Result<TypeChantier> {
        self.upsert_data(ctx, input).await
    }
    pub async fn upsert_batch(
        &self,
        ctx: &Context<'_>,
        input: Vec<TypeChantierInput>,
    ) -> crate::Result<Vec<TypeChantier>> {
        self.upsert_data_batch(ctx, input).await
    }
}
