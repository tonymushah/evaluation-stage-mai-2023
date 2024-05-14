use crate::{schema::clients, DbPoolConnection};
use async_graphql::SimpleObject;
use diesel::{associations::HasTable, insert_into, prelude::*};
use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    SimpleObject,
    Identifiable,
    Selectable,
    Insertable,
    Deserialize,
    Serialize,
    Queryable,
)]
#[diesel(table_name = clients)]
#[diesel(primary_key(telephone))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Client {
    pub(crate) telephone: String,
}

impl Client {
    pub fn new(telephone: String) -> Self {
        Self { telephone }
    }
    pub fn login(tel: String, con: &mut DbPoolConnection) -> crate::Result<Self> {
        use crate::schema::clients::dsl::*;
        let getted: Vec<Self> = clients
            .filter(telephone.eq(&tel))
            .limit(1)
            .select(Self::as_select())
            .load(con)?;
        if let Some(cli) = getted.first() {
            Ok(cli.clone())
        } else {
            Ok(insert_into(clients::table())
                .values(&Self::new(tel))
                .returning(Self::as_returning())
                .get_result(con)?)
        }
    }
}
