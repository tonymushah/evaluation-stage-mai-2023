pub mod error;
pub mod graphql;
pub mod models;
pub mod reset;
pub mod schema;

pub type Result<T, E = crate::error::Error> = std::result::Result<T, E>;

use std::env;

// add the `r2d2` feature for diesel
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use dotenvy::dotenv;
use graphql::{admin::AdminSchema, frontoffice::FrontOfficeSchema};

// set an alias, so we don't have to keep writing out this long type
pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn etablish_connection() -> DbPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    Pool::builder()
        .build(manager)
        .expect("Failed to create a pool.")
}

#[derive(Clone)]
pub struct ServerState {
    pub db: DbPool,
    pub front_office: FrontOfficeSchema,
    pub admin: AdminSchema,
}

impl Default for ServerState {
    fn default() -> Self {
        Self {
            db: etablish_connection(),
            front_office: Default::default(),
            admin: Default::default(),
        }
    }
}
