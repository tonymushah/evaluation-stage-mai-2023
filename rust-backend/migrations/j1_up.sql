create table clients(
    telephone varchar(25) PRIMARY KEY
);

create table unite(
    id_unite UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    designation TEXT NOT NULL
);

create table finition(
    id_finition UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    designation TEXT NOT NULL,
    prix decimal NOT NULL,
    duree decimal NOT NULL,
    is_standard boolean
);

create table materiels (
    code varchar(25) PRIMARY KEY,
    designation text not null,
    prix_unitaire decimal,
    unite_id UUID NOT NULL REFERENCES unite(id_unite)
);

create table type_chantier(
    id_type_chantier UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    nom TEXT NOT NULL,
    description TEXT NOT NULL
);

create table devis(
    id_devis UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    type_chantier_id UUID NOT NULL REFERENCES type_chantier(id_type_chantier),
    materiel_id varchar(25) NOT NULL REFERENCES materiels(code),
    quantite decimal NOT NULL
);

create table chantier (
    id_chantier UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    client VARCHAR(25) NOT NULL REFERENCES clients(telephone),
    type_chantier_id UUID NOT NULL REFERENCES type_chantier(id_type_chantier),
    type_finition_id UUID NOT NULL REFERENCES finition(id_finition),
    date_debut DATE
);

