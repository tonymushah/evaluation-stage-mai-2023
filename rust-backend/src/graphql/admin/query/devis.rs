use async_graphql::{Context, Object};
use uuid::Uuid;

use crate::{
    generate_pagination,
    graphql::{admin::objects::devis::AdminDevis, OffsetLimit, ResultsData},
    models::devis::Devis,
};

pub struct AdminDevisQuery;

#[Object]
impl AdminDevisQuery {
    generate_pagination!(
        Devis,
        AdminDevis,
        devis,
        id_devis,
        Uuid,
        crate::schema::devis::dsl
    );
    pub async fn list(
        &self,
        ctx: &Context<'_>,
        #[graphql(default)] input: OffsetLimit,
    ) -> crate::Result<ResultsData<AdminDevis>> {
        self.get_list(ctx, input).await
    }
    pub async fn unique(&self, ctx: &Context<'_>, id: Uuid) -> crate::Result<AdminDevis> {
        self.get_unique(ctx, id).await
    }
}
