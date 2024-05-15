pub mod chantier;
pub mod clients;
pub mod devis;
pub mod finition;
pub mod materiels;
pub mod type_chantier;

use async_graphql::Object;

use self::{
    chantier::AdminChantierQuery, clients::AdminClientQuery, devis::AdminDevisQuery,
    finition::AdminFinitionQuery, materiels::AdminMaterielQuery,
    type_chantier::AdminTypeChantierQuery,
};

#[derive(Debug, Clone, Copy, Hash, Default)]
pub struct AdminQuery;

#[Object]
impl AdminQuery {
    pub async fn hello(&self) -> String {
        String::from("Hello my client")
    }
    pub async fn chantier(&self) -> AdminChantierQuery {
        AdminChantierQuery
    }
    pub async fn clients(&self) -> AdminClientQuery {
        AdminClientQuery
    }
    pub async fn devis(&self) -> AdminDevisQuery {
        AdminDevisQuery
    }
    pub async fn finition(&self) -> AdminFinitionQuery {
        AdminFinitionQuery
    }
    pub async fn materiels(&self) -> AdminMaterielQuery {
        AdminMaterielQuery
    }
    pub async fn type_chantier(&self) -> AdminTypeChantierQuery {
        AdminTypeChantierQuery
    }
}

#[macro_export]
macro_rules! generate_pagination {
    ($input: ty, $output: ty, $table: expr, $id: expr, $id_type: ty, $dsl: path) => {
        pub async fn get_list(
            &self,
            ctx: &async_graphql::Context<'_>,
            input: $crate::graphql::OffsetLimit,
        ) -> $crate::Result<$crate::graphql::ResultsData<$output>> {
            let mut pool = $crate::graphql::get_pool(ctx)?;
            actix_web::web::block(
                move || -> $crate::Result<$crate::graphql::ResultsData<$output>> {
                    use diesel::prelude::*;
                    use $crate::models::Paginate;
                    use $dsl::*;
                    let (data, total) = $table
                        .select(<$input as SelectableHelper<diesel::pg::Pg>>::as_select())
                        .paginate_with_param(input)
                        .load_and_count_pages::<$input>(&mut pool)?;
                    let data: Vec<$output> = data.into_iter().map(|i| i.into()).collect();
                    Ok($crate::graphql::ResultsData {
                        data,
                        total,
                        limit: input.limit,
                        offset: input.offset,
                    })
                },
            )
            .await?
        }
        pub async fn get_unique(
            &self,
            ctx: &async_graphql::Context<'_>,
            id: $id_type,
        ) -> $crate::Result<$output> {
            let mut pool = $crate::graphql::get_pool(ctx)?;
            actix_web::web::block(move || -> $crate::Result<$output> {
                use diesel::prelude::*;
                use $dsl::*;
                let data = $table
                    .filter($id.eq(id))
                    .select(<$input as SelectableHelper<diesel::pg::Pg>>::as_select())
                    .load::<$input>(&mut pool)?
                    .first()
                    .cloned()
                    .ok_or($crate::Error::Diesel(diesel::result::Error::NotFound))?;
                Ok(data.into())
            })
            .await?
        }
    };
}
