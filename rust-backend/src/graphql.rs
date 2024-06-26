use async_graphql::{InputObject, SimpleObject};
use diesel::{query_dsl::methods::LoadQuery, PgConnection, QueryResult};

use self::{
    admin::objects::{
        chantier::AdminChantier, clients::AdminClient, devis::AdminDevis, finition::AdminFinition,
        materiels::AdminMateriel, type_chantier::AdminTypeChantier, unite::AdminUnite,
    },
    frontoffice::models::chantier::ClientChantier,
};

use crate::models::{finition::Finition, type_chantier::TypeChantier, Paginated};

pub mod admin;
pub mod frontoffice;
pub mod objects;

#[derive(Debug, Clone, Copy, InputObject)]
pub struct OffsetLimit {
    pub offset: i64,
    pub limit: i64,
}

impl Default for OffsetLimit {
    fn default() -> Self {
        Self {
            offset: 0,
            limit: 10,
        }
    }
}

#[derive(Debug, Clone, SimpleObject)]
#[graphql(concrete(name = "ClientChantierResults", params(ClientChantier)))]
#[graphql(concrete(name = "FinitionResults", params(Finition)))]
#[graphql(concrete(name = "TypeChantierResults", params(TypeChantier)))]
#[graphql(concrete(name = "AdminChantierResults", params(AdminChantier)))]
#[graphql(concrete(name = "AdminClientResults", params(AdminClient)))]
#[graphql(concrete(name = "AdminFinitionResults", params(AdminFinition)))]
#[graphql(concrete(name = "AdminMaterielResults", params(AdminMateriel)))]
#[graphql(concrete(name = "AdminTypeChantierResults", params(AdminTypeChantier)))]
#[graphql(concrete(name = "AdminUniteResults", params(AdminUnite)))]
#[graphql(concrete(name = "AdminDevisResults", params(AdminDevis)))]
pub struct ResultsData<T>
where
    T: async_graphql::OutputType,
{
    pub data: Vec<T>,
    pub limit: i64,
    pub offset: i64,
    pub total: i64,
}

impl<T> Paginated<T> {
    pub fn to_results_data<'a, U>(self, conn: &mut PgConnection) -> QueryResult<ResultsData<U>>
    where
        Self: LoadQuery<'a, PgConnection, (U, i64)>,
        U: async_graphql::OutputType,
    {
        let offset = self.get_offset();
        let limit = self.get_limit();
        let (data, total) = self.load_and_count_pages(conn)?;
        Ok(ResultsData {
            data,
            limit,
            offset,
            total,
        })
    }
}

pub(crate) fn get_pool(ctx: &async_graphql::Context<'_>) -> crate::Result<crate::DbPoolConnection> {
    Ok(ctx
        .data::<crate::DbPool>()
        .map_err(crate::Error::GraphQL)?
        .get()?)
}
