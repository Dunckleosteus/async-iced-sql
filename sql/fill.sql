INSERT INTO traffic (nom, traffic) VALUES ('Texp', 'fort');
INSERT INTO traffic (nom, traffic) VALUES ('TS', 'fort');
INSERT INTO traffic (nom, traffic) VALUES ('T0', 'moyen');
INSERT INTO traffic (nom, traffic) VALUES ('T1', 'moyen');
INSERT INTO traffic (nom, traffic) VALUES ('T2', 'moyen');
INSERT INTO traffic (nom, traffic) VALUES ('T3+', 'faible');
INSERT INTO traffic (nom, traffic) VALUES ('T3', 'faible');
INSERT INTO traffic (nom, traffic) VALUES ('T4', 'faible');
INSERT INTO traffic (nom, traffic) VALUES ('T5', 'faible');
-- adding values to materiaux assise and couche roulement
INSERT INTO roulement_material (nom, emissions) VALUES ('BI250', 7.65E-02), ('BMP 3.5%', 8.35E-02), ('BMP 6%', 8.79E-02), ('Enduit', 0.0109); 
INSERT INTO assise_material (nom, emissions) VALUES ('BI250', 6.53E-02), ('BMP 3.5%', 7.09E-02), ('BMP 6%', 7.46E-02), ('GE', 0.01846),
-- sols traités
('Chaux vive type route', 1.09761), 
('Chaux vive type route taux faible', 1.09761),
('LHR:clinker 70%', 0.54471),
('Ciment CEM I', 0.765),
('Ciment CEM II', 0.628),
('Ciment CEM III', 0.337),
('Aucun', 0);


INSERT INTO calculs_emissions (epaisseur_assise, assise_id, epaisseur_roulement, roulement_id) VALUES (2, 1, 4, 1);
-- partie suivante seulement valide si le traffique est faible
-- prend seulement en compte le traffic de l'utilisateur. 
INSERT INTO structures (name) VALUES ('GNT2'), ('GE sur GNT'), ('Sol traité'), ('retraitement'), ('Sisi Test');
-- Add values to traffic structure table

