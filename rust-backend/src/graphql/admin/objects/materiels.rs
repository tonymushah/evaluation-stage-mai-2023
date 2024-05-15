use actix_web::web::block;
use async_graphql::{ComplexObject, Context};

use crate::{
    generate_admin_object,
    graphql::get_pool,
    models::{materiel::Materiel, unite::Unite},
};

use super::unite::AdminUnite;

generate_admin_object!(AdminMateriel, Materiel);

#[ComplexObject]
impl AdminMateriel {
    pub async fn unite(&self, ctx: &Context<'_>) -> crate::Result<AdminUnite> {
        let mut pool = get_pool(ctx)?;
        let id = self.unite_id;
        block(move || -> crate::Result<AdminUnite> {
            use crate::schema::unite::dsl::*;
            use diesel::prelude::*;
            unite
                .filter(id_unite.eq(id))
                .limit(1)
                .select(Unite::as_select())
                .load(&mut pool)?
                .first()
                .cloned()
                .map(|i| i.into())
                .ok_or(crate::Error::Diesel(diesel::result::Error::NotFound))
        })
        .await?
    }
}
