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
├── frontend/                              # React + TypeScript demo interface
│   ├── public/
│   │   └── index.html                     # HTML template
│   ├── src/
│   │   ├── components/                    # React components
│   │   │   ├── police/                    # Police system components
│   │   │   │   ├── SuspectList.tsx        # Display all suspects
│   │   │   │   ├── SuspectForm.tsx        # Create/update suspect form
│   │   │   │   └── FlagControl.tsx        # Flag management interface
│   │   │   ├── hospital/                  # Hospital system components
│   │   │   │   ├── PatientList.tsx        # Display all patients
│   │   │   │   ├── PatientForm.tsx        # Create/update patient form
│   │   │   │   └── FlaggedPatients.tsx    # View flagged patients
│   │   │   ├── shared/                    # Shared/cross-system components
│   │   │   │   ├── CrossSystemQuery.tsx   # Inter-system data lookup
│   │   │   │   ├── PersonCard.tsx         # Reusable person display card
│   │   │   │   └── SyncIndicator.tsx      # Real-time sync status display
│   │   │   └── layout/                    # Layout components
│   │   │       ├── Header.tsx             # Application header
│   │   │       ├── SystemPanel.tsx        # Container for each system view
│   │   │       └── Footer.tsx             # Application footer
│   │   ├── services/                      # API communication layer
│   │   │   ├── policeApi.ts               # Police system API calls
│   │   │   ├── hospitalApi.ts             # Hospital system API calls
│   │   │   ├── api.ts                     # Shared API utilities (axios config)
│   │   │   └── types.ts                   # TypeScript type definitions
│   │   ├── hooks/                         # Custom React hooks
│   │   │   ├── usePoliceData.ts           # Police data fetching/management
│   │   │   ├── useHospitalData.ts         # Hospital data fetching/management
│   │   │   ├── useFlagSync.ts             # Flag synchronization monitoring
│   │   │   └── useCrossSystemQuery.ts     # Inter-system queries
│   │   ├── context/                       # React Context for state
│   │   │   └── AppContext.tsx             # Global application state
│   │   ├── utils/                         # Utility functions
│   │   │   ├── validation.ts              # Swedish personal ID validation
│   │   │   └── formatting.ts              # Data formatting helpers
│   │   ├── App.tsx                        # Root application component
│   │   ├── main.tsx                       # Application entry point
│   │   └── index.css                      # Global styles
│   ├── package.json                       # Node dependencies
│   ├── tsconfig.json                      # TypeScript configuration
│   ├── vite.config.ts                     # Vite bundler configuration
│   └── .env.example                       # Environment variables template
│
├── shared/
│   └── database-schemas/                  # SQL schema definitions
│       ├── schema.sql                     # Database structure and FDW setup
│       └── seed-data.sql                  # Sample data for testing
│
├── docs/                                  # Documentation
│   ├── psql-guide.md                      # PostgreSQL setup and usage guide
│   ├── API.md                             # Complete API reference
│   └── TESTING.md                         # Backend testing guide
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

### **frontend/**

React + TypeScript demo interface that demonstrates the system's core functionality:

- **src/components/** - UI components organized by system (police/hospital) and shared functionality
- **src/services/** - API communication layer that interfaces with both backend systems
- **src/hooks/** - Custom React hooks for data fetching, state management, and synchronization
- **src/context/** - Global state management using React Context API
- **src/utils/** - Helper functions for validation and formatting

The frontend serves as a minimal demonstration interface, focusing on:

- Visualizing data from both systems side-by-side
- Demonstrating flag synchronization in real-time
- Showing cross-system data queries
- Providing basic CRUD operations for testing

### **shared/database-schemas/**

SQL files for database setup:

- **schema.sql** - Creates both databases, sets up tables, configures postgres_fdw for cross-database synchronization, and creates triggers
- **seed-data.sql** - Inserts sample Swedish individuals for testing

### **docs/**

- **psql-guide.md** - Comprehensive guide for PostgreSQL setup, database creation, and testing synchronization
- **API.md** - Complete REST API documentation for both systems
- **TESTING.md** - Manual testing guide with test scenarios and scripts

## Architecture Overview

```
┌─────────────────────────────────────────┐
│       React Frontend (Port: 5173)       │
│  ┌───────────────────────────────────┐  │
│  │  Police View  │  Hospital View    │  │
│  │  - Suspects   │  - Patients       │  │
│  │  - Flag ctrl  │  - Flagged list   │  │
│  │  - Query hosp │  - Query police   │  │
│  └───────────────────────────────────┘  │
│         │                    │          │
│      Axios API Services                 │
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
│  │ - Suspects    │  │  │  │ - Patients    │  │
│  │ - Flag mgmt   │  │  │  │ - Flagged     │  │
│  │ - Shared API  │  │  │  │ - Shared API  │  │
│  └───────────────┘  │  │  └───────────────┘  │
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
               │              Auto-sync trigger
               │              (flag updates)
```

## Frontend Architecture Details

### Component Organization

The frontend follows a modular architecture with clear separation of concerns:

#### **Police System Components** (`components/police/`)

- **SuspectList.tsx** - Displays all suspects in a table/list format with sorting and filtering
- **SuspectForm.tsx** - Form for creating new suspects and updating existing ones
- **FlagControl.tsx** - Dedicated interface for flagging/unflagging suspects with visual feedback

#### **Hospital System Components** (`components/hospital/`)

- **PatientList.tsx** - Displays all patients with their flag status
- **PatientForm.tsx** - Form for patient registration and updates
- **FlaggedPatients.tsx** - Filtered view showing only flagged patients (auto-synced from police)

#### **Shared Components** (`components/shared/`)

- **CrossSystemQuery.tsx** - Interface for querying one system from another (e.g., hospital checking if patient has police record)
- **PersonCard.tsx** - Reusable card component for displaying person information consistently
- **SyncIndicator.tsx** - Visual indicator showing flag synchronization status in real-time

#### **Layout Components** (`components/layout/`)

- **Header.tsx** - Application title, navigation, and system status indicators
- **SystemPanel.tsx** - Container component for each system's view with consistent styling
- **Footer.tsx** - Footer with project information and links

### Data Flow Architecture

```
User Interaction
      ↓
React Component
      ↓
Custom Hook (usePoliceData / useHospitalData)
      ↓
API Service (policeApi.ts / hospitalApi.ts)
      ↓
Axios HTTP Request
      ↓
Backend REST API
      ↓
PostgreSQL Database
```

### API Service Layer

The service layer (`services/`) provides a clean abstraction over HTTP requests:

**policeApi.ts** - Police system operations:

- `getAllSuspects()` - GET /suspects
- `getSuspectById(id)` - GET /suspects/{id}
- `getSuspectByPersonalId(personalId)` - GET /suspects/personal/{personalId}
- `createSuspect(data)` - POST /suspects
- `updateSuspect(id, data)` - PUT /suspects/{id}
- `deleteSuspect(id)` - DELETE /suspects/{id}
- `updateFlag(personalId, flag)` - PUT /suspects/{personalId}/flag
- `queryPatient(personalId)` - GET /api/shared/patients/{personalId}

**hospitalApi.ts** - Hospital system operations:

- `getAllPatients()` - GET /patients
- `getPatientById(id)` - GET /patients/{id}
- `getPatientByPersonalId(personalId)` - GET /patients/personal/{personalId}
- `getFlaggedPatients()` - GET /patients/flagged
- `createPatient(data)` - POST /patients
- `updatePatient(id, data)` - PUT /patients/{id}
- `deletePatient(id)` - DELETE /patients/{id}
- `querySuspect(personalId)` - GET /api/shared/suspects/{personalId}

**api.ts** - Shared utilities:

- Axios instance with base configuration
- Request/response interceptors for error handling
- Common headers (Content-Type, CORS)

### Custom Hooks

Custom hooks (`hooks/`) encapsulate data fetching and state management logic:

**usePoliceData.ts**

- Fetches and manages suspect data
- Provides loading states and error handling
- Implements create, update, delete operations
- Polls for updates or uses WebSocket for real-time data (if implemented)

**useHospitalData.ts**

- Fetches and manages patient data
- Handles flagged patients separately
- Provides CRUD operations
- Syncs with police system changes

**useFlagSync.ts**

- Monitors flag synchronization status
- Compares flag states between systems
- Provides synchronization indicators
- Triggers re-fetches when flags change

**useCrossSystemQuery.ts**

- Handles queries from one system to another
- Manages query results and error states
- Provides lookup functionality for personal IDs

### State Management

The frontend uses **React Context API** for minimal global state management:

```typescript
// AppContext.tsx structure
{
  police: {
    suspects: Suspect[],
    loading: boolean,
    error: string | null
  },
  hospital: {
    patients: Patient[],
    flaggedPatients: Patient[],
    loading: boolean,
    error: string | null
  },
  sync: {
    lastSyncTime: Date,
    syncStatus: 'idle' | 'syncing' | 'success' | 'error'
  }
}
```

No complex state management library (like Redux) is needed for this demo interface.

### TypeScript Types

**types.ts** defines shared interfaces:

```typescript
interface Suspect {
  id: number;
  full_name: string | null;
  personal_id: string | null;
  flag: boolean | null;
}

interface Patient {
  id: number;
  full_name: string | null;
  personal_id: string | null;
  flag: boolean | null;
}

interface CreateSuspect {
  full_name: string;
  personal_id: string;
  flag: boolean;
}

interface CreatePatient {
  full_name: string;
  personal_id: string;
  flag: boolean;
}

interface FlagUpdate {
  flag: boolean;
}
```

### Key Demo Features

The frontend focuses on demonstrating these core capabilities:

1. **Side-by-Side System Views**

   - Police and hospital systems visible simultaneously
   - Easy comparison of data in both systems

2. **Flag Synchronization Visualization**

   - Real-time indicators when flags are updated
   - Visual highlighting of synchronized records
   - Sync status animations

3. **Cross-System Queries**

   - Hospital can check if patient has police record
   - Police can check if suspect has hospital record
   - Clear display of query results

4. **CRUD Operations**

   - Create, read, update, delete for both systems
   - Form validation (Swedish personal ID format)
   - Success/error feedback

5. **Flagged Patient Monitoring**
   - Dedicated view for flagged patients in hospital
   - Automatic updates when police flags suspects
   - Clear visual indicators for flagged status

### Technology Stack

**Core:**

- React 18+ with TypeScript
- Vite (fast build tool and dev server)
- Axios (HTTP client)

**Styling:**

- Tailwind CSS (utility-first CSS framework) - recommended for rapid development
- OR minimal custom CSS if preferred

**Development Tools:**

- ESLint (code linting)
- Prettier (code formatting)
- TypeScript compiler

### Environment Configuration

**Frontend .env.example:**

```env
# Backend API URLs
VITE_POLICE_API_URL=http://localhost:8000
VITE_HOSPITAL_API_URL=http://localhost:8001

# Polling interval for data refresh (milliseconds)
VITE_POLL_INTERVAL=5000

# Enable debug logging
VITE_DEBUG_MODE=false
```

### Integration Points with Backend

The frontend integrates with backend systems through:

1. **REST API Endpoints** - All communication uses documented REST APIs
2. **CORS Configuration** - Backend CORS settings allow frontend origin (localhost:5173)
3. **Error Handling** - Frontend gracefully handles API errors (404, 500, etc.)
4. **Data Validation** - Client-side validation matches backend requirements
5. **Real-time Updates** - Polling mechanism to detect flag synchronization (optional WebSocket upgrade)

### Development Workflow

1. **Start PostgreSQL** - `brew services start postgresql@15`
2. **Start Police Backend** - `cd backend/police-system && cargo run` (port 8000)
3. **Start Hospital Backend** - `cd backend/hospital-system && cargo run` (port 8001)
4. **Start Frontend Dev Server** - `cd frontend && npm run dev` (port 5173)
5. **Access Demo** - Navigate to `http://localhost:5173`

### Recommended File Structure Example

```
frontend/src/
├── App.tsx                    # Root component with routing
├── main.tsx                   # Entry point
├── components/
│   ├── police/
│   │   ├── SuspectList.tsx
│   │   ├── SuspectForm.tsx
│   │   └── FlagControl.tsx
│   ├── hospital/
│   │   ├── PatientList.tsx
│   │   ├── PatientForm.tsx
│   │   └── FlaggedPatients.tsx
│   ├── shared/
│   │   ├── CrossSystemQuery.tsx
│   │   ├── PersonCard.tsx
│   │   └── SyncIndicator.tsx
│   └── layout/
│       ├── Header.tsx
│       ├── SystemPanel.tsx
│       └── Footer.tsx
├── services/
│   ├── api.ts                # Axios configuration
│   ├── policeApi.ts          # Police API calls
│   ├── hospitalApi.ts        # Hospital API calls
│   └── types.ts              # TypeScript types
├── hooks/
│   ├── usePoliceData.ts
│   ├── useHospitalData.ts
│   ├── useFlagSync.ts
│   └── useCrossSystemQuery.ts
├── context/
│   └── AppContext.tsx        # Global state
├── utils/
│   ├── validation.ts         # Personal ID validation
│   └── formatting.ts         # Display formatting
└── index.css                 # Global styles
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
- ✅ Rust backend services (both police and hospital systems)
- ✅ REST API endpoints
- ✅ Database connection pools
- ✅ API documentation
- ✅ Backend testing guide

**To Be Implemented:**

- ⏳ Frontend React application
  - [ ] Project setup (Vite + React + TypeScript)
  - [ ] Component implementation
  - [ ] API service layer
  - [ ] Custom hooks for data management
  - [ ] State management with Context API
  - [ ] Styling (Tailwind CSS or custom)
  - [ ] Flag synchronization visualization
  - [ ] Cross-system query interface
  - [ ] Form validation and error handling

## Technology Stack

**Backend:**

- Rust (latest stable)
- actix-web (web framework)
- sqlx (async PostgreSQL driver)
- tokio (async runtime)
- serde (serialization)

**Frontend:**

- React 18+ (UI library)
- TypeScript (type safety)
- Vite (build tool)
- Axios (HTTP client)
- Tailwind CSS (styling) - recommended
- React Context API (state management)

**Database:**

- PostgreSQL 15+
- postgres_fdw extension

**Development Tools:**

- Cargo (Rust package manager)
- npm/yarn (Node package manager)
- VS Code with Rust and React extensions
- Homebrew (macOS package manager)

## Environment Configuration

### Backend Systems

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

### Frontend

The frontend requires a `.env` file for API endpoints:

```env
VITE_POLICE_API_URL=http://localhost:8000
VITE_HOSPITAL_API_URL=http://localhost:8001
VITE_POLL_INTERVAL=5000
VITE_DEBUG_MODE=false
```

## Getting Started

### Backend Setup

1. **Install PostgreSQL** (see docs/psql-guide.md)
2. **Create databases:**
   ```bash
   psql -U postgres -f shared/database-schemas/schema.sql
   ```
3. **Seed data:**
   ```bash
   psql -U postgres -f shared/database-schemas/seed-data.sql
   ```
4. **Configure environment:**
   ```bash
   cp backend/police-system/.env.example backend/police-system/.env
   cp backend/hospital-system/.env.example backend/hospital-system/.env
   ```
5. **Start backend services:**

   ```bash
   # Terminal 1 - Police System
   cd backend/police-system
   cargo run

   # Terminal 2 - Hospital System
   cd backend/hospital-system
   cargo run
   ```

### Frontend Setup

1. **Install dependencies:**
   ```bash
   cd frontend
   npm install
   ```
2. **Configure environment:**
   ```bash
   cp .env.example .env
   ```
3. **Start development server:**
   ```bash
   npm run dev
   ```
4. **Access the application:**
   - Frontend: `http://localhost:5173`
   - Police API: `http://localhost:8000`
   - Hospital API: `http://localhost:8001`

## Security Considerations

**Current State (Development):**

- No authentication
- Direct database access
- Local-only connections
- CORS enabled for localhost

**Production Requirements (Future):**

- JWT authentication for both backend and frontend
- API key validation
- Rate limiting
- Encrypted connections (HTTPS)
- Audit logging
- GDPR compliance for personal data
- Secure storage of credentials
- Input sanitization and validation

## Testing Strategy

### Backend Testing

- Manual API testing (see docs/TESTING.md)
- Automated test script (`test-all.sh`)
- Database-level verification
- Flag synchronization validation

### Frontend Testing (To Be Implemented)

- Unit tests with Vitest
- Component tests with React Testing Library
- Integration tests for API calls
- End-to-end tests with Playwright (optional)

## Notes

- Personal IDs use Swedish format (YYYYMMDD-XXXX)
- Sample data includes 8 shared records + 2 police-only records
- Flag synchronization is unidirectional (police → hospital)
- Frontend is intentionally minimal, focusing on core demo functionality
- The demo interface prioritizes clarity over visual polish
- Real-time synchronization can be enhanced with WebSocket (optional upgrade)
