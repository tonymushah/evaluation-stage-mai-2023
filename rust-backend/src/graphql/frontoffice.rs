pub mod mutation;
pub mod query;

use std::ops::{Deref, DerefMut};

use actix_web::{get, post, web, HttpRequest, HttpResponse};
use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

use crate::ServerState;

use self::query::FrontOfficeQuery;

#[derive(Clone)]
pub struct FrontOfficeSchema(Schema<FrontOfficeQuery, EmptyMutation, EmptySubscription>);

impl Deref for FrontOfficeSchema {
    type Target = Schema<FrontOfficeQuery, EmptyMutation, EmptySubscription>;
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
            EmptyMutation,
            EmptySubscription,
        ))
    }
}

pub fn new_front_office_schema() -> FrontOfficeSchema {
    FrontOfficeSchema::default()
}

#[post("/front-office")]
pub async fn front_office(
    state: web::Data<ServerState>,
    req: HttpRequest,
    gql_request: GraphQLRequest,
) -> GraphQLResponse {
    let mut request = gql_request.into_inner();
    request = request.data(state.db.clone());
    state.front_office.execute(request).await.into()
}

#[get("/front-office")]
pub async fn front_office_graphiql() -> actix_web::Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(GraphiQLSource::build().endpoint("/front-office").finish()))
}
