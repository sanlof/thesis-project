-- Hospital DB
CREATE DATABASE hospital_db;
-- Connect to hospital_db via psql client

CREATE EXTENSION IF NOT EXISTS postgres_fdw;

CREATE TABLE RAC_patients (
    id SERIAL PRIMARY KEY,
    full_name TEXT,
    personal_id TEXT UNIQUE,
    flag BOOLEAN
);

-- Police DB
CREATE DATABASE police_db;
-- Connect to police_db via psql client

CREATE EXTENSION IF NOT EXISTS postgres_fdw;

CREATE TABLE RAC_suspects (
    id SERIAL PRIMARY KEY,
    full_name TEXT,
    personal_id TEXT UNIQUE,
    flag BOOLEAN
);

-- Configure FDW from police_db to hospital_db
CREATE SERVER hospital_server
    FOREIGN DATA WRAPPER postgres_fdw
    OPTIONS (dbname 'hospital_db', host 'localhost');

CREATE USER MAPPING FOR CURRENT_USER
    SERVER hospital_server
    OPTIONS (user 'postgres', password '');

-- Import RAC_patients table
IMPORT FOREIGN SCHEMA public
    LIMIT TO (RAC_patients)
    FROM SERVER hospital_server
    INTO public;

-- Trigger function to sync flag updates
CREATE OR REPLACE FUNCTION sync_flag_to_hospital() 
RETURNS TRIGGER AS $$
BEGIN
    UPDATE RAC_patients
    SET flag = NEW.flag 
    WHERE personal_id = NEW.personal_id;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Trigger on RAC_suspects
CREATE TRIGGER trg_sync_flag
    AFTER UPDATE OF flag ON RAC_suspects
    FOR EACH ROW
    EXECUTE FUNCTION sync_flag_to_hospital();
