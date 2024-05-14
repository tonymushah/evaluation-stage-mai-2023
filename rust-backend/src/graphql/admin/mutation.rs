pub mod chantier;
pub mod devis;
pub mod finition;
pub mod materiels;
pub mod type_chantier;
pub mod unite;

use actix_web::web;
use async_graphql::{Context, Object};

use crate::{reset::reset_db, DbPool};

use self::{
    chantier::AdminChantierMutations, devis::AdminDevisMutations, finition::AdminFinitionMutations,
    materiels::AdminMaterielMutations, type_chantier::AdminTypeChantierMutations,
    unite::AdminUniteMutations,
};

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
    pub async fn chantier(&self) -> AdminChantierMutations {
        AdminChantierMutations
    }
    pub async fn devis(&self) -> AdminDevisMutations {
        AdminDevisMutations
    }
    pub async fn finition(&self) -> AdminFinitionMutations {
        AdminFinitionMutations
    }
    pub async fn materiel(&self) -> AdminMaterielMutations {
        AdminMaterielMutations
    }
    pub async fn type_chantier(&self) -> AdminTypeChantierMutations {
        AdminTypeChantierMutations
    }
    pub async fn unite(&self) -> AdminUniteMutations {
        AdminUniteMutations
    }
}

#[macro_export]
macro_rules! generate_upserts {
    ($input: ty, $output: ty, $table: expr, $id: expr, $dsl: path) => {
        pub async fn upsert_data(
            &self,
            ctx: &async_graphql::Context<'_>,
            input: $input,
        ) -> $crate::Result<$output> {
            let mut pool = ctx
                .data::<$crate::DbPool>()
                .map_err($crate::Error::GraphQL)?
                .get()?;
            actix_web::web::block(move || -> $crate::Result<$output> {
                use $dsl::*;
                let to_input: $output = input.into();
                diesel::insert_into($table)
                    .values(&to_input)
                    .on_conflict($id)
                    .do_update()
                    .set(&to_input)
                    .get_results(&mut pool)?
                    .first()
                    .cloned()
                    .ok_or($crate::Error::UpsertNotFound)
            })
            .await?
        }
        pub async fn upsert_data_batch(
            &self,
            ctx: &async_graphql::Context<'_>,
            input: Vec<$input>,
        ) -> $crate::Result<Vec<$output>> {
            let mut pool = ctx
                .data::<$crate::DbPool>()
                .map_err($crate::Error::GraphQL)?
                .get()?;
            actix_web::web::block(move || -> $crate::Result<Vec<$output>> {
                use $dsl::*;
                let to_input: Vec<$output> = input.into_iter().map(|i| i.into()).collect();
                let mut res = diesel::insert_into($table)
                    .values(&to_input)
                    .on_conflict($id)
                    .do_nothing()
                    .get_results(&mut pool)?;
                for i in &to_input {
                    res.append(&mut diesel::update($table).set(i).get_results(&mut pool)?);
                }
                Ok(res)
            })
            .await?
        }
    };
}
