use std::ops::Deref;

use actix_web::web::block;
use async_graphql::{Context, Object};
use jwt::SignWithKey;

use crate::{models::client::Client, ClientHmac, DbPool};

use super::CurrentClient;

#[derive(Debug, Clone, Copy)]
pub struct FrontOfficeMutation;

#[Object]
impl FrontOfficeMutation {
    pub async fn login(&self, ctx: &Context<'_>, telephone: String) -> crate::Result<String> {
        let db = ctx.data::<DbPool>().map_err(crate::Error::GraphQL)?;
        let key = ctx.data::<ClientHmac>().map_err(crate::Error::GraphQL)?;
        let mut pool = db.get()?;
        let current_client = block(move || {
            Ok::<CurrentClient, crate::Error>(Client::login(telephone, &mut pool)?.into())
        })
        .await??;
        Ok(current_client.sign_with_key(key.deref())?)
    }
}
