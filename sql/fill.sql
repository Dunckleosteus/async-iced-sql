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
INSERT INTO assise_material (nom, emissions) VALUES ('BI250', 6.53E-02), ('BMP 3.5%', 7.09E-02), ('BMP 6%', 7.46E-02), 
-- grave emulsion
('GE', 0.01846),
('GNT', 0.00268), -- Que des graviers
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
INSERT INTO structures (name) VALUES ('GNT2'), ('GE sur GNT'), ('Sol traité'), ('retraitement');
-- inserting values into traffic epaisseur 
INSERT INTO traffic_epaisseurs (traffic_id, struct_id, epaisseur_roulement, mat_roulement_id,  epaisseur_assise2, mat_assise2_id, epaisseur_assise, mat_assise_id) VALUES 
	-- traffic_id, struct_id, epaisseur_roulement, mat_roulement_id, epaisseur_assise, mat_assise_id, epaisseur_assise2, mat_assise2_id
	(	
		(SELECT id FROM traffic WHERE nom='T5'), -- traffic_id
		(SELECT id FROM structures WHERE name = 'GE sur GNT'), -- struct_id
		1, -- epaisseur de la couche de la couche de roulement 
		(SELECT id FROM roulement_material WHERE nom='Enduit'), -- mat_roulement_id
		8, -- epaisseur assise 2
		(SELECT id FROM assise_material WHERE nom='GE'),
		38, -- epaisseur assise 
		(SELECT id FROM assise_material WHERE nom='GNT') -- materiau assise
	);

INSERT INTO traffic_epaisseurs (traffic_id, struct_id, epaisseur_roulement, mat_roulement_id,  epaisseur_assise2, mat_assise2_id, epaisseur_assise, mat_assise_id) VALUES 
	-- traffic_id, struct_id, epaisseur_roulement, mat_roulement_id, epaisseur_assise, mat_assise_id, epaisseur_assise2, mat_assise2_id
	(	
		(SELECT id FROM traffic WHERE nom='T4'), -- traffic_id
		(SELECT id FROM structures WHERE name = 'GE sur GNT'), -- struct_id
		1, -- epaisseur de la couche de la couche de roulement 
		(SELECT id FROM roulement_material WHERE nom='Enduit'), -- mat_roulement_id
		8, -- epaisseur assise 2
		(SELECT id FROM assise_material WHERE nom='GE'),
		38, -- epaisseur assise 
		(SELECT id FROM assise_material WHERE nom='GNT') -- materiau assise
	);

INSERT INTO traffic_epaisseurs (traffic_id, struct_id, epaisseur_roulement, mat_roulement_id,  epaisseur_assise2, mat_assise2_id, epaisseur_assise, mat_assise_id) VALUES 
	-- traffic_id, struct_id, epaisseur_roulement, mat_roulement_id, epaisseur_assise, mat_assise_id, epaisseur_assise2, mat_assise2_id
	(	
		(SELECT id FROM traffic WHERE nom='T3'), -- traffic_id
		(SELECT id FROM structures WHERE name = 'GE sur GNT'), -- struct_id
		1, -- epaisseur de la couche de la couche de roulement 
		(SELECT id FROM roulement_material WHERE nom='Enduit'), -- mat_roulement_id
		8, -- epaisseur assise 2
		(SELECT id FROM assise_material WHERE nom='GE'),
		38, -- epaisseur assise 
		(SELECT id FROM assise_material WHERE nom='GNT') -- materiau assise
	);

INSERT INTO traffic_epaisseurs (traffic_id, struct_id, epaisseur_roulement, mat_roulement_id,  epaisseur_assise2, mat_assise2_id, epaisseur_assise, mat_assise_id) VALUES 
	-- traffic_id, struct_id, epaisseur_roulement, mat_roulement_id, epaisseur_assise, mat_assise_id, epaisseur_assise2, mat_assise2_id
	(	
		(SELECT id FROM traffic WHERE nom='T3+'), -- traffic_id
		(SELECT id FROM structures WHERE name = 'GE sur GNT'), -- struct_id
		1, -- epaisseur de la couche de la couche de roulement 
		(SELECT id FROM roulement_material WHERE nom='Enduit'), -- mat_roulement_id
		8, -- epaisseur assise 2
		(SELECT id FROM assise_material WHERE nom='GE'),
		38, -- epaisseur assise 
		(SELECT id FROM assise_material WHERE nom='GNT') -- materiau assise
	);
	-- Structures GNT2
INSERT INTO traffic_epaisseurs (traffic_id, struct_id, epaisseur_roulement, mat_roulement_id,  epaisseur_assise2, mat_assise2_id, epaisseur_assise, mat_assise_id) VALUES 
	-- traffic_id, struct_id, epaisseur_roulement, mat_roulement_id, epaisseur_assise, mat_assise_id, epaisseur_assise2, mat_assise2_id
	(	
		(SELECT id FROM traffic WHERE nom='T5'), -- traffic_id
		(SELECT id FROM structures WHERE name = 'GNT2'), -- struct_id
		1, -- epaisseur de la couche de la couche de roulement 
		(SELECT id FROM roulement_material WHERE nom='Enduit'), -- mat_roulement_id
		NULL, -- epaisseur assise 2
		(SELECT id FROM assise_material WHERE nom='Aucun'),
		45, -- epaisseur assise 
		(SELECT id FROM assise_material WHERE nom='GNT') -- materiau assise
	);
INSERT INTO traffic_epaisseurs (traffic_id, struct_id, epaisseur_roulement, mat_roulement_id,  epaisseur_assise2, mat_assise2_id, epaisseur_assise, mat_assise_id) VALUES 
	-- traffic_id, struct_id, epaisseur_roulement, mat_roulement_id, epaisseur_assise, mat_assise_id, epaisseur_assise2, mat_assise2_id
	(	
		(SELECT id FROM traffic WHERE nom='T4'), -- traffic_id
		(SELECT id FROM structures WHERE name = 'GNT2'), -- struct_id
		1, -- epaisseur de la couche de la couche de roulement 
		(SELECT id FROM roulement_material WHERE nom='Enduit'), -- mat_roulement_id
		NULL, -- epaisseur assise 2
		(SELECT id FROM assise_material WHERE nom='Aucun'),
		45, -- epaisseur assise 
		(SELECT id FROM assise_material WHERE nom='GNT') -- materiau assise
	);
INSERT INTO traffic_epaisseurs (traffic_id, struct_id, epaisseur_roulement, mat_roulement_id,  epaisseur_assise2, mat_assise2_id, epaisseur_assise, mat_assise_id) VALUES 
	-- traffic_id, struct_id, epaisseur_roulement, mat_roulement_id, epaisseur_assise, mat_assise_id, epaisseur_assise2, mat_assise2_id
	(	
		(SELECT id FROM traffic WHERE nom='T3'), -- traffic_id
		(SELECT id FROM structures WHERE name = 'GNT2'), -- struct_id
		1, -- epaisseur de la couche de la couche de roulement 
		(SELECT id FROM roulement_material WHERE nom='Enduit'), -- mat_roulement_id
		NULL, -- epaisseur assise 2
		(SELECT id FROM assise_material WHERE nom='Aucun'),
		45, -- epaisseur assise 
		(SELECT id FROM assise_material WHERE nom='GNT') -- materiau assise
	);
INSERT INTO traffic_epaisseurs (traffic_id, struct_id, epaisseur_roulement, mat_roulement_id,  epaisseur_assise2, mat_assise2_id, epaisseur_assise, mat_assise_id) VALUES 
	-- traffic_id, struct_id, epaisseur_roulement, mat_roulement_id, epaisseur_assise, mat_assise_id, epaisseur_assise2, mat_assise2_id
	(	
		(SELECT id FROM traffic WHERE nom='T3+'), -- traffic_id
		(SELECT id FROM structures WHERE name = 'GNT2'), -- struct_id
		1, -- epaisseur de la couche de la couche de roulement 
		(SELECT id FROM roulement_material WHERE nom='Enduit'), -- mat_roulement_id
		NULL, -- epaisseur assise 2
		(SELECT id FROM assise_material WHERE nom='Aucun'),
		45, -- epaisseur assise 
		(SELECT id FROM assise_material WHERE nom='GNT') -- materiau assise
	);
-- Sols traités
INSERT INTO traffic_epaisseurs (traffic_id, struct_id, epaisseur_roulement, mat_roulement_id,  epaisseur_assise2, mat_assise2_id, epaisseur_assise, mat_assise_id) VALUES 
	-- traffic_id, struct_id, epaisseur_roulement, mat_roulement_id, epaisseur_assise, mat_assise_id, epaisseur_assise2, mat_assise2_id
	(	
		(SELECT id FROM traffic WHERE nom='T5'), -- traffic_id
		(SELECT id FROM structures WHERE name = 'Sol traité'), -- struct_id
		6, -- epaisseur de la couche de la couche de roulement 
		(SELECT id FROM roulement_material WHERE nom='BI250'), -- mat_roulement_id
		NULL, -- epaisseur assise 2
		(SELECT id FROM assise_material WHERE nom='Aucun'),
		47, -- epaisseur assise 
		-- TODO: the user may have to be able to chose the liant type
		(SELECT id FROM assise_material WHERE nom='LHR:clinker 70%') -- materiau assise
	);
INSERT INTO traffic_epaisseurs (traffic_id, struct_id, epaisseur_roulement, mat_roulement_id,  epaisseur_assise2, mat_assise2_id, epaisseur_assise, mat_assise_id) VALUES 
	-- traffic_id, struct_id, epaisseur_roulement, mat_roulement_id, epaisseur_assise, mat_assise_id, epaisseur_assise2, mat_assise2_id
	(	
		(SELECT id FROM traffic WHERE nom='T4'), -- traffic_id
		(SELECT id FROM structures WHERE name = 'Sol traité'), -- struct_id
		6, -- epaisseur de la couche de la couche de roulement 
		(SELECT id FROM roulement_material WHERE nom='BI250'), -- mat_roulement_id
		NULL, -- epaisseur assise 2
		(SELECT id FROM assise_material WHERE nom='Aucun'),
		47, -- epaisseur assise 
		-- TODO: the user may have to be able to chose the liant type
		(SELECT id FROM assise_material WHERE nom='LHR:clinker 70%') -- materiau assise
	);
INSERT INTO traffic_epaisseurs (traffic_id, struct_id, epaisseur_roulement, mat_roulement_id,  epaisseur_assise2, mat_assise2_id, epaisseur_assise, mat_assise_id) VALUES 
	-- traffic_id, struct_id, epaisseur_roulement, mat_roulement_id, epaisseur_assise, mat_assise_id, epaisseur_assise2, mat_assise2_id
	(	
		(SELECT id FROM traffic WHERE nom='T3'), -- traffic_id
		(SELECT id FROM structures WHERE name = 'Sol traité'), -- struct_id
		6, -- epaisseur de la couche de la couche de roulement 
		(SELECT id FROM roulement_material WHERE nom='BI250'), -- mat_roulement_id
		NULL, -- epaisseur assise 2
		(SELECT id FROM assise_material WHERE nom='Aucun'),
		47, -- epaisseur assise 
		-- TODO: the user may have to be able to chose the liant type
		(SELECT id FROM assise_material WHERE nom='LHR:clinker 70%') -- materiau assise
	);
INSERT INTO traffic_epaisseurs (traffic_id, struct_id, epaisseur_roulement, mat_roulement_id,  epaisseur_assise2, mat_assise2_id, epaisseur_assise, mat_assise_id) VALUES 
	-- traffic_id, struct_id, epaisseur_roulement, mat_roulement_id, epaisseur_assise, mat_assise_id, epaisseur_assise2, mat_assise2_id
	(	
		(SELECT id FROM traffic WHERE nom='T3+'), -- traffic_id
		(SELECT id FROM structures WHERE name = 'Sol traité'), -- struct_id
		6, -- epaisseur de la couche de la couche de roulement 
		(SELECT id FROM roulement_material WHERE nom='BI250'), -- mat_roulement_id
		NULL, -- epaisseur assise 2
		(SELECT id FROM assise_material WHERE nom='Aucun'),
		47, -- epaisseur assise 
		-- TODO: the user may have to be able to chose the liant type
		(SELECT id FROM assise_material WHERE nom='LHR:clinker 70%') -- materiau assise
	);
-- Retraitement
INSERT INTO traffic_epaisseurs (traffic_id, struct_id, epaisseur_roulement, mat_roulement_id,  epaisseur_assise2, mat_assise2_id, epaisseur_assise, mat_assise_id) VALUES 
	-- traffic_id, struct_id, epaisseur_roulement, mat_roulement_id, epaisseur_assise, mat_assise_id, epaisseur_assise2, mat_assise2_id
	(	
		(SELECT id FROM traffic WHERE nom='T5'), -- traffic_id
		(SELECT id FROM structures WHERE name = 'retraitement'), -- struct_id
		4, -- epaisseur de la couche de la couche de roulement 
		(SELECT id FROM roulement_material WHERE nom='BI250'), -- mat_roulement_id
		NULL, -- epaisseur assise 2
		(SELECT id FROM assise_material WHERE nom='Aucun'),
		34, -- epaisseur assise 
		(SELECT id FROM assise_material WHERE nom='LHR:clinker 70%') -- materiau assise
	);
INSERT INTO traffic_epaisseurs (traffic_id, struct_id, epaisseur_roulement, mat_roulement_id,  epaisseur_assise2, mat_assise2_id, epaisseur_assise, mat_assise_id) VALUES 
	-- traffic_id, struct_id, epaisseur_roulement, mat_roulement_id, epaisseur_assise, mat_assise_id, epaisseur_assise2, mat_assise2_id
	(	
		(SELECT id FROM traffic WHERE nom='T4'), -- traffic_id
		(SELECT id FROM structures WHERE name = 'retraitement'), -- struct_id
		4, -- epaisseur de la couche de la couche de roulement 
		(SELECT id FROM roulement_material WHERE nom='BI250'), -- mat_roulement_id
		NULL, -- epaisseur assise 2
		(SELECT id FROM assise_material WHERE nom='Aucun'),
		36, -- epaisseur assise 
		(SELECT id FROM assise_material WHERE nom='LHR:clinker 70%') -- materiau assise
	);
INSERT INTO traffic_epaisseurs (traffic_id, struct_id, epaisseur_roulement, mat_roulement_id,  epaisseur_assise2, mat_assise2_id, epaisseur_assise, mat_assise_id) VALUES 
	-- traffic_id, struct_id, epaisseur_roulement, mat_roulement_id, epaisseur_assise, mat_assise_id, epaisseur_assise2, mat_assise2_id
	(	
		(SELECT id FROM traffic WHERE nom='T3'), -- traffic_id
		(SELECT id FROM structures WHERE name = 'retraitement'), -- struct_id
		6, -- epaisseur de la couche de la couche de roulement 
		(SELECT id FROM roulement_material WHERE nom='BI250'), -- mat_roulement_id
		NULL, -- epaisseur assise 2
		(SELECT id FROM assise_material WHERE nom='Aucun'),
		36, -- epaisseur assise 
		(SELECT id FROM assise_material WHERE nom='LHR:clinker 70%') -- materiau assise
	);
INSERT INTO traffic_epaisseurs (traffic_id, struct_id, epaisseur_roulement, mat_roulement_id,  epaisseur_assise2, mat_assise2_id, epaisseur_assise, mat_assise_id) VALUES 
	-- traffic_id, struct_id, epaisseur_roulement, mat_roulement_id, epaisseur_assise, mat_assise_id, epaisseur_assise2, mat_assise2_id
	(	
		(SELECT id FROM traffic WHERE nom='T3+'), -- traffic_id
		(SELECT id FROM structures WHERE name = 'retraitement'), -- struct_id
		6, -- epaisseur de la couche de la couche de roulement 
		(SELECT id FROM roulement_material WHERE nom='BI250'), -- mat_roulement_id
		NULL, -- epaisseur assise 2
		(SELECT id FROM assise_material WHERE nom='Aucun'),
		36, -- epaisseur assise 
		(SELECT id FROM assise_material WHERE nom='LHR:clinker 70%') -- materiau assise
	);
