pub mod finitions;

use actix_web::web::block;
use async_graphql::{Context, Object};
use diesel::prelude::*;

use crate::{
    graphql::{OffsetLimit, ResultsData},
    models::{v_chantier_finition::VChantierFinition, Paginate},
    DbPool,
};

use super::{models::chantier::ClientChantier, CurrentClient};

#[derive(Debug, Clone, Copy, Hash, Default)]
pub struct FrontOfficeQuery;

#[Object]
impl FrontOfficeQuery {
    pub async fn hello(&self) -> String {
        String::from("Hello my client")
    }
    pub async fn chantiers(
        &self,
        ctx: &Context<'_>,
        #[graphql(default)] pagination: OffsetLimit,
    ) -> crate::Result<ResultsData<ClientChantier>> {
        let current_client = ctx
            .data::<CurrentClient>()
            .clone()
            .map_err(|_| crate::Error::Forbidden)?
            .clone();
        let db = ctx.data::<DbPool>().map_err(crate::Error::GraphQL)?;
        let mut pool = db.get()?;
        let res = block(move || {
            use crate::views::v_chantier_finition::dsl::*;
            let (data, total) = v_chantier_finition
                .filter(client.eq(current_client.0.telephone.clone()))
                .select(VChantierFinition::as_select())
                .paginate_with_param(pagination)
                .load_and_count_pages::<VChantierFinition>(&mut pool)?;
            Ok::<ResultsData<ClientChantier>, crate::Error>(ResultsData {
                data: data
                    .into_iter()
                    .map(|i| -> ClientChantier { i.into() })
                    .collect(),
                limit: pagination.limit,
                offset: pagination.offset,
                total,
            })
        })
        .await??;
        Ok(res)
    }
}
