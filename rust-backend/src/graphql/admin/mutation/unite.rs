use crate::{
    generate_upserts,
    graphql::{admin::objects::unite::AdminUnite, objects::unite::UniteInput},
    models::unite::Unite,
};

use async_graphql::{Context, Object};
use diesel::prelude::*;

pub struct AdminUniteMutations;

#[Object]
impl AdminUniteMutations {
    generate_upserts!(
        UniteInput,
        Unite,
        unite,
        id_unite,
        crate::schema::unite::dsl
    );
    pub async fn upsert(&self, ctx: &Context<'_>, input: UniteInput) -> crate::Result<AdminUnite> {
        self.upsert_data(ctx, input).await.map(|i| i.into())
    }
    pub async fn upsert_batch(
        &self,
        ctx: &Context<'_>,
        input: Vec<UniteInput>,
    ) -> crate::Result<Vec<AdminUnite>> {
        self.upsert_data_batch(ctx, input)
            .await
            .map(|v| v.into_iter().map(|i| i.into()).collect())
    }
}
