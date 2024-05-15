use async_graphql::ComplexObject;

use crate::{generate_admin_object, models::unite::Unite};

generate_admin_object!(AdminUnite, Unite);

#[ComplexObject]
impl AdminUnite {}
