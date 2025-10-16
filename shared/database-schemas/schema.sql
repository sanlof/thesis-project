CREATE DATABASE police_db;
CREATE DATABASE hospital_db;

\c hospital_db

CREATE EXTENSION IF NOT EXISTS postgres_fdw;

CREATE TABLE patients (
    id SERIAL PRIMARY KEY,
    full_name TEXT,
    personal_id TEXT UNIQUE,
    flag BOOLEAN
);

\c police_db

CREATE EXTENSION IF NOT EXISTS postgres_fdw;

CREATE TABLE individuals (
    id SERIAL PRIMARY KEY,
    full_name TEXT,
    personal_id TEXT UNIQUE,
    flag BOOLEAN
);

CREATE SERVER hospital_server
    FOREIGN DATA WRAPPER postgres_fdw
    OPTIONS (dbname 'hospital_db', host 'localhost');

CREATE USER MAPPING FOR CURRENT_USER
    SERVER hospital_server
    OPTIONS (user 'postgres', password '');

IMPORT FOREIGN SCHEMA public
    LIMIT TO (patients)
    FROM SERVER hospital_server
    INTO public;

CREATE OR REPLACE FUNCTION sync_flag_to_hospital() RETURNS TRIGGER AS $$
BEGIN
    UPDATE patients SET flag = NEW.flag WHERE personal_id = NEW.personal_id;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_sync_flag
AFTER UPDATE OF flag ON individuals
FOR EACH ROW
EXECUTE FUNCTION sync_flag_to_hospital();