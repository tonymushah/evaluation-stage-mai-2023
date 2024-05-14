use crate::{graphql::objects::client::ClientInput, models::client::Client, DbPool};

use async_graphql::{Context, Object};
use diesel::prelude::*;

pub struct AdminClientMutations;

#[Object]
impl AdminClientMutations {
    pub async fn insert(&self, ctx: &Context<'_>, input: ClientInput) -> crate::Result<Client> {
        let mut pool = ctx.data::<DbPool>().map_err(crate::Error::GraphQL)?.get()?;
        actix_web::web::block(move || -> crate::Result<Client> {
            use crate::schema::clients::dsl::*;
            let to_input: Client = input.into();
            diesel::insert_into(clients)
                .values(&to_input)
                .get_results(&mut pool)?
                .first()
                .cloned()
                .ok_or(crate::Error::UpsertNotFound)
        })
        .await?
    }
    pub async fn insert_batch(
        &self,
        ctx: &Context<'_>,
        input: Vec<ClientInput>,
    ) -> crate::Result<Vec<Client>> {
        let mut pool = ctx.data::<DbPool>().map_err(crate::Error::GraphQL)?.get()?;
        actix_web::web::block(move || -> crate::Result<Vec<Client>> {
            use crate::schema::clients::dsl::*;
            let to_input: Vec<Client> = input.into_iter().map(|i| i.into()).collect();
            Ok(diesel::insert_into(clients)
                .values(&to_input)
                .get_results(&mut pool)?)
        })
        .await?
    }
}
