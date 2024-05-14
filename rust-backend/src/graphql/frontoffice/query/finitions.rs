use actix_web::web::block;
use async_graphql::{Context, Object};
use diesel::prelude::*;

use crate::{
    graphql::{OffsetLimit, ResultsData},
    models::{finition::Finition, Paginate},
    DbPool,
};

#[derive(Debug, Default, Clone, Copy)]
pub struct FinitionQueries;

#[Object]
impl FinitionQueries {
    pub async fn get_default(&self, ctx: &Context<'_>) -> crate::Result<Finition> {
        let mut pool = ctx.data::<DbPool>().map_err(crate::Error::GraphQL)?.get()?;
        block(move || Finition::get_first_standard(&mut pool)).await?
    }
    pub async fn list(
        &self,
        ctx: &Context<'_>,
        #[graphql(default)] page: OffsetLimit,
    ) -> crate::Result<ResultsData<Finition>> {
        let mut pool = ctx.data::<DbPool>().map_err(crate::Error::GraphQL)?.get()?;
        block(move || {
            use crate::schema::finition::dsl::*;
            let (data, total) = finition
                .select(Finition::as_select())
                .paginate_with_param(page)
                .load_and_count_pages::<Finition>(&mut pool)?;
            Ok::<ResultsData<Finition>, crate::Error>(ResultsData {
                data,
                limit: page.limit,
                offset: page.offset,
                total,
            })
        })
        .await?
    }
}
