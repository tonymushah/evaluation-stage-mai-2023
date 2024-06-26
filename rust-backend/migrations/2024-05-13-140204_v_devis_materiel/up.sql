-- Your SQL goes here
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