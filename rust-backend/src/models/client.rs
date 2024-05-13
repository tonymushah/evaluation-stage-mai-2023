use crate::schema::clients;
use async_graphql::SimpleObject;
use diesel::{associations::Identifiable, Selectable};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, SimpleObject, Identifiable, Selectable)]
#[diesel(table_name = clients)]
#[primary_key(telephone)]
pub struct Client {
    telephone: String,
}

impl Client {
    pub fn new(telephone: String) -> Self {
        Self { telephone }
    }
}
