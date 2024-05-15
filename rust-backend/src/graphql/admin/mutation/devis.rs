use crate::graphql::admin::objects::devis::AdminDevis;
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
    pub async fn upsert(&self, ctx: &Context<'_>, input: DevisInput) -> crate::Result<AdminDevis> {
        self.upsert_data(ctx, input).await.map(|i| i.into())
    }
    pub async fn upsert_batch(
        &self,
        ctx: &Context<'_>,
        input: Vec<DevisInput>,
    ) -> crate::Result<Vec<AdminDevis>> {
        self.upsert_data_batch(ctx, input)
            .await
            .map(|v| v.into_iter().map(|i| i.into()).collect())
    }
}
