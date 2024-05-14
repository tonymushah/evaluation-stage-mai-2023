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

table! {
    v_devis_materiel (id_devis, unite_id, materiel_id) {
        id_devis -> Uuid,
        type_chantier -> Uuid,
        #[max_length = 25]
        materiel_id -> Varchar,
        materiel -> Text,
        quantite -> Numeric,
        prix_unitaire -> Numeric,
        unite -> Text,
        unite_id -> Uuid,
        prix_total -> Numeric
    }
}

table! {
    v_chantier_finition(id_chantier, id_finition, client) {
        id_chantier -> Uuid,
        #[max_length = 25]
        client -> VarChar,
        date_debut -> Nullable<Date>,
        finition_prix -> Numeric,
        finition_duree -> Numeric,
        finition -> Text,
        id_finition -> Uuid,
        type_chantier_id -> Uuid
    }
}
