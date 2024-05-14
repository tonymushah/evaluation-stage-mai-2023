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
    description TEXT NOT NULL,
    duree DECIMAL NOT NULL
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

-- Your SQL goes here
CREATE VIEW V_MATERIELS_UNITE AS
SELECT CODE,
	MA.DESIGNATION AS DESIGNATION,
	PRIX_UNITAIRE,
	UNITE_ID,
	U.DESIGNATION AS UNITE
FROM MATERIELS AS MA
JOIN UNITE AS U ON MA.UNITE_ID = U.ID_UNITE;

CREATE VIEW V_DEVIS_MATERIEL AS
SELECT ID_DEVIS,
	TYPE_CHANTIER_ID AS TYPE_CHANTIER,
	MATERIEL_ID,
	MU.DESIGNATION AS MATERIEL,
	QUANTITE,
	PRIX_UNITAIRE,
	UNITE,
	UNITE_ID,
	(QUANTITE * PRIX_UNITAIRE) AS PRIX_TOTAL
FROM DEVIS
JOIN V_MATERIELS_UNITE AS MU ON DEVIS.MATERIEL_ID = MU.CODE;

CREATE VIEW V_CHANTIER_FINITION AS
SELECT ID_CHANTIER,
	CLIENT,
	DATE_DEBUT,
	F.PRIX AS FINITION_PRIX,
	F.DUREE AS FINITION_DUREE,
	F.DESIGNATION AS FINITION,
	ID_FINITION,
	type_chantier_id
FROM CHANTIER AS CH
JOIN FINITION AS F ON CH.TYPE_FINITION_ID = F.ID_FINITION;
