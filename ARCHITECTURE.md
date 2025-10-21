# Project Structure Overview

```
thesis-project/
├── backend/
│   ├── police-system/                     # Police backend service
│   │   ├── src/
│   │   │   ├── main.rs                    # Application entry point
│   │   │   ├── api/                       # REST API endpoints
│   │   │   │   ├── mod.rs
│   │   │   │   ├── suspects.rs            # Suspect management endpoints
│   │   │   │   └── shared.rs              # Inter-system API endpoints
│   │   │   ├── models/                    # Data structures
│   │   │   │   ├── mod.rs
│   │   │   │   └── suspect.rs
│   │   │   └── database/                  # Database layer
│   │   │       ├── mod.rs
│   │   │       ├── connection.rs          # PostgreSQL connection pool
│   │   │       └── queries.rs             # SQL queries
│   │   ├── Cargo.toml                     # Rust dependencies
│   │   └── .env.example                   # Environment configuration template
│   │
│   └── hospital-system/                   # Hospital backend service
│       ├── src/
│       │   ├── main.rs                    # Application entry point
│       │   ├── api/                       # REST API endpoints
│       │   │   ├── mod.rs
│       │   │   ├── patients.rs            # Patient management endpoints
│       │   │   └── shared.rs              # Inter-system API endpoints
│       │   ├── models/                    # Data structures
│       │   │   ├── mod.rs
│       │   │   └── patient.rs
│       │   └── database/                  # Database layer
│       │       ├── mod.rs
│       │       ├── connection.rs          # PostgreSQL connection pool
│       │       └── queries.rs             # SQL queries
│       ├── Cargo.toml                     # Rust dependencies
│       └── .env.example                   # Environment configuration template
│
├── shared/
│   └── database-schemas/                  # SQL schema definitions
│       ├── schema.sql                     # Database structure and FDW setup
│       └── seed-data.sql                  # Sample data for testing
│
├── docs/                                  # Documentation
│   └── psql-guide.md                      # PostgreSQL setup and usage guide
│
├── .gitignore                             # Git ignore patterns
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
- **Cargo.toml** - Rust dependencies (actix-web, sqlx, tokio, etc.)
- **.env.example** - Template for environment variables (copy to .env)

### **shared/database-schemas/**

SQL files for database setup:

- **schema.sql** - Creates both databases, sets up tables, configures postgres_fdw for cross-database synchronization, and creates triggers
- **seed-data.sql** - Inserts sample Swedish individuals for testing

### **docs/**

- **psql-guide.md** - Comprehensive guide for PostgreSQL setup, database creation, and testing synchronization

## Architecture Overview

```
┌─────────────────────────────────────────┐
│         Police Backend (Rust)           │
│         Port: 8000                      │
│  ┌─────────────────────────────────┐    │
│  │  API Endpoints                  │    │
│  │  - GET/POST/PUT/DELETE suspects │    │
│  │  - Flag management              │    │
│  │  - Hospital data requests       │    │
│  └─────────────────────────────────┘    │
└──────────────┬──────────────────────────┘
               │
               │ postgres_fdw
               │ (Foreign Data Wrapper)
               ▼
    ┌──────────────────────┐
    │   PostgreSQL Setup   │
    ├──────────────────────┤
    │  police_db           │
    │  ├─ suspects table   │
    │  └─ patients (FDW)   │◄───────┐
    │                      │        │
    │  hospital_db         │        │
    │  └─ patients table   │        │
    └──────────────────────┘        │
               ▲                    │
               │                    │
               │              Auto-sync trigger
               │              (flag updates)
┌──────────────┴──────────────────────────┐
│       Hospital Backend (Rust)           │
│         Port: 8001                      │
│  ┌─────────────────────────────────┐    │
│  │  API Endpoints                  │    │
│  │  - GET/POST/PUT/DELETE patients │    │
│  │  - Medical records              │    │
│  │  - Police data requests         │    │
│  └─────────────────────────────────┘    │
└─────────────────────────────────────────┘
```

## Database Architecture

### Cross-Database Communication

The project uses **postgres_fdw** (Foreign Data Wrapper) to enable direct database-to-database communication:

1. **Police database** can directly query the hospital's `patients` table
2. **Automatic flag synchronization** via triggers
3. **No application-level coordination** needed for flag updates

### Tables

**Police Database (police_db):**

```sql
suspects (
    id SERIAL PRIMARY KEY,
    full_name TEXT,
    personal_id TEXT UNIQUE,
    flag BOOLEAN
)
```

**Hospital Database (hospital_db):**

```sql
patients (
    id SERIAL PRIMARY KEY,
    full_name TEXT,
    personal_id TEXT UNIQUE,
    flag BOOLEAN
)
```

### Flag Synchronization

When a suspect is flagged in the police database:

1. A trigger (`trg_sync_flag`) fires automatically
2. The flag is instantly synchronized to matching `personal_id` in hospital database
3. No API calls or external processes needed

Example:

```sql
-- In police_db
UPDATE suspects SET flag = true WHERE personal_id = '19850312-2398';

-- Automatically syncs to hospital_db.patients
-- No additional code required!
```

## Implementation Status

**Completed:**

- ✅ Database schema design
- ✅ Cross-database synchronization setup
- ✅ Sample seed data
- ✅ PostgreSQL configuration guide
- ✅ Project structure

**To Be Implemented:**

- ⏳ Rust backend services (Cargo.toml, all .rs files)
- ⏳ REST API endpoints
- ⏳ Database connection pools
- ⏳ API documentation
- ⏳ Frontend (future phase)

## Technology Stack

**Backend:**

- Rust (latest stable)
- actix-web (web framework)
- sqlx (async PostgreSQL driver)
- tokio (async runtime)
- serde (serialization)

**Database:**

- PostgreSQL 15+
- postgres_fdw extension

**Development Tools:**

- Cargo (Rust package manager)
- VS Code with Rust extensions
- Homebrew (macOS package manager)

## Environment Configuration

Both backend services require a `.env` file (copy from `.env.example`):

**Police System:**

```env
DATABASE_URL=postgresql://postgres@localhost/police_db
SERVER_PORT=8000
HOSPITAL_API_URL=http://localhost:8001
```

**Hospital System:**

```env
DATABASE_URL=postgresql://postgres@localhost/hospital_db
SERVER_PORT=8001
POLICE_API_URL=http://localhost:8000
```

## Getting Started

1. **Install PostgreSQL** (see docs/psql-guide.md)
2. **Create databases:**
   ```bash
   psql -U postgres -f shared/database-schemas/schema.sql
   ```
3. **Seed data:**
   ```bash
   psql -U postgres -f shared/database-schemas/seed-data.sql
   ```
4. **Test synchronization** (see docs/psql-guide.md for test queries)
5. **Implement backend services** (next phase)

## Security Considerations

**Current State (Development):**

- No authentication
- Direct database access
- Local-only connections

**Production Requirements (Future):**

- JWT authentication
- API key validation
- Rate limiting
- Encrypted connections
- Audit logging
- GDPR compliance for personal data

## Notes

- Personal IDs use Swedish format (YYYYMMDD-XXXX)
- Sample data includes 8 shared records + 2 police-only records
- Flag synchronization is unidirectional (police → hospital)
- Frontend will be added in a future phase
