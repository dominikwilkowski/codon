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
