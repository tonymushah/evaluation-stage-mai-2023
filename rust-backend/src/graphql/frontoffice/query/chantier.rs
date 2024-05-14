use actix_web::web::block;
use async_graphql::{Context, Object};
use diesel::prelude::*;

use crate::{
    graphql::{OffsetLimit, ResultsData},
    models::{v_chantier_finition::VChantierFinition, Paginate},
    DbPool,
};

use crate::graphql::frontoffice::{models::chantier::ClientChantier, CurrentClient};

#[derive(Clone, Debug)]
pub struct ClientChantierQueries(pub(super) CurrentClient);

#[Object]
impl ClientChantierQueries {
    pub async fn list(
        &self,
        ctx: &Context<'_>,
        #[graphql(default)] pagination: OffsetLimit,
    ) -> crate::Result<ResultsData<ClientChantier>> {
        let current_client = self.0.clone();
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
