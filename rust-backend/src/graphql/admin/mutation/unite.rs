use crate::{generate_upserts, graphql::objects::unite::UniteInput, models::unite::Unite};

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
    pub async fn upsert(&self, ctx: &Context<'_>, input: UniteInput) -> crate::Result<Unite> {
        self.upsert_data(ctx, input).await
    }
    pub async fn upsert_batch(
        &self,
        ctx: &Context<'_>,
        input: Vec<UniteInput>,
    ) -> crate::Result<Vec<Unite>> {
        self.upsert_data_batch(ctx, input).await
    }
}
