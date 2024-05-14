use actix_web::{
    web::{self},
    App, HttpServer,
};
use dotenvy::dotenv;
use rust_backend::{
    graphql::{
        admin::{admin, admin_graphiql, admin_reset},
        frontoffice::{front_office, front_office_graphiql},
    },
    ServerState,
};

use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let state = ServerState::default();
    let port: u16 = env::var("BACKEND_PORT").unwrap().parse().unwrap();
    let adress = env::var("BACKEND_HOST").unwrap();
    println!("Server started at http://{adress}:{port}");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .service(front_office)
            .service(front_office_graphiql)
            .service(admin)
            .service(admin_graphiql)
            .service(admin_reset)
    })
    .bind((adress, port))?
    .run()
    .await
}
