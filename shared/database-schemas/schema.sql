-- Create databases
CREATE DATABASE police_db;
CREATE DATABASE hospital_db;

-- Set up hospital database
\c hospital_db

CREATE EXTENSION IF NOT EXISTS postgres_fdw;

CREATE TABLE patients (
    id SERIAL PRIMARY KEY,
    full_name TEXT,
    personal_id TEXT UNIQUE,
    flag BOOLEAN
);

-- Set up police database
\c police_db

CREATE EXTENSION IF NOT EXISTS postgres_fdw;

CREATE TABLE suspects (
    id SERIAL PRIMARY KEY,
    full_name TEXT,
    personal_id TEXT UNIQUE,
    flag BOOLEAN
);

-- Configure foreign data wrapper to connect to hospital database
CREATE SERVER hospital_server
    FOREIGN DATA WRAPPER postgres_fdw
    OPTIONS (dbname 'hospital_db', host 'localhost');

CREATE USER MAPPING FOR CURRENT_USER
    SERVER hospital_server
    OPTIONS (user 'postgres', password '');

-- Import the patients table from hospital database
IMPORT FOREIGN SCHEMA public
    LIMIT TO (patients)
    FROM SERVER hospital_server
    INTO public;

-- Function to sync flag updates to hospital database
CREATE OR REPLACE FUNCTION sync_flag_to_hospital() 
RETURNS TRIGGER AS $$
BEGIN
    UPDATE patients 
    SET flag = NEW.flag 
    WHERE personal_id = NEW.personal_id;
    
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Trigger to automatically sync flag changes
CREATE TRIGGER trg_sync_flag
    AFTER UPDATE OF flag ON suspects
    FOR EACH ROW
    EXECUTE FUNCTION sync_flag_to_hospital();