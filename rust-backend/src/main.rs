use actix_web::{web, App, HttpServer};
use rust_backend::{
    graphql::frontoffice::{front_office, front_office_graphiql},
    ServerState,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = ServerState::default();
    println!("Server started at http://127.0.0.1:1354");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .service(front_office)
            .service(front_office_graphiql)
    })
    .bind(("127.0.0.1", 1354))?
    .run()
    .await
}
