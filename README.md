# Thesis Project - Secure Data Sharing Between IT Systems

## Prerequisites

- Rust (https://rustup.rs/)
- Node.js 18+ (https://nodejs.org/)
- PostgreSQL 14+ (https://www.postgresql.org/)

## Setup Instructions

### 1. Database Setup

```bash
# Start PostgreSQL and run the schema files
psql -U postgres -f shared/database-schemas/police-schema.sql
psql -U postgres -f shared/database-schemas/hospital-schema.sql
```

### 2. Backend Setup

```bash
# Police system
cd backend/police-system
cargo run

# Hospital system (in new terminal)
cd backend/hospital-system
cargo run
```

### 3. Frontend Setup

```bash
# Police UI
cd frontend/police-ui
npm install
npm start

# Hospital UI (in new terminal)
cd frontend/hospital-ui
npm install
npm start
```

## Access Points

- Police UI: http://localhost:3000
- Hospital UI: http://localhost:3001
- Police API: http://localhost:8000
- Hospital API: http://localhost:8001

## Project Structure

See folder structure in the ARCHITECTURE.md documentation.
