use crate::{generate_upserts, graphql::objects::devis::DevisInput, models::devis::Devis};
use async_graphql::Context;

use async_graphql::Object;
use diesel::prelude::*;

pub struct AdminDevisMutations;

#[Object]
impl AdminDevisMutations {
    generate_upserts!(
        DevisInput,
        Devis,
        devis,
        id_devis,
        crate::schema::devis::dsl
    );
    pub async fn upsert(&self, ctx: &Context<'_>, input: DevisInput) -> crate::Result<Devis> {
        self.upsert_data(ctx, input).await
    }
    pub async fn upsert_batch(
        &self,
        ctx: &Context<'_>,
        input: Vec<DevisInput>,
    ) -> crate::Result<Vec<Devis>> {
        self.upsert_data_batch(ctx, input).await
    }
}
