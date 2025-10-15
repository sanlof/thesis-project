# Project Structure Overview
```
thesis-project/
├── backend/
│   ├── police-system/                     # Police backend service
│   │   ├── src/
│   │   │   ├── main.rs                    # Application entry point
│   │   │   ├── api/                       # REST API endpoints
│   │   │   │   ├── mod.rs
│   │   │   │   ├── cases.rs               # Case management endpoints
│   │   │   │   ├── suspects.rs            # Suspect management endpoints
│   │   │   │   └── shared.rs              # Inter-system API endpoints
│   │   │   ├── models/                    # Data structures
│   │   │   │   ├── mod.rs
│   │   │   │   ├── case.rs
│   │   │   │   └── suspect.rs
│   │   │   └── database/                  # Database layer
│   │   │       ├── mod.rs
│   │   │       ├── connection.rs          # PostgreSQL connection pool
│   │   │       └── queries.rs             # SQL queries
│   │   ├── Cargo.toml                     # Rust dependencies
│   │   └── .env                           # Environment configuration
│   │
│   └── hospital-system/                   # Hospital backend service
│       ├── src/
│       │   ├── main.rs                    # Application entry point
│       │   ├── api/                       # REST API endpoints
│       │   │   ├── mod.rs
│       │   │   ├── patients.rs            # Patient management endpoints
│       │   │   ├── records.rs             # Medical records endpoints
│       │   │   └── shared.rs              # Inter-system API endpoints
│       │   ├── models/                    # Data structures
│       │   │   ├── mod.rs
│       │   │   ├── patient.rs
│       │   │   └── record.rs
│       │   └── database/                  # Database layer
│       │       ├── mod.rs
│       │       ├── connection.rs          # PostgreSQL connection pool
│       │       └── queries.rs             # SQL queries
│       ├── Cargo.toml                     # Rust dependencies
│       └── .env                           # Environment configuration
│
├── frontend/
│   ├── police-ui/                         # Police web interface
│   │   ├── src/
│   │   │   ├── App.tsx                    # Main React component
│   │   │   ├── index.tsx                  # Application entry point
│   │   │   ├── App.css                    # Styling
│   │   │   ├── components/                # Reusable UI components
│   │   │   │   ├── CaseList.tsx
│   │   │   │   ├── CaseForm.tsx
│   │   │   │   └── DataRequest.tsx        # Request data from hospital
│   │   │   ├── services/                  # API communication layer
│   │   │   │   ├── api.ts                 # Base API configuration
│   │   │   │   ├── cases.ts               # Case-related API calls
│   │   │   │   └── shared.ts              # Inter-system requests
│   │   │   └── types/                     # TypeScript type definitions
│   │   │       ├── case.ts
│   │   │       └── suspect.ts
│   │   ├── public/
│   │   │   └── index.html                 # HTML template
│   │   ├── package.json                   # Node dependencies
│   │   └── tsconfig.json                  # TypeScript configuration
│   │
│   └── hospital-ui/                       # Hospital web interface
│       ├── src/
│       │   ├── App.tsx                    # Main React component
│       │   ├── index.tsx                  # Application entry point
│       │   ├── App.css                    # Styling
│       │   ├── components/                # Reusable UI components
│       │   │   ├── PatientList.tsx
│       │   │   ├── PatientForm.tsx
│       │   │   └── DataRequest.tsx        # Request data from police
│       │   ├── services/                  # API communication layer
│       │   │   ├── api.ts                 # Base API configuration
│       │   │   ├── patients.ts            # Patient-related API calls
│       │   │   └── shared.ts              # Inter-system requests
│       │   └── types/                     # TypeScript type definitions
│       │       ├── patient.ts
│       │       └── record.ts
│       ├── public/
│       │   └── index.html                 # HTML template
│       ├── package.json                   # Node dependencies
│       └── tsconfig.json                  # TypeScript configuration
│
├── shared/
│   ├── database-schemas/                  # SQL schema definitions
│   │   ├── police-schema.sql              # Police database structure
│   │   └── hospital-schema.sql            # Hospital database structure
│   └── api-contracts/                     # API documentation (optional)
│       ├── police-api.md
│       └── hospital-api.md
│
├── docs/                                  # Additional documentation
│   ├── setup-guide.md
│   ├── api-documentation.md
│   └── security-analysis.md
│
├── README.md                              # Project overview and quick start
└── ARCHITECTURE.md                        # This file
```

## Key Folders Explained

### **backend/police-system/** and **backend/hospital-system/**
Each system has its own independent Rust backend with:
- **src/main.rs** - Entry point, starts the web server
- **src/api/** - REST API endpoints for CRUD operations and data sharing
- **src/models/** - Data structures (structs) representing database tables
- **src/database/** - PostgreSQL connection and query logic
- **Cargo.toml** - Rust dependencies (e.g., actix-web, sqlx)
- **.env** - Database connection strings and secrets

### **frontend/police-ui/** and **frontend/hospital-ui/**
Separate React applications for each system:
- **src/App.tsx** - Main React component
- **src/components/** - UI components (forms, tables, buttons)
- **src/services/** - API calls to the backend (fetch/axios)
- **src/types/** - TypeScript interfaces matching backend data
- **package.json** - Node.js dependencies (React, TypeScript, etc.)

### **shared/database-schemas/**
SQL files to initialize both PostgreSQL databases with their tables and relationships.

## Architecture Overview
```
┌─────────────────┐          ┌─────────────────┐
│  Police UI      │          │  Hospital UI    │
│  (React/TS)     │          │  (React/TS)     │
│  Port: 3000     │          │  Port: 3001     │
└────────┬────────┘          └────────┬────────┘
         │                            │
         │ HTTP/REST                  │ HTTP/REST
         │                            │
┌────────▼────────┐          ┌────────▼────────┐
│ Police Backend  │◄────────►│ Hospital Backend│
│ (Rust)          │  Secure  │ (Rust)          │
│ Port: 8000      │   API    │ Port: 8001      │
└────────┬────────┘          └────────┬────────┘
         │                            │
┌────────▼────────┐          ┌────────▼────────┐
│ Police DB       │          │ Hospital DB     │
│ (PostgreSQL)    │          │ (PostgreSQL)    │
└─────────────────┘          └─────────────────┘
```
