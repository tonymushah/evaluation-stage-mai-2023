use actix_web::web;
use async_graphql::{Context, Object};

use crate::{reset::reset_db, DbPool};

#[derive(Debug, Clone, Copy, Hash, Default)]
pub struct AdminMutation;

#[Object]
impl AdminMutation {
    pub async fn reset_db(&self, context: &Context<'_>) -> crate::Result<bool> {
        let mut pool = context
            .data::<DbPool>()
            .map_err(crate::error::Error::GraphQL)?
            .clone();
        web::block(move || reset_db(&mut pool)).await??;
        Ok(true)
    }
}
