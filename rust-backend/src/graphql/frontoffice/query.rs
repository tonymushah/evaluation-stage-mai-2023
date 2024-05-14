pub mod chantier;
pub mod finitions;
pub mod type_chantier;

use async_graphql::{Context, Object};

use self::{
    chantier::ClientChantierQueries, finitions::ClientFinitionQueries,
    type_chantier::ClientTypeChantierQueries,
};

use super::CurrentClient;

#[derive(Debug, Clone, Copy, Hash, Default)]
pub struct FrontOfficeQuery;

#[Object]
impl FrontOfficeQuery {
    pub async fn hello(&self) -> String {
        String::from("Hello my client")
    }
    pub async fn chantiers(&self, ctx: &Context<'_>) -> crate::Result<ClientChantierQueries> {
        let current_client = ctx
            .data::<CurrentClient>()
            .clone()
            .map_err(|_| crate::Error::Forbidden)?
            .clone();
        Ok(ClientChantierQueries(current_client))
    }
    pub async fn finitions(&self) -> ClientFinitionQueries {
        ClientFinitionQueries
    }
    pub async fn type_chantier(&self) -> ClientTypeChantierQueries {
        ClientTypeChantierQueries
    }
}
