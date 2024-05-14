pub mod mutation;
pub mod query;

use std::ops::{Deref, DerefMut};

use actix_web::{get, http::header::AUTHORIZATION, post, web, HttpRequest, HttpResponse};
use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use jwt::VerifyWithKey;
use serde::{Deserialize, Serialize};

use crate::{models::client::Client, ServerState};

use self::{mutation::FrontOfficeMutation, query::FrontOfficeQuery};

#[derive(Clone)]
pub struct FrontOfficeSchema(Schema<FrontOfficeQuery, FrontOfficeMutation, EmptySubscription>);

impl Deref for FrontOfficeSchema {
    type Target = Schema<FrontOfficeQuery, FrontOfficeMutation, EmptySubscription>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for FrontOfficeSchema {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Default for FrontOfficeSchema {
    fn default() -> Self {
        FrontOfficeSchema(Schema::new(
            FrontOfficeQuery,
            FrontOfficeMutation,
            EmptySubscription,
        ))
    }
}

pub fn new_front_office_schema() -> FrontOfficeSchema {
    FrontOfficeSchema::default()
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CurrentClient(Client);

impl Deref for CurrentClient {
    type Target = Client;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Client> for CurrentClient {
    fn from(value: Client) -> Self {
        Self(value)
    }
}

#[post("/front-office")]
pub async fn front_office(
    state: web::Data<ServerState>,
    req: HttpRequest,
    gql_request: GraphQLRequest,
) -> GraphQLResponse {
    let mut request = gql_request
        .into_inner()
        .data(state.db.clone())
        .data(state.client_hmac.clone());
    if let Some(head) = req.headers().get(AUTHORIZATION).and_then(|head| {
        let head = String::from(head.to_str().ok()?);
        let res: Option<CurrentClient> = head.verify_with_key(state.client_hmac.deref()).ok();
        res
    }) {
        request = request.data(head);
    }
    state.front_office.execute(request).await.into()
}

#[get("/front-office")]
pub async fn front_office_graphiql() -> actix_web::Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(GraphiQLSource::build().endpoint("/front-office").finish()))
}
