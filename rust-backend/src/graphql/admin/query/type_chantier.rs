use async_graphql::{Context, Object};
use uuid::Uuid;

use crate::{
    generate_pagination,
    graphql::{admin::objects::type_chantier::AdminTypeChantier, OffsetLimit, ResultsData},
    models::type_chantier::TypeChantier,
};

pub struct AdminTypeChantierQuery;

#[Object]
impl AdminTypeChantierQuery {
    generate_pagination!(
        TypeChantier,
        AdminTypeChantier,
        type_chantier,
        id_type_chantier,
        Uuid,
        crate::schema::type_chantier::dsl
    );
    pub async fn list(
        &self,
        ctx: &Context<'_>,
        #[graphql(default)] input: OffsetLimit,
    ) -> crate::Result<ResultsData<AdminTypeChantier>> {
        self.get_list(ctx, input).await
    }
    pub async fn unique(&self, ctx: &Context<'_>, id: Uuid) -> crate::Result<AdminTypeChantier> {
        self.get_unique(ctx, id).await
    }
}
