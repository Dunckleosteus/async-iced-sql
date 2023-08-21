
DROP TABLE IF EXISTS traffic CASCADE; 
CREATE TABLE traffic (
	id SERIAL PRIMARY KEY, 
	nom Varchar(10) UNIQUE,
	traffic Varchar(10)
); 
-- Matériaux couche de roulement
DROP TABLE IF EXISTS roulement_material CASCADE; 
CREATE TABLE roulement_material (
	id SERIAL PRIMARY KEY, 
	nom VARCHAR(40) UNIQUE, 
	emissions NUMERIC
);
-- Matériaux Assise
DROP TABLE IF EXISTS assise_material CASCADE;
CREATE TABLE assise_material(
	id SERIAL PRIMARY KEY, 
	nom VARCHAR(40) UNIQUE, 
	emissions NUMERIC
);	
-- Table calcul emissions	
DROP TABLE IF EXISTS calculs_emissions; 
CREATE TABLE calculs_emissions(
	epaisseur_assise NUMERIC, 
	assise_id INT REFERENCES assise_material(id), 
	epaisseur_roulement NUMERIC, 
	roulement_id INT REFERENCES roulement_material(id)
);
-- structures
DROP TABLE IF EXISTS structures CASCADE; 
CREATE TABLE structures (
	id SERIAL PRIMARY KEY,
	name VARCHAR(30) UNIQUE
);	
-- Table traffic epaisseur
DROP TABLE IF EXISTS traffic_epaisseurs; 
CREATE TABLE traffic_epaisseurs (
	id SERIAL PRIMARY KEY, 
	traffic_id INT REFERENCES traffic(id) NOT NULL,
	struct_id INT REFERENCES structures(id) NOT NULL,
	epaisseur_roulement numeric NOT NULL,
	mat_roulement_id INT REFERENCES roulement_material(id) NOT NULL,
	epaisseur_assise2 numeric,
	mat_assise2_id INT REFERENCES assise_material(id),
	epaisseur_assise numeric NOT NULL, 
	mat_assise_id INT REFERENCES assise_material(id) NOT NULL
	
);