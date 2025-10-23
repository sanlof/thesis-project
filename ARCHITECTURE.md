# Project Architecture

A thesis project demonstrating secure data sharing between IT systems using PostgreSQL Foreign Data Wrapper (FDW) for automatic flag synchronization between police and hospital databases.

## Table of Contents

- [Project Overview](#project-overview)
- [Project Structure](#project-structure)
- [Technology Stack](#technology-stack)
- [Architecture Diagram](#architecture-diagram)
- [Backend Architecture](#backend-architecture)
- [Frontend Architecture](#frontend-architecture)
- [Database Architecture](#database-architecture)
- [Implementation Status](#implementation-status)
- [Security Considerations](#security-considerations)

---

## Project Overview

This project demonstrates a proof-of-concept system for secure data sharing between two independent organizations (police and hospital) while maintaining data sovereignty. The key innovation is using PostgreSQL's Foreign Data Wrapper to enable automatic flag synchronization at the database level, eliminating the need for complex API-level coordination.

**Key Features:**

- Independent backend services for police and hospital systems
- Automatic flag synchronization via database triggers
- Cross-system data queries through shared API endpoints
- React frontend for data visualization
- Swedish personal ID format (YYYYMMDD-XXXX) for identity management

---

## Project Structure

```
thesis-project/
├── backend/
│   ├── police-system/                  # Police backend service (Rust/Actix)
│   │   ├── src/
│   │   │   ├── main.rs                # Application entry point
│   │   │   ├── api/                   # REST API endpoints
│   │   │   │   ├── mod.rs
│   │   │   │   ├── suspects.rs        # Suspect CRUD operations
│   │   │   │   └── shared.rs          # Inter-system API
│   │   │   ├── models/                # Data structures
│   │   │   │   ├── mod.rs
│   │   │   │   └── suspect.rs
│   │   │   └── database/              # Database layer
│   │   │       ├── mod.rs
│   │   │       ├── connection.rs      # PostgreSQL connection pool
│   │   │       └── queries.rs         # SQL queries
│   │   ├── Cargo.toml                 # Rust dependencies
│   │   └── .env.example               # Environment configuration
│   │
│   └── hospital-system/                # Hospital backend service (Rust/Actix)
│       ├── src/
│       │   ├── main.rs                # Application entry point
│       │   ├── api/                   # REST API endpoints
│       │   │   ├── mod.rs
│       │   │   ├── patients.rs        # Patient CRUD operations
│       │   │   └── shared.rs          # Inter-system API
│       │   ├── models/                # Data structures
│       │   │   ├── mod.rs
│       │   │   └── patient.rs
│       │   └── database/              # Database layer
│       │       ├── mod.rs
│       │       ├── connection.rs      # PostgreSQL connection pool
│       │       └── queries.rs         # SQL queries
│       ├── Cargo.toml                 # Rust dependencies
│       └── .env.example               # Environment configuration
│
├── frontend/                           # React + TypeScript UI
│   ├── src/
│   │   ├── main.tsx                   # React entry point
│   │   ├── App.tsx                    # Root component
│   │   ├── types.ts                   # TypeScript type definitions
│   │   └── components/
│   │       ├── PoliceData.tsx         # Display suspects
│   │       └── HospitalData.tsx       # Display patients
│   ├── index.html                     # HTML template
│   ├── package.json                   # Node dependencies
│   ├── tsconfig.json                  # TypeScript configuration
│   ├── vite.config.ts                 # Vite config with API proxies
│   └── .gitignore
│
├── shared/
│   └── database-schemas/               # SQL schema definitions
│       ├── schema.sql                 # Database structure and FDW setup
│       └── seed-data.sql              # Sample data for testing
│
├── docs/                               # Documentation
│   ├── psql-guide.md                  # PostgreSQL setup guide
│   ├── API.md                         # Complete API reference
│   └── TESTING.md                     # Backend testing guide
│
├── .gitignore                         # Git ignore patterns
├── README.md                          # Project overview and setup
└── ARCHITECTURE.md                    # This file
```

---

## Technology Stack

### Backend

- **Language:** Rust (latest stable)
- **Web Framework:** Actix-web 4.x
- **Database Driver:** SQLx 0.7 (async PostgreSQL driver)
- **Async Runtime:** Tokio 1.x
- **Serialization:** Serde 1.x
- **CORS:** Actix-cors 0.7
- **Logging:** env_logger 0.11, log 0.4
- **Environment:** dotenv 0.15

### Frontend

- **UI Library:** React 18.3.1
- **Language:** TypeScript 5.5.3
- **Build Tool:** Vite 5.4.0
- **Dev Server:** Vite (with API proxy)

### Database

- **Database:** PostgreSQL 15+
- **Extensions:** postgres_fdw (Foreign Data Wrapper)

### Development Tools

- **Rust:** Cargo (package manager)
- **Node.js:** npm (package manager)
- **macOS:** Homebrew (for PostgreSQL installation)

---

## Architecture Diagram

```
┌─────────────────────────────────────────┐
│     React Frontend (Port: 3000)         │
│  ┌───────────────────────────────────┐  │
│  │  PoliceData   │  HospitalData     │  │
│  │  Component    │  Component        │  │
│  └───────────────────────────────────┘  │
│         │                    │          │
│      Vite Proxy (API calls)             │
└─────────┼────────────────────┼──────────┘
          │                    │
          │ HTTP REST          │ HTTP REST
          ▼                    ▼
┌─────────────────────┐  ┌─────────────────────┐
│  Police Backend     │  │  Hospital Backend   │
│  (Rust/Actix)       │  │  (Rust/Actix)       │
│  Port: 8000         │  │  Port: 8001         │
│  ┌───────────────┐  │  │  ┌───────────────┐  │
│  │ API Endpoints │  │  │  │ API Endpoints │  │
│  │ - /suspects   │  │  │  │ - /patients   │  │
│  │ - /api/shared │  │  │  │ - /api/shared │  │
│  └───────────────┘  │  │  └───────────────┘  │
│         │           │  │         │           │
│    SQLx Pool        │  │    SQLx Pool        │
└──────────┬──────────┘  └──────────┬──────────┘
           │                        │
           │ postgres_fdw           │
           │ (Foreign Data Wrapper) │
           ▼                        │
    ┌──────────────────────┐        │
    │   PostgreSQL Setup   │        │
    ├──────────────────────┤        │
    │  police_db           │        │
    │  ├─ suspects table   │        │
    │  └─ patients (FDW)   │◄───────┘
    │                      │
    │  hospital_db         │
    │  └─ patients table   │
    └──────────────────────┘
               ▲
               │  Trigger: sync_flag_to_hospital()
               │  Auto-syncs flag updates
```

---

## Backend Architecture

### Police System (Port 8000)

**API Endpoints:**

- `GET /suspects` - List all suspects
- `POST /suspects` - Create new suspect
- `GET /suspects/{id}` - Get suspect by ID
- `PUT /suspects/{id}` - Update suspect
- `DELETE /suspects/{id}` - Delete suspect
- `GET /suspects/personal/{personal_id}` - Get suspect by Swedish personal ID
- `PUT /suspects/{personal_id}/flag` - Update flag (triggers sync)
- `GET /api/shared/suspects` - List all suspects (for hospital)
- `GET /api/shared/suspects/{personal_id}` - Check suspect record (for hospital)
- `GET /health` - Health check

**Key Features:**

- Flag updates automatically sync to hospital database via trigger
- Provides shared API for hospital system to query suspect records
- Uses SQLx for type-safe, async database queries
- Connection pooling with max 5 connections
- CORS enabled for localhost:8001 and localhost:3000

### Hospital System (Port 8001)

**API Endpoints:**

- `GET /patients` - List all patients
- `POST /patients` - Create new patient
- `GET /patients/{id}` - Get patient by ID
- `PUT /patients/{id}` - Update patient
- `DELETE /patients/{id}` - Delete patient
- `GET /patients/personal/{personal_id}` - Get patient by Swedish personal ID
- `GET /patients/flagged` - Get all flagged patients (auto-synced from police)
- `GET /api/shared/patients` - List all patients (for police)
- `GET /api/shared/patients/flagged` - List flagged patients (for police)
- `GET /api/shared/patients/{personal_id}` - Check patient record (for police)
- `GET /health` - Health check

**Key Features:**

- Receives automatic flag updates from police system
- Provides shared API for police system to query patient records
- Read-only flag field (updated automatically from police)
- Uses SQLx for type-safe, async database queries
- Connection pooling with max 5 connections
- CORS enabled for localhost:8000 and localhost:3000

### Shared Architecture Patterns

Both backend systems follow identical patterns:

1. **Layered Architecture:**

   - `main.rs` - Application entry, HTTP server setup
   - `api/` - REST endpoint handlers
   - `models/` - Data structures (Suspect/Patient)
   - `database/` - Database connection and queries

2. **Environment Configuration:**

   ```env
   DATABASE_URL=postgresql://postgres@localhost/[db_name]
   SERVER_PORT=[8000|8001]
   [OTHER_SYSTEM]_API_URL=http://localhost:[8001|8000]
   ```

3. **Error Handling:**

   - Consistent JSON error responses
   - Detailed logging with log levels
   - HTTP status codes (200, 201, 204, 404, 500)

4. **CORS Configuration:**
   - Allows cross-origin requests between systems
   - Permits frontend access (localhost:3000)
   - Development mode: allows all origins

---

## Frontend Architecture

### Technology Stack

- **React 18.3.1** - UI library
- **TypeScript 5.5.3** - Type safety
- **Vite 5.4.0** - Build tool and dev server

### Component Structure

```
src/
├── main.tsx              # React entry point
├── App.tsx               # Root component
├── types.ts              # TypeScript type definitions
└── components/
    ├── PoliceData.tsx    # Fetches and displays suspects
    └── HospitalData.tsx  # Fetches and displays patients
```

### Key Features

1. **API Proxy Configuration:**

   ```typescript
   // vite.config.ts
   proxy: {
     "/api/police": {
       target: "http://localhost:8000",
       changeOrigin: true,
       rewrite: (path) => path.replace(/^\/api\/police/, ""),
     },
     "/api/hospital": {
       target: "http://localhost:8001",
       changeOrigin: true,
       rewrite: (path) => path.replace(/^\/api\/hospital/, ""),
     },
   }
   ```

2. **Type Definitions:**

   - `Suspect` and `Patient` interfaces match backend models
   - `CreateSuspect`, `CreatePatient` for POST requests
   - `UpdateSuspect`, `UpdatePatient` for PUT requests
   - `FlagUpdate` for flag status changes
   - `HealthResponse` for health checks
   - `ApiError` for error responses

3. **Data Fetching:**

   - Uses `useEffect` for initial data load
   - `useState` for loading, error, and data state
   - Error handling with try-catch
   - Loading indicators

4. **Display:**
   - Simple HTML tables
   - Minimal styling (no CSS framework)
   - Read-only data visualization
   - Null-safe rendering

### Current Limitations

- **Read-only:** No create, update, or delete functionality
- **No forms:** Data entry must be done via API directly
- **No routing:** Single-page application
- **No real-time updates:** Manual refresh required
- **Basic styling:** Plain HTML tables
- **No authentication:** Open access (development mode)

---

## Database Architecture

### Schema Design

**Police Database (police_db):**

```sql
CREATE TABLE suspects (
    id SERIAL PRIMARY KEY,
    full_name TEXT,
    personal_id TEXT UNIQUE,
    flag BOOLEAN
);
```

**Hospital Database (hospital_db):**

```sql
CREATE TABLE patients (
    id SERIAL PRIMARY KEY,
    full_name TEXT,
    personal_id TEXT UNIQUE,
    flag BOOLEAN
);
```

### Foreign Data Wrapper (FDW) Configuration

The police database can directly query the hospital's `patients` table:

```sql
-- In police_db
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
```

### Automatic Flag Synchronization

When a suspect's flag is updated in the police database, it automatically synchronizes to the hospital database:

```sql
-- Trigger function
CREATE OR REPLACE FUNCTION sync_flag_to_hospital()
RETURNS TRIGGER AS $$
BEGIN
    UPDATE patients
    SET flag = NEW.flag
    WHERE personal_id = NEW.personal_id;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Trigger
CREATE TRIGGER trg_sync_flag
    AFTER UPDATE OF flag ON suspects
    FOR EACH ROW
    EXECUTE FUNCTION sync_flag_to_hospital();
```

### Flow Diagram

```
┌─────────────────────────────────────────┐
│ Police updates flag for suspect         │
│ UPDATE suspects SET flag = true         │
│ WHERE personal_id = '19850312-2398'     │
└──────────────┬──────────────────────────┘
               │
               ▼
┌─────────────────────────────────────────┐
│ Trigger: trg_sync_flag fires            │
│ Executes: sync_flag_to_hospital()       │
└──────────────┬──────────────────────────┘
               │
               ▼
┌─────────────────────────────────────────┐
│ UPDATE patients SET flag = true         │
│ WHERE personal_id = '19850312-2398'     │
│ (via Foreign Data Wrapper)              │
└─────────────────────────────────────────┘
```

**Key Benefits:**

- No application-level coordination needed
- Instant synchronization (same transaction)
- Database-level consistency guarantees
- Simpler application code

### Sample Data

The seed data includes:

- **8 shared records** - Present in both police and hospital databases
- **2 police-only records** - Simon Nyberg and Carina Dahl
- Swedish names and personal IDs
- Mixed flag statuses for testing

---

## Implementation Status

### ✅ Completed

**Backend:**

- [x] Police system (Rust/Actix)
- [x] Hospital system (Rust/Actix)
- [x] REST API endpoints (CRUD + shared)
- [x] Database connection pools
- [x] SQLx queries
- [x] CORS configuration
- [x] Health check endpoints
- [x] Flag update endpoint
- [x] Shared API for cross-system queries
- [x] Logging and error handling

**Database:**

- [x] PostgreSQL schema
- [x] Foreign Data Wrapper setup
- [x] Flag synchronization trigger
- [x] Seed data

**Frontend:**

- [x] React + TypeScript setup
- [x] Vite configuration with API proxy
- [x] Type definitions
- [x] PoliceData component
- [x] HospitalData component
- [x] Data fetching with useEffect
- [x] Error handling and loading states

**Documentation:**

- [x] PostgreSQL setup guide (psql-guide.md)
- [x] API documentation (API.md)
- [x] Testing guide (TESTING.md)
- [x] Architecture documentation (this file)
- [x] README with quick start

### ⏳ To Be Implemented (Future Enhancements)

**Backend:**

- [ ] JWT authentication
- [ ] API key validation
- [ ] Rate limiting
- [ ] Audit logging
- [ ] Input validation middleware
- [ ] Pagination for list endpoints
- [ ] Search and filtering
- [ ] WebSocket for real-time updates

**Frontend:**

- [ ] Create/Update/Delete forms
- [ ] Flag toggle interface
- [ ] Cross-system query interface
- [ ] Real-time synchronization visualization
- [ ] Flagged patients view
- [ ] Search and filtering UI
- [ ] Proper styling (CSS/Tailwind)
- [ ] Loading skeletons
- [ ] Toast notifications
- [ ] Confirmation dialogs
- [ ] Form validation

**Database:**

- [ ] Audit tables
- [ ] Performance indexes
- [ ] Database migrations system
- [ ] Backup strategy

**Testing:**

- [ ] Unit tests (Rust)
- [ ] Integration tests (Rust)
- [ ] Frontend component tests
- [ ] End-to-end tests
- [ ] CI/CD pipeline

**Security:**

- [ ] HTTPS enforcement
- [ ] Encrypted database connections
- [ ] Input sanitization
- [ ] SQL injection prevention (already handled by SQLx)
- [ ] GDPR compliance measures
- [ ] Data retention policies

**Deployment:**

- [ ] Docker containers
- [ ] Docker Compose setup
- [ ] Production environment configuration
- [ ] Monitoring and alerting
- [ ] Backup automation

---

## Security Considerations

### Current State (Development)

**Strengths:**

- SQLx prevents SQL injection through prepared statements
- Rust's memory safety prevents buffer overflows
- Type-safe APIs prevent data corruption

**Limitations (Development Only):**

- ❌ No authentication
- ❌ No authorization/access control
- ❌ No rate limiting
- ❌ No input validation
- ❌ HTTP only (no HTTPS)
- ❌ CORS allows all origins
- ❌ No audit logging
- ❌ Database credentials in .env files

### Production Requirements

**Must Implement:**

1. **Authentication & Authorization:**

   - JWT-based authentication
   - Role-based access control (RBAC)
   - Separate permissions for police vs hospital users
   - API key authentication for inter-system communication

2. **Network Security:**

   - HTTPS/TLS for all connections
   - Encrypted database connections
   - Firewall rules
   - VPN for inter-system communication

3. **Data Protection:**

   - Encryption at rest for sensitive data
   - Hashed passwords
   - Secure credential storage (e.g., HashiCorp Vault)
   - GDPR compliance (data retention, right to deletion)

4. **Input Validation:**

   - Swedish personal ID format validation
   - Request payload size limits
   - SQL injection prevention (already handled by SQLx)
   - XSS prevention in frontend

5. **Monitoring & Auditing:**

   - Audit logs for all data access
   - Security event logging
   - Intrusion detection
   - Performance monitoring

6. **Rate Limiting:**
   - Per-IP rate limiting
   - Per-user rate limiting
   - API endpoint throttling

---

## Development Workflow

### Starting the System

1. **Start PostgreSQL:**

   ```bash
   brew services start postgresql@15
   ```

2. **Start Police Backend:**

   ```bash
   cd backend/police-system
   cargo run
   ```

3. **Start Hospital Backend:**

   ```bash
   cd backend/hospital-system
   cargo run
   ```

4. **Start Frontend:**

   ```bash
   cd frontend
   npm run dev
   ```

5. **Access:**
   - Frontend: http://localhost:3000
   - Police API: http://localhost:8000
   - Hospital API: http://localhost:8001

### Testing Flag Synchronization

```bash
# Flag a suspect in police system
curl -X PUT http://localhost:8000/suspects/19850312-2398/flag \
  -H "Content-Type: application/json" \
  -d '{"flag": true}'

# Verify sync in hospital system (immediate)
curl http://localhost:8001/patients/personal/19850312-2398
```

---

## Performance Considerations

### Database

- Connection pooling (max 5 connections per service)
- Indexes on `personal_id` (UNIQUE constraint provides index)
- Consider additional indexes for `flag` if querying flagged records frequently

### Backend

- Async/await throughout (non-blocking I/O)
- Compiled Rust binary (fast execution)
- Minimal memory footprint

### Frontend

- Vite for fast development builds
- React 18 with concurrent rendering
- Consider implementing pagination for large datasets
- Consider implementing virtual scrolling for very large lists

---

## Scalability Considerations

### Current Limitations

- Single database instance
- No load balancing
- No caching layer
- Synchronous flag synchronization (blocks on trigger)

### Future Improvements

- **Database:** PostgreSQL read replicas for queries
- **Backend:** Horizontal scaling with load balancer
- **Caching:** Redis for frequently accessed data
- **Message Queue:** Async flag synchronization with RabbitMQ/Kafka
- **CDN:** Static asset delivery
- **Microservices:** Further decomposition if needed

---

## Contributing

When contributing to this project:

1. Follow Rust style guidelines (`cargo fmt`)
2. Add tests for new functionality
3. Update documentation
4. Ensure all tests pass (`cargo test`)
5. Use meaningful commit messages
6. Update ARCHITECTURE.md for significant changes

---

## References

- [PostgreSQL Foreign Data Wrapper Documentation](https://www.postgresql.org/docs/current/postgres-fdw.html)
- [Actix-web Documentation](https://actix.rs/)
- [SQLx Documentation](https://github.com/launchbadge/sqlx)
- [React Documentation](https://react.dev/)
- [Vite Documentation](https://vitejs.dev/)

---

_Last Updated: January 2025_
