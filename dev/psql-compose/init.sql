-- init.sql
CREATE TABLE samples (
    id SERIAL PRIMARY KEY,
    sample_type VARCHAR(100),
    analyst VARCHAR(100)
);
INSERT INTO samples (sample_type, analyst) VALUES ('sample_type1', 'Analyst One'), ('sample_type1', 'Analyst Two'), ('sample_type2', 'Analyst Three'), ('sample_type2', 'Analyst Four');



CREATE TYPE people_status AS ENUM ('active', 'on_leave', 'left');
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
    ('Michael', 'CNO (chief nerd officer)', 'active'),
    ('Dom', 'nerd', 'active'),
    ('Josh', 'jock', 'active'),
    ('Old mate', 'looser', 'left');

CREATE TYPE equipment_types AS ENUM ('flask', 'vessel', 'incubation_cabinet');
CREATE TYPE equipment_status AS ENUM ('working', 'needs_cleaning', 'preparation', 'sterilization', 'broken', 'out_of_commission');
CREATE TABLE equipment (
    id SERIAL PRIMARY KEY,
    type equipment_types,
    qrcode TEXT NOT NULL UNIQUE,
    create_date DATE DEFAULT CURRENT_DATE,
    name TEXT NOT NULL,
    manufacturer TEXT,
    status equipment_status,
    purchase_date DATE,
    vendor TEXT,
    cost TEXT,
    warranty_expiration_date DATE,
    location TEXT,
    notes TEXT
);

CREATE TYPE culture_contamination_status AS ENUM ('clean', 'xenic', 'monoxenic', 'axenic', 'contaminated', 'parent_contaminated', 'clean_was_contaminated');
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
