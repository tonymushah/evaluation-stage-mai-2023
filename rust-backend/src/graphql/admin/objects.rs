pub mod chantier;
pub mod clients;
pub mod devis;
pub mod finition;
pub mod type_chantier;
pub mod unite;

#[macro_export]
macro_rules! generate_admin_object {
    ($name: ident, $item: ty) => {
        #[derive(Clone, Debug, async_graphql::SimpleObject)]
        #[graphql(complex)]
        pub struct $name {
            #[graphql(flatten)]
            item: $item,
        }
        impl From<$name> for $item {
            fn from(value: $name) -> Self {
                value.item
            }
        }
        impl From<$item> for $name {
            fn from(item: $item) -> Self {
                Self { item }
            }
        }
        impl std::ops::Deref for $name {
            type Target = $item;
            fn deref(&self) -> &Self::Target {
                &self.item
            }
        }
    };
}
