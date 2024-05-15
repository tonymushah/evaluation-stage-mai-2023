pub mod create_chantier;

use std::ops::Deref;

use actix_web::web::block;
use async_graphql::{Context, Object};
use jwt::SignWithKey;
use uuid::Uuid;

use crate::{
    graphql::get_pool,
    models::{
        chantier::Chantier, client::Client, finition::Finition,
        v_chantier_finition::VChantierFinition,
    },
    ClientHmac, DbPool, DbPoolConnection,
};

use self::create_chantier::ClientCreateChantier;

use super::{models::chantier::ClientChantier, CurrentClient};

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
    pub async fn create_chantier(
        &self,
        ctx: &Context<'_>,
        input: ClientCreateChantier,
    ) -> crate::Result<ClientChantier> {
        let current_client = ctx
            .data::<CurrentClient>()
            .clone()
            .map_err(|_| crate::Error::Forbidden)?
            .clone();
        let mut pool = get_pool(ctx)?;
        let (id, mut pool) = block(move || -> crate::Result<(Uuid, DbPoolConnection)> {
            use crate::schema::chantier::dsl::*;

            use diesel::prelude::*;
            let finition_ = Finition::get_first_standard(&mut pool)?;
            let input: Chantier = input
                .into_chantier_input(finition_.id_finition, current_client.telephone.clone())
                .into();
            let output: Chantier = diesel::insert_into(chantier)
                .values(&input)
                .get_results(&mut pool)?
                .first()
                .cloned()
                .ok_or(crate::Error::UpsertNotFound)?;
            Ok((output.id_chantier, pool))
        })
        .await??;
        block(move || {
            use crate::views::v_chantier_finition::dsl::*;
            use diesel::prelude::*;
            v_chantier_finition
                .filter(id_chantier.eq(id))
                .select(VChantierFinition::as_select())
                .load(&mut pool)?
                .first()
                .cloned()
                .map(|i| i.into())
                .ok_or(crate::Error::UpsertNotFound)
        })
        .await?
    }
}
