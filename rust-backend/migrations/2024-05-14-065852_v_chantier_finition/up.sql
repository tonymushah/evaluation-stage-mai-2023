-- Your SQL goes here
CREATE VIEW V_CHANTIER_FINITION AS
SELECT ID_CHANTIER,
	CLIENT,
	DATE_DEBUT,
	F.PRIX AS FINITION_PRIX,
	F.DUREE AS FINITION_DUREE,
	F.DESIGNATION AS FINITION,
	ID_FINITION
FROM CHANTIER AS CH
JOIN FINITION AS F ON CH.TYPE_FINITION_ID = F.ID_FINITION;