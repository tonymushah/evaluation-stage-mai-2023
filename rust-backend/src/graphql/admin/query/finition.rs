use async_graphql::{Context, Object};
use uuid::Uuid;

use crate::{
    generate_pagination,
    graphql::{admin::objects::finition::AdminFinition, OffsetLimit, ResultsData},
    models::finition::Finition,
};

pub struct AdminFinitionQuery;

#[Object]
impl AdminFinitionQuery {
    generate_pagination!(
        Finition,
        AdminFinition,
        finition,
        id_finition,
        Uuid,
        crate::schema::finition::dsl
    );
    pub async fn list(
        &self,
        ctx: &Context<'_>,
        #[graphql(default)] input: OffsetLimit,
    ) -> crate::Result<ResultsData<AdminFinition>> {
        self.get_list(ctx, input).await
    }
    pub async fn unique(&self, ctx: &Context<'_>, id: Uuid) -> crate::Result<AdminFinition> {
        self.get_unique(ctx, id).await
    }
}
