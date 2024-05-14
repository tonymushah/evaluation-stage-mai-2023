-- Your SQL goes here
create table type_chantier(
    id_type_chantier UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    nom TEXT NOT NULL,
    description TEXT NOT NULL,
    duree DECIMAL NOT NULL
);