use async_graphql::ComplexObject;

use crate::{generate_admin_object, models::client::Client};

generate_admin_object!(AdminClient, Client);

#[ComplexObject]
impl AdminClient {}
