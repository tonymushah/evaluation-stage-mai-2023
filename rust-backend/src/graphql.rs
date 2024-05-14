use async_graphql::{InputObject, SimpleObject};
use diesel::{query_dsl::methods::LoadQuery, PgConnection, QueryResult};

use self::frontoffice::models::chantier::ClientChantier;

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
