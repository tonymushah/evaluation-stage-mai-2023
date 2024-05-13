use async_graphql::Object;

#[derive(Debug, Clone, Copy, Hash, Default)]
pub struct AdminQuery;

#[Object]
impl AdminQuery {
    pub async fn hello(&self) -> String {
        String::from("Hello my client")
    }
}
