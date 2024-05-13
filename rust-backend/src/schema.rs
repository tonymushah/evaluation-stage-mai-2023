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
    materiels (code) {
        #[max_length = 25]
        code -> Varchar,
        designation -> Text,
        prix_unitaire -> Nullable<Numeric>,
        unite_id -> Uuid,
    }
}

diesel::table! {
    unite (id_unite) {
        id_unite -> Uuid,
        designation -> Text,
    }
}

diesel::joinable!(materiels -> unite (unite_id));

diesel::allow_tables_to_appear_in_same_query!(
    clients,
    finition,
    materiels,
    unite,
);
