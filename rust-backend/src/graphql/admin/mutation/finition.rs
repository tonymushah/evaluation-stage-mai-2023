use crate::{
    generate_upserts, graphql::objects::finition::FinitionInput, models::finition::Finition,
};

use async_graphql::{Context, Object};
use diesel::prelude::*;

pub struct AdminFinitionMutations;

#[Object]
impl AdminFinitionMutations {
    generate_upserts!(
        FinitionInput,
        Finition,
        finition,
        id_finition,
        crate::schema::finition::dsl
    );
    pub async fn upsert(&self, ctx: &Context<'_>, input: FinitionInput) -> crate::Result<Finition> {
        self.upsert_data(ctx, input).await
    }
    pub async fn upsert_batch(
        &self,
        ctx: &Context<'_>,
        input: Vec<FinitionInput>,
    ) -> crate::Result<Vec<Finition>> {
        self.upsert_data_batch(ctx, input).await
    }
}
