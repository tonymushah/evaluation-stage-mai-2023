use async_graphql::{Context, Object};

use crate::{
    generate_pagination,
    graphql::{admin::objects::materiels::AdminMateriel, OffsetLimit, ResultsData},
    models::materiel::Materiel,
};

pub struct AdminMaterielQuery;

#[Object]
impl AdminMaterielQuery {
    generate_pagination!(
        Materiel,
        AdminMateriel,
        materiels,
        code,
        String,
        crate::schema::materiels::dsl
    );
    pub async fn list(
        &self,
        ctx: &Context<'_>,
        #[graphql(default)] input: OffsetLimit,
    ) -> crate::Result<ResultsData<AdminMateriel>> {
        self.get_list(ctx, input).await
    }
    pub async fn unique(&self, ctx: &Context<'_>, code: String) -> crate::Result<AdminMateriel> {
        self.get_unique(ctx, code).await
    }
}
