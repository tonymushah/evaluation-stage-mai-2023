use crate::{generate_upserts, graphql::objects::devis::DevisInput, models::devis::Devis};

use async_graphql::Object;
use diesel::prelude::*;

pub struct AdminDevisMutations;

#[Object]
impl AdminDevisMutations {
    generate_upserts!(
        DevisInput,
        Devis,
        devis,
        id_devis,
        crate::schema::devis::dsl
    );
    pub async fn status(&self) -> String {
        "Running HEhe!".into()
    }
}
