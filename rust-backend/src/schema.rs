// @generated automatically by Diesel CLI.

diesel::table! {
    chantier (id_chantier) {
        id_chantier -> Uuid,
        #[max_length = 25]
        client -> Varchar,
        type_chantier_id -> Uuid,
        type_finition_id -> Uuid,
        date_debut -> Nullable<Date>,
    }
}

diesel::table! {
    clients (telephone) {
        #[max_length = 25]
        telephone -> Varchar,
    }
}

diesel::table! {
    devis (id_devis) {
        id_devis -> Uuid,
        type_chantier_id -> Uuid,
        #[max_length = 25]
        materiel_id -> Varchar,
        quantite -> Numeric,
    }
}

diesel::table! {
    finition (id_finition) {
        id_finition -> Uuid,
        designation -> Text,
        prix -> Numeric,
        duree -> Numeric,
        is_standard -> Nullable<Bool>,
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
    type_chantier (id_type_chantier) {
        id_type_chantier -> Uuid,
        nom -> Text,
        description -> Text,
    }
}

diesel::table! {
    unite (id_unite) {
        id_unite -> Uuid,
        designation -> Text,
    }
}

diesel::joinable!(chantier -> clients (client));
diesel::joinable!(chantier -> finition (type_finition_id));
diesel::joinable!(chantier -> type_chantier (type_chantier_id));
diesel::joinable!(devis -> materiels (materiel_id));
diesel::joinable!(devis -> type_chantier (type_chantier_id));
diesel::joinable!(materiels -> unite (unite_id));

diesel::allow_tables_to_appear_in_same_query!(
    chantier,
    clients,
    devis,
    finition,
    materiels,
    type_chantier,
    unite,
);
