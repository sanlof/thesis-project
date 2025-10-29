-- Seed police database
--\c police_db;

CREATE TABLE IF NOT EXISTS RAC_suspects (
    id SERIAL PRIMARY KEY,
    full_name TEXT NOT NULL,
    personal_id TEXT UNIQUE NOT NULL,
    flag BOOLEAN DEFAULT FALSE
);

INSERT INTO RAC_suspects (full_name, personal_id, flag) VALUES
    ('Erik Andersson',    '19850312-2398', FALSE),
    ('Anna Karlsson',     '19900204-1457', TRUE),
    ('Johan Lindström',   '19781123-5634', FALSE),
    ('Maria Svensson',    '19891215-0912', TRUE),
    ('Lars Johansson',    '19670630-8841', FALSE),
    ('Emma Nilsson',      '19950419-3325', TRUE),
    ('Oskar Berg',        '19801005-7420', FALSE),
    ('Elin Eriksson',     '20010122-2183', TRUE),
    ('Simon Nyberg',      '19930808-4417', TRUE),
    ('Carina Dahl',       '19870527-6675', FALSE);


-- Seed hospital database
--\c hospital_db;

CREATE TABLE IF NOT EXISTS RAC_patients (
    id SERIAL PRIMARY KEY,
    full_name TEXT NOT NULL,
    personal_id TEXT UNIQUE NOT NULL,
    flag BOOLEAN DEFAULT FALSE
);

INSERT INTO RAC_patients (full_name, personal_id, flag) VALUES
    ('Erik Andersson',    '19850312-2398', FALSE),
    ('Anna Karlsson',     '19900204-1457', TRUE),
    ('Johan Lindström',   '19781123-5634', FALSE),
    ('Maria Svensson',    '19891215-0912', TRUE),
    ('Lars Johansson',    '19670630-8841', FALSE),
    ('Emma Nilsson',      '19950419-3325', TRUE),
    ('Oskar Berg',        '19801005-7420', FALSE),
    ('Elin Eriksson',     '20010122-2183', TRUE);