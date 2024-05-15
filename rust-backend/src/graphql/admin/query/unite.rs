use async_graphql::{Context, Object};
use uuid::Uuid;

use crate::{
    generate_pagination,
    graphql::{admin::objects::unite::AdminUnite, OffsetLimit, ResultsData},
    models::unite::Unite,
};

pub struct AdminUniteQuery;

#[Object]
impl AdminUniteQuery {
    generate_pagination!(
        Unite,
        AdminUnite,
        unite,
        id_unite,
        Uuid,
        crate::schema::unite::dsl
    );
    pub async fn list(
        &self,
        ctx: &Context<'_>,
        #[graphql(default)] input: OffsetLimit,
    ) -> crate::Result<ResultsData<AdminUnite>> {
        self.get_list(ctx, input).await
    }
    pub async fn unique(&self, ctx: &Context<'_>, id: Uuid) -> crate::Result<AdminUnite> {
        self.get_unique(ctx, id).await
    }
}
