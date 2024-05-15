use async_graphql::ComplexObject;

use crate::{generate_admin_object, models::materiel::Materiel};

generate_admin_object!(AdminMateriel, Materiel);

#[ComplexObject]
impl AdminMateriel {}
