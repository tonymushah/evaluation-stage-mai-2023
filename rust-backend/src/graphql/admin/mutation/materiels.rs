use crate::{
    generate_upserts, graphql::objects::materiel::MaterielInput, models::materiel::Materiel,
};

use async_graphql::{Context, Object};
use diesel::prelude::*;

pub struct AdminMaterielMutations;

#[Object]
impl AdminMaterielMutations {
    generate_upserts!(
        MaterielInput,
        Materiel,
        materiels,
        code,
        crate::schema::materiels::dsl
    );
    pub async fn upsert(&self, ctx: &Context<'_>, input: MaterielInput) -> crate::Result<Materiel> {
        self.upsert_data(ctx, input).await
    }
    pub async fn upsert_batch(
        &self,
        ctx: &Context<'_>,
        input: Vec<MaterielInput>,
    ) -> crate::Result<Vec<Materiel>> {
        self.upsert_data_batch(ctx, input).await
    }
}
