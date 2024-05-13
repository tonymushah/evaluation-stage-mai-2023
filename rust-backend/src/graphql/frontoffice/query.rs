use async_graphql::Object;

#[derive(Debug, Clone, Copy, Hash, Default)]
pub struct FrontOfficeQuery;

#[Object]
impl FrontOfficeQuery {
    pub async fn hello(&self) -> String {
        String::from("Hello my client")
    }
}
