use crate::{
    generate_upserts, graphql::objects::type_chantier::TypeChantierInput,
    models::type_chantier::TypeChantier,
};

use async_graphql::Object;
use diesel::prelude::*;

pub struct AdminTypeChantierMutations;

#[Object]
impl AdminTypeChantierMutations {
    generate_upserts!(
        TypeChantierInput,
        TypeChantier,
        type_chantier,
        id_type_chantier,
        crate::schema::type_chantier::dsl
    );
    pub async fn status(&self) -> String {
        "Running HEhe!".into()
    }
}
