use crate::{
    generate_upserts,
    graphql::{admin::objects::materiels::AdminMateriel, objects::materiel::MaterielInput},
    models::materiel::Materiel,
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
    pub async fn upsert(
        &self,
        ctx: &Context<'_>,
        input: MaterielInput,
    ) -> crate::Result<AdminMateriel> {
        self.upsert_data(ctx, input).await.map(|i| i.into())
    }
    pub async fn upsert_batch(
        &self,
        ctx: &Context<'_>,
        input: Vec<MaterielInput>,
    ) -> crate::Result<Vec<AdminMateriel>> {
        self.upsert_data_batch(ctx, input)
            .await
            .map(|v| v.into_iter().map(|i| i.into()).collect())
    }
}
