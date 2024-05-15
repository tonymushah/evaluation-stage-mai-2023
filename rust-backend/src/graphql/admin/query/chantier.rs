use async_graphql::{Context, Object};

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
        crate::schema::chantier::dsl
    );
    pub async fn list(
        &self,
        ctx: &Context<'_>,
        #[graphql(default)] input: OffsetLimit,
    ) -> crate::Result<ResultsData<AdminChantier>> {
        self.get_list(ctx, input).await
    }
}
