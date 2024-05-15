use async_graphql::ComplexObject;

use crate::{generate_admin_object, models::chantier::Chantier};

generate_admin_object!(AdminChantier, Chantier);

#[ComplexObject]
impl AdminChantier {}
