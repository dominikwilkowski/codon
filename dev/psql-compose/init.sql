-- init.sql
CREATE TABLE samples (
    id SERIAL PRIMARY KEY,
    sample_type VARCHAR(100),
    analyst VARCHAR(100)
);
INSERT INTO samples (sample_type, analyst) VALUES ('sample_type1', 'Analyst One'), ('sample_type1', 'Analyst Two'), ('sample_type2', 'Analyst Three'), ('sample_type2', 'Analyst Four');



CREATE TYPE people_status AS ENUM ('Active', 'OnLeave', 'Left');
CREATE TABLE people (
    id SERIAL PRIMARY KEY,
    employee_id TEXT UNIQUE,
    first_name TEXT,
    last_name TEXT,
    preferred_name TEXT NOT NULL,
    email TEXT UNIQUE,
    phone_number TEXT,
    notes TEXT,
    department TEXT,
    role TEXT,
    hire_date DATE,
    status people_status,
    emergency_contact TEXT,
    certifications TEXT,
    specializations TEXT,
    picture TEXT
);
INSERT INTO people (preferred_name, notes, status) VALUES
    ('Michael', 'CNO (chief nerd officer)', 'Active'),
    ('Dom', 'nerd', 'Active'),
    ('Josh', 'jock', 'Active'),
    ('Old mate', 'looser', 'Left');

CREATE TYPE EquipmentTypes AS ENUM ('Flask', 'Vessel', 'IncubationCabinet');
CREATE TYPE EquipmentStatus AS ENUM ('Working', 'NeedsCleaning', 'Preparation', 'Sterilization', 'Broken', 'OutOfCommission');
CREATE TABLE equipment (
    id SERIAL PRIMARY KEY,
    equipment_type EquipmentTypes NOT NULL,
    qrcode TEXT NOT NULL UNIQUE,
    create_date DATE DEFAULT CURRENT_DATE NOT NULL,
    name TEXT NOT NULL,
    status EquipmentStatus NOT NULL,
    manufacturer TEXT,
    purchase_date DATE,
    vendor TEXT,
    cost TEXT,
    warranty_expiration_date DATE,
    location TEXT,
    notes TEXT
);

INSERT INTO equipment (equipment_type, qrcode, create_date, name, status, manufacturer, purchase_date, vendor, cost, warranty_expiration_date, location, notes) VALUES
('Flask','equipment/qr_F_1234.svg','2022-09-18','Stevens Flask','Working','Flasktastic Labs','2022-09-19','SupplySidekick','500.00','2025-01-15','Back Row D, Column 27','This one is a working horse!'),
('Vessel','equipment/qr_V_1234.svg','2024-08-15','Britta','NeedsCleaning','VesselForge','2022-06-10','ScienceStash','750.00','2024-06-10','In the cafeterias fridge','Has a crack on the top left'),
('Flask','equipment/qr_F_5678.svg','2023-01-12','Flasky McFlaskface','Sterilization','BeakerMakers','2022-12-01','LabGear Galore','620.00','2026-01-12','Top Shelf, Aisle 5','Always keeps it clean!'),
('Vessel','equipment/qr_V_5678.svg','2023-05-20','Vessela','Broken','VesselMakers Inc.','2021-07-15','SupplySidekick','820.00','2023-11-20','Storage Room B, Shelf 3','Handle with care... or not.'),
('IncubationCabinet','equipment/qr_I_9012.svg','2022-11-05','Flasknado','NeedsCleaning','FlaskForge','2022-10-25','ScienceStash','580.00','2025-10-05','Chemical Lab, Table 4','Whips up a storm in experiments!'),
('Vessel','equipment/qr_V_9012.svg','2024-03-30','Vesselina','OutOfCommission','VesselVentures','2023-04-18','LabGear Galore','990.00','2024-03-30','Main Lab, Counter 12','All sailed out.'),
('IncubationCabinet','equipment/qr_I_3456.svg','2024-02-10','Sir Mix-a-Lot','Working','Flasky Foundry','2023-11-11','SupplySidekick','730.00','2027-02-10','Mixology Lab, Station 8','Holds mixes like a pro!'),
('IncubationCabinet','equipment/qr_I_6299.svg','2023-07-22','Captain Contain','Preparation','Containment Creations','2022-08-02','ScienceStash','670.00','2026-07-22','Secure Storage, Area 3','A true captain of containment.'),
('Flask','equipment/qr_F_7890.svg','2023-09-09','Dr. Flaskenstein','Working','Mad Flask Labs','2023-05-19','LabGear Galore','520.00','2025-09-09','Experiment Zone, Bay 2','Brings experiments to life!'),
('Vessel','equipment/qr_V_7890.svg','2024-04-15','Vesselocity','Sterilization','Vessel Velocity Inc.','2023-07-01','SupplySidekick','845.00','2026-04-15','High-Speed Lab, Corner 9','Speedy and sleek, always on the go!'),
('Flask','equipment/qr_F_0123.svg','2024-01-18','Flask-in-a-Box','Working','Boxed Labs','2022-12-15','ScienceStash','540.00','2027-01-18','Storage Unit 12, Shelf 4','Comes with a twist!'),
('IncubationCabinet','equipment/qr_I_0123.svg','2023-10-25','Vessel of Wonder','NeedsCleaning','WonderVessels','2022-03-11','LabGear Galore','910.00','2024-10-25','Mystery Lab, Area 7','A cabinet full of surprises!');

CREATE TYPE culture_contamination_status AS ENUM ('Clean', 'Xenic', 'Monoxenic', 'Axenic', 'Contaminated', 'ParentContaminated', 'CleanWasContaminated');
CREATE TABLE culture (
    id SERIAL PRIMARY KEY,
    qrcode TEXT NOT NULL UNIQUE,
    create_date DATE DEFAULT CURRENT_DATE,
    create_by INT REFERENCES people(id),
    name TEXT NOT NULL,
    parent INT, CONSTRAINT fk_parent FOREIGN KEY (parent) REFERENCES culture(id) ON DELETE SET NULL,
    culture_method TEXT,
    species TEXT,
    genus TEXT,
    location TEXT,
    storage_conditions TEXT,
    equipment INT REFERENCES equipment(id),
    growth_medium TEXT,
    contamination_status culture_contamination_status,
    notes TEXT
);
