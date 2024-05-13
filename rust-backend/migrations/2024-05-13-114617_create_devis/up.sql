-- Your SQL goes here
create table devis(
    id_devis UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    type_chantier_id UUID NOT NULL REFERENCES type_chantier(id_type_chantier),
    materiel_id varchar(25) NOT NULL REFERENCES materiels(code),
    quantite decimal NOT NULL
);