-- Your SQL goes here
create table chantier (
    id_chantier UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    client VARCHAR(25) NOT NULL REFERENCES clients(telephone),
    type_chantier_id UUID NOT NULL REFERENCES type_chantier(id_type_chantier),
    type_finition_id UUID NOT NULL REFERENCES finition(id_finition),
    date_debut DATE
);