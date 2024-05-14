use crate::{generate_upserts, graphql::objects::unite::UniteInput, models::unite::Unite};

use async_graphql::Object;
use diesel::prelude::*;

pub struct AdminUniteMutations;

#[Object]
impl AdminUniteMutations {
    generate_upserts!(
        UniteInput,
        Unite,
        unite,
        id_unite,
        crate::schema::unite::dsl
    );
    pub async fn status(&self) -> String {
        "Running HEhe!".into()
    }
}
