use crate::{
    generate_upserts,
    graphql::{admin::objects::chantier::AdminChantier, objects::chantier::ChantierInput},
    models::chantier::Chantier,
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
    pub async fn upsert(
        &self,
        ctx: &Context<'_>,
        input: ChantierInput,
    ) -> crate::Result<AdminChantier> {
        Ok(self.upsert_data(ctx, input).await?.into())
    }
    pub async fn upsert_batch(
        &self,
        ctx: &Context<'_>,
        input: Vec<ChantierInput>,
    ) -> crate::Result<Vec<AdminChantier>> {
        Ok(self
            .upsert_data_batch(ctx, input)
            .await?
            .into_iter()
            .map(|i| i.into())
            .collect())
    }
}
