use async_graphql::ComplexObject;

use crate::{generate_admin_object, models::type_chantier::TypeChantier};

generate_admin_object!(AdminTypeChantier, TypeChantier);

#[ComplexObject]
impl AdminTypeChantier {}
