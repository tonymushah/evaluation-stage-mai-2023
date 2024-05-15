use async_graphql::{Context, Object};

use crate::{
    generate_pagination,
    graphql::{admin::objects::clients::AdminClient, OffsetLimit, ResultsData},
    models::client::Client,
};

pub struct AdminClientQuery;

#[Object]
impl AdminClientQuery {
    generate_pagination!(
        Client,
        AdminClient,
        clients,
        telephone,
        String,
        crate::schema::clients::dsl
    );
    pub async fn list(
        &self,
        ctx: &Context<'_>,
        #[graphql(default)] input: OffsetLimit,
    ) -> crate::Result<ResultsData<AdminClient>> {
        self.get_list(ctx, input).await
    }
    pub async fn unique(&self, ctx: &Context<'_>, telephone: String) -> crate::Result<AdminClient> {
        self.get_unique(ctx, telephone).await
    }
}
