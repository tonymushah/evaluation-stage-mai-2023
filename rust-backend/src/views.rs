use diesel::table;

table! {
    v_materiels_unite (code, unite_id) {
        #[max_length = 25]
        code -> Varchar,
        designation -> Text,
        prix_unitaire -> Numeric,
        unite_id -> Uuid,
        unite -> Text
    }
}
