use crate::{
    generate_upserts, graphql::objects::materiel::MaterielInput, models::materiel::Materiel,
};

use async_graphql::Object;
use diesel::prelude::*;

pub struct AdminMaterielMutations;

#[Object]
impl AdminMaterielMutations {
    generate_upserts!(
        MaterielInput,
        Materiel,
        materiels,
        code,
        crate::schema::materiels::dsl
    );
    pub async fn status(&self) -> String {
        "Running HEhe!".into()
    }
}
