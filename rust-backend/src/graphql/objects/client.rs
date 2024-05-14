use async_graphql::InputObject;

use crate::models::client::Client;

#[derive(Clone, Debug, InputObject)]
pub struct ClientInput {
    pub telephone: String,
}

impl From<ClientInput> for Client {
    fn from(value: ClientInput) -> Self {
        Self {
            telephone: value.telephone,
        }
    }
}
