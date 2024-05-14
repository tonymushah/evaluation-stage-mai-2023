use crate::{
    generate_upserts, graphql::objects::chantier::ChantierInput, models::chantier::Chantier,
};

use async_graphql::{Context, Object};
use diesel::prelude::*;

pub struct AdminChantierMutations;

impl AdminChantierMutations {
    generate_upserts!(
        ChantierInput,
        Chantier,
        chantier,
        id_chantier,
        crate::schema::chantier::dsl
    );
}

#[Object]
impl AdminChantierMutations {
    pub async fn upsert(&self, ctx: &Context<'_>, input: ChantierInput) -> crate::Result<Chantier> {
        self.upsert_data(ctx, input).await
    }
    pub async fn upsert_batch(
        &self,
        ctx: &Context<'_>,
        input: Vec<ChantierInput>,
    ) -> crate::Result<Vec<Chantier>> {
        self.upsert_data_batch(ctx, input).await
    }
}
