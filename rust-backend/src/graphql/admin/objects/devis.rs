use async_graphql::ComplexObject;

use crate::{generate_admin_object, models::devis::Devis};

generate_admin_object!(AdminDevis, Devis);

#[ComplexObject]
impl AdminDevis {}
