-- init.sql
CREATE TABLE samples (
    id SERIAL PRIMARY KEY,
    sample_type VARCHAR(100),
    analyst VARCHAR(100)
);

INSERT INTO samples (sample_type, analyst) VALUES ('sample_type1', 'Analyst One'), ('sample_type1', 'Analyst Two'), ('sample_type2', 'Analyst Three'), ('sample_type2', 'Analyst Four');