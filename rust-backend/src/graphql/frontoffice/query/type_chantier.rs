use async_graphql::{Context, Object};
use diesel::prelude::*;

use actix_web::web::block;

use crate::{
    graphql::{OffsetLimit, ResultsData},
    models::{type_chantier::TypeChantier, Paginate},
    DbPool,
};

#[derive(Debug, Clone, Default)]
pub struct ClientTypeChantierQueries;

#[Object]
impl ClientTypeChantierQueries {
    pub async fn list(
        &self,
        ctx: &Context<'_>,
        #[graphql(default)] page: OffsetLimit,
    ) -> crate::Result<ResultsData<TypeChantier>> {
        let mut pool = ctx.data::<DbPool>().map_err(crate::Error::GraphQL)?.get()?;
        block(move || {
            use crate::schema::type_chantier::dsl::*;
            let (data, total) = type_chantier
                .select(TypeChantier::as_select())
                .paginate_with_param(page)
                .load_and_count_pages::<TypeChantier>(&mut pool)?;
            Ok(ResultsData {
                data,
                limit: page.limit,
                offset: page.offset,
                total,
            })
        })
        .await?
    }
}
