use async_graphql::Object;

#[derive(Debug, Clone, Copy, Hash, Default)]
pub struct AdminQuery;

#[Object]
impl AdminQuery {
    pub async fn hello(&self) -> String {
        String::from("Hello my client")
    }
}

#[macro_export]
macro_rules! generate_pagination {
    ($input: ty, $output: ty, $table: expr, $id: expr, $dsl: path) => {
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
                        .select($input::as_select())
                        .paginate_with_params(input)
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
    };
}
