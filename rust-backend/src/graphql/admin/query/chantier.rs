use async_graphql::{Context, Object};
use uuid::Uuid;

use crate::{
    generate_pagination,
    graphql::{admin::objects::chantier::AdminChantier, OffsetLimit, ResultsData},
    models::chantier::Chantier,
};

pub struct AdminChantierQuery;

#[Object]
impl AdminChantierQuery {
    generate_pagination!(
        Chantier,
        AdminChantier,
        chantier,
        id_chantier,
        Uuid,
        crate::schema::chantier::dsl
    );
    pub async fn list(
        &self,
        ctx: &Context<'_>,
        #[graphql(default)] input: OffsetLimit,
    ) -> crate::Result<ResultsData<AdminChantier>> {
        self.get_list(ctx, input).await
    }
    pub async fn unique(&self, ctx: &Context<'_>, id: Uuid) -> crate::Result<AdminChantier> {
        self.get_unique(ctx, id).await
    }
}
