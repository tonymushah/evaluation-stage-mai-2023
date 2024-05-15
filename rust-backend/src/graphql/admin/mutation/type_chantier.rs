use crate::{
    generate_upserts,
    graphql::{
        admin::objects::type_chantier::AdminTypeChantier, objects::type_chantier::TypeChantierInput,
    },
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
    ) -> crate::Result<AdminTypeChantier> {
        self.upsert_data(ctx, input).await.map(|i| i.into())
    }
    pub async fn upsert_batch(
        &self,
        ctx: &Context<'_>,
        input: Vec<TypeChantierInput>,
    ) -> crate::Result<Vec<AdminTypeChantier>> {
        self.upsert_data_batch(ctx, input)
            .await
            .map(|v| v.into_iter().map(|i| i.into()).collect())
    }
}
