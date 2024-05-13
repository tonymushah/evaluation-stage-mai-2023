// @generated automatically by Diesel CLI.

diesel::table! {
    clients (telephone) {
        #[max_length = 25]
        telephone -> Varchar,
    }
}

diesel::table! {
    finition (id_finition) {
        id_finition -> Uuid,
        designation -> Text,
        prix -> Numeric,
        duree -> Numeric,
    }
}

diesel::table! {
    unite (id_unite) {
        id_unite -> Uuid,
        designation -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    clients,
    finition,
    unite,
);
