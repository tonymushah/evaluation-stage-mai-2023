use async_graphql::ErrorExtensions;
use diesel::r2d2;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Pool(#[from] r2d2::Error),
    #[error("{0}")]
    DieselConnection(#[from] diesel::ConnectionError),
    #[error("{0}")]
    Diesel(#[from] diesel::result::Error),
    #[error("{0}")]
    ActixWeb(#[from] actix_web::error::Error),
}

impl ErrorExtensions for Error {
    fn extend(&self) -> async_graphql::Error {
        async_graphql::Error::new(format!("{}", self)).extend_with(|_err, e| match self {
            Error::Pool(_) => e.set("code", "DB_POOL"),
            Error::DieselConnection(_) => e.set("code", "DB_CONNECT"),
            Error::Diesel(er) => match er {
                diesel::result::Error::InvalidCString(_) => e.set("code", "INVALID_CS_STRING"),
                diesel::result::Error::DatabaseError(_, _) => e.set("code", "DB_INNER"),
                diesel::result::Error::NotFound => e.set("code", "NOT_FOUND"),
                diesel::result::Error::QueryBuilderError(_) => e.set("code", "QUERY_BUILDER"),
                diesel::result::Error::DeserializationError(_) => e.set("code", "DESERIALIZATION"),
                diesel::result::Error::SerializationError(_) => e.set("code", "SERIALIZATION"),
                diesel::result::Error::RollbackErrorOnCommit {
                    rollback_error,
                    commit_error,
                } => {
                    e.set("code", "ROLLBACK_ON_COMMIT");
                    e.set("rollback", rollback_error.to_string());
                    e.set("commit", commit_error.to_string())
                }
                diesel::result::Error::RollbackTransaction => e.set("code", "ROLLBACK_TRANSACTION"),
                diesel::result::Error::AlreadyInTransaction => {
                    e.set("code", "ALREADY_IN_TRANSACTION")
                }
                diesel::result::Error::NotInTransaction => e.set("code", "NOT_IN_TRANSACTION"),
                diesel::result::Error::BrokenTransactionManager => {
                    e.set("code", "BROKEN_TRANSACTION_MANAGER")
                }
                _ => e.set("code", "UNKNOW_DIESEL_ERROR"),
            },
            Error::ActixWeb(_) => e.set("code", "ACTIX_WEB"),
        })
    }
}
