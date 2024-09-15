-- init.sql
CREATE TABLE samples (
	id SERIAL PRIMARY KEY,
	sample_type VARCHAR(100),
	analyst VARCHAR(100)
);
INSERT INTO samples (sample_type, analyst) VALUES ('sample_type1', 'Analyst One'), ('sample_type1', 'Analyst Two'), ('sample_type2', 'Analyst Three'), ('sample_type2', 'Analyst Four');



-- PEOPLE --

-- people_status = 'Active', 'OnLeave', 'Left'
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
	hire_date TIMESTAMPTZ,
	status TEXT,
	emergency_contact TEXT,
	certifications TEXT,
	specializations TEXT,
	picture TEXT
);
INSERT INTO people (employee_id, first_name, last_name, preferred_name, email, phone_number, notes, department, role, hire_date, status, emergency_contact, certifications, specializations, picture) VALUES
('BIO10-0001', 'Gene', 'Splicer', 'Gene', 'gene.splicer@biolab.com', '555-0100', 'Always on the cutting edge.', 'Genetics', 'Senior Scientist', '2020-01-15', 'Active', 'Dr. Helix (555-1111)', 'PhD in Genetics', 'DNA recombination', 'gene_splicer.jpg'),
('BIO10-0002', 'Elle', 'Ment', 'Elle', 'elle.ment@biolab.com', '555-0101', 'Her experiments always have great chemistry.', 'Chemistry', 'Lab Assistant', '2021-06-01', 'Active', 'Sam Ment (555-2222)', 'BSc in Chemistry', 'Chemical analysis', 'elle_ment.jpg'),
('BIO10-0003', 'Adam', 'Zyme', 'Adam', 'adam.zyme@biolab.com', '555-0102', 'Enzymes are his catalyst.', 'Biochemistry', 'Research Scientist', '2019-09-10', 'Active', 'Eve Zyme (555-3333)', 'PhD in Biochemistry', 'Enzymology', 'adam_zyme.jpg'),
('BIO10-0004', 'Ann', 'Tibody', 'Ann', 'ann.tibody@biolab.com', '555-0103', 'Her work is highly defensive.', 'Immunology', 'Lab Technician', '2018-05-20', 'Active', 'Ian Tibody (555-4444)', 'BSc in Biology', 'Antibody production', 'ann_tibody.jpg'),
('BIO10-0005', 'Phil', 'Ter', 'Phil', 'phil.ter@biolab.com', '555-0104', 'Filtering out the impurities.', 'Microbiology', 'Lab Assistant', '2022-02-14', 'Active', 'Sue Ter (555-5555)', 'BSc in Microbiology', 'Filtration techniques', 'phil_ter.jpg'),
('BIO10-0006', 'Ben', 'Doyle', 'Ben', 'ben.doyle@biolab.com', '555-0105', 'Digging deep into research.', 'Geology', 'Research Scientist', '2017-11-01', 'Active', 'Maggie Doyle (555-6666)', 'PhD in Geology', 'Soil analysis', 'ben_doyle.jpg'),
('BIO10-0007', 'Ari', 'Ous', 'Ari', 'ari.ous@biolab.com', '555-0106', 'Her ideas are contagious.', 'Virology', 'Senior Scientist', '2016-07-30', 'Active', 'Val Ous (555-7777)', 'PhD in Virology', 'Virus research', 'ari_ous.jpg'),
('BIO10-0008', 'Polly', 'Merase', 'Polly', 'polly.merase@biolab.com', '555-0107', 'She never stops copying.', 'Molecular Biology', 'Lab Technician', '2021-03-12', 'Active', 'Ruth Merase (555-8888)', 'BSc in Molecular Biology', 'PCR techniques', 'polly_merase.jpg'),
('BIO10-0009', 'Dee', 'N.A.', 'Dee', 'dee.na@biolab.com', '555-0108', 'At the core of every discovery.', 'Genomics', 'Research Scientist', '2018-08-24', 'Active', 'Jay N.A. (555-9999)', 'PhD in Genomics', 'Genome sequencing', 'dee_na.jpg'),
('BIO10-0010', 'Helix', 'Turner', 'Helix', 'helix.turner@biolab.com', '555-0109', 'Twists and turns in his research.', 'Structural Biology', 'Lab Assistant', '2020-12-05', 'Active', 'Felix Turner (555-1010)', 'BSc in Structural Biology', 'Protein folding', 'helix_turner.jpg'),
('BIO10-0011', 'Max', 'A Million', 'Max', 'max.a.million@biolab.com', '555-0110', 'Always reaching for the maximum.', 'Data Analysis', 'Data Scientist', '2019-04-01', 'Active', 'Mina Million (555-1111)', 'MSc in Data Science', 'Big data analytics', 'max_a_million.jpg'),
('BIO10-0012', 'Eve', 'Olution', 'Eve', 'eve.olution@biolab.com', '555-0111', 'Constantly adapting.', 'Evolutionary Biology', 'Senior Scientist', '2015-10-10', 'Active', 'Adam Olution (555-1212)', 'PhD in Evolutionary Biology', 'Species adaptation', 'eve_olution.jpg'),
('BIO10-0013', 'Michael', 'Shipley', 'Mic', 'mic@biolab.com', '555-25372', 'Calm but busy', 'Operations', 'Research Operations', '2011-01-12', 'Active', 'Briar (555-836417)', 'PhD in Patience', 'Species of Dune', 'mic.jpg'),
('BIO10-0014', 'Dominik', 'Wilkowski', 'Dom', 'dom.wilko@biolab.com', '555-666', 'Total nerd.', 'Computer science', 'Senior Dork', '2023-12-24', 'Active', 'Belle (555-11101)', 'Dropout', 'Compiling things', 'dom.jpg');



-- EQUIPMENT --

-- equipment_type = 'Flask', 'Vessel', 'IncubationCabinet'
-- status = 'Working', 'NeedsCleaning', 'Preparation', 'Sterilization', 'Broken', 'OutOfCommission'
CREATE TABLE equipment (
	id SERIAL PRIMARY KEY,
	equipment_type TEXT NOT NULL,
	qrcode TEXT UNIQUE,
	create_date TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
	name TEXT NOT NULL,
	status TEXT NOT NULL,
	manufacturer TEXT,
	purchase_date TIMESTAMPTZ,
	vendor TEXT,
	cost_in_cent INT,
	warranty_expiration_date TIMESTAMPTZ,
	location TEXT,
	notes TEXT
);
INSERT INTO equipment (equipment_type, qrcode, create_date, name, status, manufacturer, purchase_date, vendor, cost_in_cent, warranty_expiration_date, location, notes) VALUES
('Flask','equipment/qr_00001_F.svg','2022-09-18T12:45:59.324310806Z','Stevens Flask','Working','Flasktastic Labs','2022-09-19T12:45:59.324310806Z','SupplySidekick',50000,'2025-01-15T12:45:59.324310806Z','Back Row D, Column 27','This one is a working horse!'),
('Vessel','equipment/qr_00002_V.svg','2024-08-15T12:45:59.324310806Z','Britta','NeedsCleaning','VesselForge','2022-06-10T12:45:59.324310806Z','ScienceStash',75000,'2024-06-10T12:45:59.324310806Z','In the cafeterias fridge','Has a crack on the top left'),
('Flask','equipment/qr_00003_F.svg','2023-01-12T12:45:59.324310806Z','Flasky McFlaskface','Sterilization','BeakerMakers','2022-12-01T12:45:59.324310806Z','LabGear Galore',62050,'2026-01-12T12:45:59.324310806Z','Top Shelf, Aisle 5','Always keeps it clean!'),
('Vessel','equipment/qr_00004_V.svg','2023-05-20T12:45:59.324310806Z','Vessela','Broken','VesselMakers Inc.','2021-07-15T12:45:59.324310806Z','SupplySidekick',82000,'2023-11-20T12:45:59.324310806Z','Storage Room B, Shelf 3','Handle with care... or not.'),
('IncubationCabinet','equipment/qr_00005_I.svg','2022-11-05T12:45:59.324310806Z','Flasknado','NeedsCleaning','FlaskForge','2022-10-25T12:45:59.324310806Z','ScienceStash',58099,'2025-10-05T12:45:59.324310806Z','Chemical Lab, Table 4','Whips up a storm in experiments!'),
('Vessel','equipment/qr_00006_V.svg','2024-03-30T12:45:59.324310806Z','Vesselina','OutOfCommission','VesselVentures','2023-04-18T12:45:59.324310806Z','LabGear Galore',99000,'2024-03-30T12:45:59.324310806Z','Main Lab, Counter 12','All sailed out.'),
('IncubationCabinet','equipment/qr_00007_I.svg','2024-02-10T12:45:59.324310806Z','Sir Mix-a-Lot','Working','Flasky Foundry','2023-11-11T12:45:59.324310806Z','SupplySidekick',73000,'2027-02-10T12:45:59.324310806Z','Mixology Lab, Station 8','Holds mixes like a pro!'),
('IncubationCabinet','equipment/qr_00008_I.svg','2023-07-22T12:45:59.324310806Z','Captain Contain','Preparation','Containment Creations','2022-08-02T12:45:59.324310806Z','ScienceStash',67000,'2026-07-22T12:45:59.324310806Z','Secure Storage, Area 3','A true captain of containment.'),
('Flask','equipment/qr_00009_F.svg','2023-09-09T12:45:59.324310806Z','Dr. Flaskenstein','Working','Mad Flask Labs','2023-05-19T12:45:59.324310806Z','LabGear Galore',52069,'2025-09-09T12:45:59.324310806Z','Experiment Zone, Bay 2','Brings experiments to life!'),
('Vessel','equipment/qr_00010_V.svg','2024-04-15T12:45:59.324310806Z','Vesselocity','Sterilization','Vessel Velocity Inc.','2023-07-01T12:45:59.324310806Z','SupplySidekick',84500,'2026-04-15T12:45:59.324310806Z','High-Speed Lab, Corner 9','Speedy and sleek, always on the go!'),
('Flask','equipment/qr_00011_F.svg','2024-01-18T12:45:59.324310806Z','Flask-in-a-Box','Working','Boxed Labs','2022-12-15T12:45:59.324310806Z','ScienceStash',54000,'2027-01-18T12:45:59.324310806Z','Storage Unit 12, Shelf 4','Comes with a twist!'),
('IncubationCabinet','equipment/qr_00012_I.svg','2023-10-25T12:45:59.324310806Z','Vessel of Wonder','NeedsCleaning','WonderVessels','2022-03-11T12:45:59.324310806Z','LabGear Galore',91010,'2024-10-25T12:45:59.324310806Z','Mystery Lab, Area 7','A cabinet full of surprises!');

CREATE TABLE equipment_notes (
	id SERIAL PRIMARY KEY,
	equipment INT NOT NULL REFERENCES equipment(id) ON DELETE CASCADE,
	create_date TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
	person INT NOT NULL REFERENCES people(id),
	notes TEXT NOT NULL,
	media1 TEXT,
	media2 TEXT,
	media3 TEXT,
	media4 TEXT,
	media5 TEXT,
	media6 TEXT,
	media7 TEXT,
	media8 TEXT,
	media9 TEXT,
	media10 TEXT
);
CREATE INDEX equipment_notes_equipment ON equipment_notes (equipment);
CREATE INDEX equipment_notes_person ON equipment_notes (person);
INSERT INTO equipment_notes (equipment, person, notes) VALUES
(1, 2, 'The thing still refuses to make coffee.'),
(1, 3, 'Found a sandwich from 2019 inside the item.'),
(1, 1, 'Do not press the red button unless you enjoy alarms.'),
(1, 2, 'Equipment refuses to operate until it gets a motivational speech.'),
(3, 4, 'Dust levels exceeded recommended safe limits—again.'),
(3, 2, 'Do not challenge the machine to a staring contest—you will lose.'),
(3, 5, 'The humming sound is normal. The screaming is not.'),
(4, 1, 'If this equipment asks for a raise, politely decline.'),
(4, 3, 'All systems nominal, except for the blinking light of mystery.'),
(4, 4, 'Removed what seemed to be a nest of paperclips.'),
(6, 2, 'Machine insists it is not broken, just differently functional.'),
(6, 5, 'The machine may or may not grant wishes when turned off and on again.'),
(6, 1, 'Managed to remove the sticker saying "Kick Me".'),
(7, 3, 'Do not taunt the equipment. It seems to understand sarcasm.'),
(7, 2, 'Found no issues except a mild attitude problem.'),
(7, 2, 'Equipment demands to be addressed as "Your Highness."'),
(7, 4, 'Machine is now 90% less sticky.'),
(8, 5, 'Confirmed the equipment does not appreciate knock-knock jokes.'),
(8, 2, 'It believes we sholuld use the Imperial system. Expect slow performance.'),
(8, 1, 'If the equipment starts glowing, evacuate the building.'),
(8, 1, 'Do not feed the equipment after midnight. You know the rules.'),
(9, 3, 'The self-destruct sequence is NOT a feature to be tested.'),
(9, 4, 'Found extra parts lying around, but everything seems to work.'),
(9, 5, 'Removed evidence of late-night snack sessions.'),
(10, 1, 'The machine is in denial about its age.'),
(10, 2, 'If the equipment starts smoking, its not on fire—its just stressed.'),
(10, 3, 'It responds well to compliments.'),
(11, 4, 'The equipment seems to be communicating in Morse code via blinking lights.'),
(11, 5, 'Discovered the source of the beeping—it was just hungry.'),
(11, 1, 'Polished to a shine; can now see your reflection. Need a shave!'),
(12, 2, 'Machine believes its a toaster. It is not a toaster.'),
(12, 3, 'Do not attempt to fix the machine with a hammer. Again.'),
(12, 4, 'Removed traces of glitter bomb. The prank war continues.'),
(12, 5, 'If found speaking to the equipment, please consult HR. Again'),
(12, 5, 'Do not mention obsolescence; its a sensitive topic.');

-- action_type = 'cleaning', 'sterilization', 'preparation', 'edit'
CREATE TABLE equipment_actions (
	id SERIAL PRIMARY KEY,
	action_type TEXT NOT NULL,
	equipment INT NOT NULL REFERENCES equipment(id) ON DELETE CASCADE,
	create_date TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
	person INT NOT NULL REFERENCES people(id),
	notes TEXT,
	field TEXT,
	old_value TEXT,
	new_value TEXT,
	media1 TEXT,
	media2 TEXT,
	media3 TEXT,
	media4 TEXT,
	media5 TEXT,
	media6 TEXT,
	media7 TEXT,
	media8 TEXT,
	media9 TEXT,
	media10 TEXT
);
CREATE INDEX equipment_actions_equipment ON equipment_actions (equipment);
CREATE INDEX equipment_actions_person ON equipment_actions (person);
INSERT INTO equipment_actions (action_type, equipment, person, notes, field, old_value, new_value) VALUES
('cleaning', 1, 3, 'Found a tiny civilization of dust bunnies; negotiations are ongoing.', '', '', ''),
('edit', 1, 5, 'Status changed to "Sentient"; it passed the Turing test.', 'status', 'working', 'broken'),
('edit', 2, 8, 'Moved to the "Noisy Equipment" section; it wouldnt stop humming.', 'location', 'Under desk', 'Shelve 12, Cell 12, Row E'),
('edit', 2, 2, 'Updated manual to include "Do not feed after midnight."', 'notes', '', 'No heat for this one'),
('sterilization', 3, 6, 'Sterilized after it started growing its own cultures.', '', '', ''),
('cleaning', 3, 9, 'Removed mysterious sticky substance; taste test inconclusive.', '', '', ''),
('edit', 4, 1, 'Status changed to "Possessed"; it started making eerie sounds.', 'status', 'NeedsCleaning', 'OutOfComission'),
('edit', 4, 14, 'Edited settings to disable the "Self-Destruct" feature.', 'notes', 'TODO', ''),
('preparation', 5, 7, 'Prepared for the annual "Equipment Talent Show".', '', '', ''),
('sterilization', 5, 12, 'Sterilized after it didnt want to join the bacteria union.', '', '', ''),
('edit', 6, 4, 'Moved to storage; it needs some "alone time".', 'location', 'Shelf Top Floor', 'Basement'),
('cleaning', 7, 11, 'Cleaned the "Error 404: Cleanliness Not Found".', '', '', ''),
('edit', 7, 2, 'Status changed to "Mysteriously Wet"; source of moisture unknown.', 'status', 'NeedsCleaning', 'Working'),
('edit', 7, 13, 'Updated firmware to version 1.21 Flask.', 'notes', 'Reminder: to clean this', ''),
('preparation', 8, 5, 'Prepared for interdimensional travel; safety not guaranteed.', '', '', ''),
('sterilization', 9, 10, 'Sterilized after an incident with a radioactive burrito.', '', '', ''),
('edit', 9, 3, 'Adjusted settings to "Do Not Disturb"; its meditating.', 'status', 'Working', 'NeedsCleaning'),
('edit', 10, 6, 'Relocated to Lab 42; because its the answer to everything.', 'location', 'Shelf 41', 'Shelf 42'),
('edit', 11, 8, 'Status changed to "Invisible"; cant find it anywhere.', 'location', 'nowhere', 'Shelf 7'),
('cleaning', 11, 1, 'Attempted to clean; ended up in a philosophical debate.', '', '', ''),
('edit', 12, 14, 'Edited user manual to include "Beware of the Leopard".', 'notes', 'Contains Leopard', 'Contains Leptospira'),
('preparation', 12, 7, 'Prepared for upgrade; hoping it doesnt gain self-awareness.', '', '', ''),
('cleaning', 12, 9, 'Cleaned glitter residue; who had a party without inviting me?', '', '', '');



-- CULTURE

-- culture_contamination_status = 'Clean', 'Xenic', 'Monoxenic', 'Axenic', 'Contaminated', 'ParentContaminated', 'CleanWasContaminated'
CREATE TABLE culture (
	id SERIAL PRIMARY KEY,
	qrcode TEXT NOT NULL UNIQUE,
	create_date TIMESTAMPTZ DEFAULT CURRENT_DATE,
	create_by INT REFERENCES people(id),
	name TEXT NOT NULL,
	parent INT NOT NULL REFERENCES culture(id),
	culture_method TEXT,
	species TEXT,
	genus TEXT,
	location TEXT,
	storage_conditions TEXT,
	equipment INT REFERENCES equipment(id),
	growth_medium TEXT,
	contamination_status TEXT,
	notes TEXT
);
