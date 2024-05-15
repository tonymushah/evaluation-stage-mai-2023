use async_graphql::ComplexObject;

use crate::{generate_admin_object, models::finition::Finition};

generate_admin_object!(AdminFinition, Finition);

#[ComplexObject]
impl AdminFinition {}
