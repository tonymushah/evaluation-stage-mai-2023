pub mod error;
pub mod graphql;
pub mod models;
pub mod reset;
pub mod schema;
pub mod views;

pub use error::Error;

pub type Result<T, E = crate::error::Error> = std::result::Result<T, E>;

use std::{env, ops::Deref};

// add the `r2d2` feature for diesel
use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    PgConnection,
};
use dotenvy::dotenv;
use graphql::{admin::AdminSchema, frontoffice::FrontOfficeSchema};
use hmac::{Hmac, Mac};
use sha2::Sha256;

// set an alias, so we don't have to keep writing out this long type
pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub type DbPoolConnection = PooledConnection<ConnectionManager<PgConnection>>;

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
    pub client_hmac: ClientHmac,
    pub admin_hmac: AdminHmac,
}

#[derive(Debug, Clone)]
pub struct ClientHmac(Hmac<Sha256>);

impl Deref for ClientHmac {
    type Target = Hmac<Sha256>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct AdminHmac(Hmac<Sha256>);

impl Deref for AdminHmac {
    type Target = Hmac<Sha256>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for ServerState {
    fn default() -> Self {
        let client_hmac = Hmac::<Sha256>::new_from_slice(
            env::var("CLIENT_TOKEN")
                .expect("CLIENT_TOKEN env is required")
                .as_bytes(),
        )
        .expect("Error on load the CLIENT_TOKEN");
        let admin_hmac = Hmac::<Sha256>::new_from_slice(
            env::var("ADMIN_TOKEN")
                .expect("ADMIN_TOKEN env is required")
                .as_bytes(),
        )
        .expect("Error on load the CLIENT_TOKEN");
        Self {
            db: etablish_connection(),
            front_office: Default::default(),
            admin: Default::default(),
            admin_hmac: AdminHmac(admin_hmac),
            client_hmac: ClientHmac(client_hmac),
        }
    }
}
