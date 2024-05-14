use crate::{
    generate_upserts, graphql::objects::chantier::ChantierInput, models::chantier::Chantier,
};

use async_graphql::Object;
use diesel::prelude::*;

pub struct AdminChantierMutations;

#[Object]
impl AdminChantierMutations {
    generate_upserts!(
        ChantierInput,
        Chantier,
        chantier,
        id_chantier,
        crate::schema::chantier::dsl
    );
    pub async fn status(&self) -> String {
        "Running HEhe!".into()
    }
}
