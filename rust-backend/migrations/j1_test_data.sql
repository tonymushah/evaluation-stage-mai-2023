-- Données de test pour la table clients
INSERT INTO clients (telephone) VALUES
('0123456789'),
('0987654321'),
('1122334455'),
('5544332211'),
('9876543210'),
('1234567890'),
('6789012345'),
('5432167890'),
('0987612345'),
('5678901234');

-- Données de test pour la table unite
INSERT INTO unite (designation) VALUES
('Unité 1'),
('Unité 2'),
('Unité 3'),
('Unité 4'),
('Unité 5'),
('Unité 6'),
('Unité 7'),
('Unité 8'),
('Unité 9'),
('Unité 10');

-- Données de test pour la table finition
INSERT INTO finition (designation, prix, duree, is_standard) VALUES
('Finition 1', 100.50, 5.0, true),
('Finition 2', 75.25, 4.0, false),
('Finition 3', 120.75, 6.5, true),
('Finition 4', 90.00, 3.5, false),
('Finition 5', 150.25, 7.0, true),
('Finition 6', 80.75, 4.5, false),
('Finition 7', 110.00, 6.0, true),
('Finition 8', 95.25, 3.0, false),
('Finition 9', 130.75, 5.5, true),
('Finition 10', 85.00, 4.0, false);

-- Données de test pour la table materiels
INSERT INTO materiels (code, designation, prix_unitaire, unite_id) VALUES
('M001', 'Materiel 1', 50.00, (SELECT id_unite FROM unite WHERE designation = 'Unité 1')),
('M002', 'Materiel 2', 30.00, (SELECT id_unite FROM unite WHERE designation = 'Unité 2')),
('M003', 'Materiel 3', 70.00, (SELECT id_unite FROM unite WHERE designation = 'Unité 3')),
('M004', 'Materiel 4', 40.00, (SELECT id_unite FROM unite WHERE designation = 'Unité 4')),
('M005', 'Materiel 5', 60.00, (SELECT id_unite FROM unite WHERE designation = 'Unité 5')),
('M006', 'Materiel 6', 45.00, (SELECT id_unite FROM unite WHERE designation = 'Unité 6')),
('M007', 'Materiel 7', 55.00, (SELECT id_unite FROM unite WHERE designation = 'Unité 7')),
('M008', 'Materiel 8', 65.00, (SELECT id_unite FROM unite WHERE designation = 'Unité 8')),
('M009', 'Materiel 9', 35.00, (SELECT id_unite FROM unite WHERE designation = 'Unité 9')),
('M010', 'Materiel 10', 75.00, (SELECT id_unite FROM unite WHERE designation = 'Unité 10'));

-- Données de test pour la table type_chantier
INSERT INTO type_chantier (nom, description) VALUES
('Chantier 1', 'Description du chantier 1'),
('Chantier 2', 'Description du chantier 2'),
('Chantier 3', 'Description du chantier 3'),
('Chantier 4', 'Description du chantier 4'),
('Chantier 5', 'Description du chantier 5'),
('Chantier 6', 'Description du chantier 6'),
('Chantier 7', 'Description du chantier 7'),
('Chantier 8', 'Description du chantier 8'),
('Chantier 9', 'Description du chantier 9'),
('Chantier 10', 'Description du chantier 10');

-- Données de test pour la table devis
INSERT INTO devis (type_chantier_id, materiel_id, quantite) VALUES
((SELECT id_type_chantier FROM type_chantier WHERE nom = 'Chantier 1'), 'M001', 10.0),
((SELECT id_type_chantier FROM type_chantier WHERE nom = 'Chantier 2'), 'M002', 15.0),
((SELECT id_type_chantier FROM type_chantier WHERE nom = 'Chantier 3'), 'M003', 20.0),
((SELECT id_type_chantier FROM type_chantier WHERE nom = 'Chantier 4'), 'M004', 25.0),
((SELECT id_type_chantier FROM type_chantier WHERE nom = 'Chantier 5'), 'M005', 30.0),
((SELECT id_type_chantier FROM type_chantier WHERE nom = 'Chantier 6'), 'M006', 35.0),
((SELECT id_type_chantier FROM type_chantier WHERE nom = 'Chantier 7'), 'M007', 40.0),
((SELECT id_type_chantier FROM type_chantier WHERE nom = 'Chantier 8'), 'M008', 45.0),
((SELECT id_type_chantier FROM type_chantier WHERE nom = 'Chantier 9'), 'M009', 50.0),
((SELECT id_type_chantier FROM type_chantier WHERE nom = 'Chantier 10'), 'M010', 55.0);

-- Données de test pour la table chantier
INSERT INTO chantier (client, type_chantier_id, type_finition_id, date_debut) VALUES
('0123456789', (SELECT id_type_chantier FROM type_chantier WHERE nom = 'Chantier 1'), (SELECT id_finition FROM finition WHERE designation = 'Finition 1'), '2024-05-01'),
('0987654321', (SELECT id_type_chantier FROM type_chantier WHERE nom = 'Chantier 2'), (SELECT id_finition FROM finition WHERE designation = 'Finition 2'), '2024-05-02'),
('1122334455', (SELECT id_type_chantier FROM type_chantier WHERE nom = 'Chantier 3'), (SELECT id_finition FROM finition WHERE designation = 'Finition 3'), '2024-05-03'),
('5544332211', (SELECT id_type_chantier FROM type_chantier WHERE nom = 'Chantier 4'), (SELECT id_finition FROM finition WHERE designation = 'Finition 4'), '2024-05-04'),
('9876543210', (SELECT id_type_chantier FROM type_chantier WHERE nom = 'Chantier 5'), (SELECT id_finition FROM finition WHERE designation = 'Finition 5'), '2024-05-05'),
('1234567890', (SELECT id_type_chantier FROM type_chantier WHERE nom = 'Chantier 6'), (SELECT id_finition FROM finition WHERE designation = 'Finition 6'), '2024-05-06'),
('6789012345', (SELECT id_type_chantier FROM type_chantier WHERE nom = 'Chantier 7'), (SELECT id_finition FROM finition WHERE designation = 'Finition 7'), '2024-05-07'),
('5432167890', (SELECT id_type_chantier FROM type_chantier WHERE nom = 'Chantier 8'), (SELECT id_finition FROM finition WHERE designation = 'Finition 8'), '2024-05-08'),
('0987612345', (SELECT id_type_chantier FROM type_chantier WHERE nom = 'Chantier 9'), (SELECT id_finition FROM finition WHERE designation = 'Finition 9'), '2024-05-09'),
('5678901234', (SELECT id_type_chantier FROM type_chantier WHERE nom = 'Chantier 10'), (SELECT id_finition FROM finition WHERE designation = 'Finition 10'), '2024-05-10');
