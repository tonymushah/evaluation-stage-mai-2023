use crate::{
    generate_upserts, graphql::objects::finition::FinitionInput, models::finition::Finition,
};

use async_graphql::Object;
use diesel::prelude::*;

pub struct AdminFinitionMutations;

#[Object]
impl AdminFinitionMutations {
    generate_upserts!(
        FinitionInput,
        Finition,
        finition,
        id_finition,
        crate::schema::finition::dsl
    );
    pub async fn status(&self) -> String {
        "Running HEhe!".into()
    }
}
