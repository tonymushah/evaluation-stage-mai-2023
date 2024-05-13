// @generated automatically by Diesel CLI.

diesel::table! {
    clients (telephone) {
        #[max_length = 25]
        telephone -> Varchar,
    }
}
