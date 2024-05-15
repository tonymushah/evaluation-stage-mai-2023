pub mod mutation;
pub mod objects;
pub mod query;

use std::ops::{Deref, DerefMut};

use actix_web::{get, post, web, HttpRequest, HttpResponse};
use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

use crate::{reset::reset_db, ServerState};

use self::{mutation::AdminMutation, query::AdminQuery};

#[derive(Clone)]
pub struct AdminSchema(Schema<AdminQuery, AdminMutation, EmptySubscription>);

impl Deref for AdminSchema {
    type Target = Schema<AdminQuery, AdminMutation, EmptySubscription>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for AdminSchema {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Default for AdminSchema {
    fn default() -> Self {
        Self(Schema::new(AdminQuery, AdminMutation, EmptySubscription))
    }
}

#[post("/admin")]
pub async fn admin(
    state: web::Data<ServerState>,
    _req: HttpRequest,
    gql_request: GraphQLRequest,
) -> GraphQLResponse {
    let mut request = gql_request.into_inner();
    request = request.data(state.db.clone());
    state.admin.execute(request).await.into()
}

#[get("/admin")]
pub async fn admin_graphiql() -> actix_web::Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(GraphiQLSource::build().endpoint("/admin").finish()))
}

#[get("/admin/reset")]
pub async fn admin_reset(state: web::Data<ServerState>) -> actix_web::Result<HttpResponse> {
    let mut pool = state.db.clone();
    web::block(move || reset_db(&mut pool))
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body("reseted"))
}
