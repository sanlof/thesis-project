CREATE DATABASE police_db;

\c police_db;

CREATE TABLE cases (
    id SERIAL PRIMARY KEY,
    case_number VARCHAR(50) UNIQUE NOT NULL,
    status VARCHAR(20) NOT NULL,
    description TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE suspects (
    id SERIAL PRIMARY KEY,
    case_id INTEGER REFERENCES cases(id),
    name VARCHAR(100) NOT NULL,
    personal_id VARCHAR(20),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO cases (case_number, status, description) VALUES 
('P-2024-001', 'active', 'Theft investigation'),
('P-2024-002', 'closed', 'Traffic violation');