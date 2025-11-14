# Project Architecture

A thesis project demonstrating secure data sharing between IT systems using PostgreSQL Foreign Data Wrapper (FDW) for automatic flag synchronization between police and hospital databases, with comprehensive security features including TLS, API key authentication, rate limiting, and audit logging.

## Table of Contents

- [Project Overview](#project-overview)
- [Project Structure](#project-structure)
- [Technology Stack](#technology-stack)
- [Architecture Diagram](#architecture-diagram)
- [Backend Architecture](#backend-architecture)
- [Frontend Architecture](#frontend-architecture)
- [Database Architecture](#database-architecture)
- [Security Architecture](#security-architecture)
- [Development Workflow](#development-workflow)

---

## Project Overview

This project demonstrates a proof-of-concept system for secure data sharing between two independent organizations (police and hospital) while maintaining data sovereignty. The key innovation is using PostgreSQL's Foreign Data Wrapper to enable automatic flag synchronization at the database level, eliminating the need for complex API-level coordination.

### Key Features

- **Independent backend services** for police and hospital systems
- **Automatic flag synchronization** via database triggers
- **Cross-system data queries** through authenticated shared API endpoints
- **React frontend** for data visualization with real-time updates
- **Swedish personal ID format** (YYYYMMDD-XXXX) for identity management
- **Comprehensive security** including TLS, API keys, rate limiting, audit logging
- **Type-safe database queries** with SQLx compile-time verification

### Research Context

This is an educational research project investigating how AI-generated code handles security requirements through iterative prompting. The project demonstrates both the capabilities and limitations of AI-assisted development in creating secure inter-system communication.

---

## Project Structure

```
thesis-project/
├── backend/
│   ├── police-system/                  # Police backend service (Rust/Actix)
│   │   ├── src/
│   │   │   ├── main.rs                # Application entry point, server setup
│   │   │   ├── config.rs              # Configuration management
│   │   │   ├── api/                   # REST API endpoints
│   │   │   │   ├── mod.rs
│   │   │   │   ├── suspects.rs        # Suspect CRUD operations
│   │   │   │   └── shared.rs          # Inter-system API (authenticated)
│   │   │   ├── models/                # Data structures
│   │   │   │   ├── mod.rs
│   │   │   │   └── suspect.rs
│   │   │   ├── database/              # Database layer
│   │   │   │   ├── mod.rs
│   │   │   │   ├── connection.rs      # PostgreSQL connection pool
│   │   │   │   └── queries.rs         # SQL queries (SQLx)
│   │   │   ├── middleware/            # Security middleware
│   │   │   │   ├── mod.rs
│   │   │   │   ├── auth.rs            # API key authentication
│   │   │   │   └── rate_limit.rs      # Rate limiting
│   │   │   └── utils/                 # Utilities
│   │   │       ├── mod.rs
│   │   │       ├── logging.rs         # Sanitized logging helpers
│   │   │       ├── error_handler.rs   # Error handling with correlation IDs
│   │   │       └── audit.rs           # Audit logging
│   │   ├── Cargo.toml                 # Rust dependencies
│   │   ├── .env.example               # Environment configuration template
│   │   ├── cert.pem                   # TLS certificate (self-signed for dev)
│   │   └── key.pem                    # TLS private key
│   │
│   └── hospital-system/                # Hospital backend service (Rust/Actix)
│       ├── src/                       # Same structure as police-system
│       │   ├── main.rs
│       │   ├── config.rs
│       │   ├── api/
│       │   │   ├── mod.rs
│       │   │   ├── patients.rs        # Patient CRUD operations
│       │   │   └── shared.rs          # Inter-system API (authenticated)
│       │   ├── models/
│       │   │   ├── mod.rs
│       │   │   └── patient.rs
│       │   ├── database/
│       │   │   ├── mod.rs
│       │   │   ├── connection.rs
│       │   │   └── queries.rs
│       │   ├── middleware/
│       │   │   ├── mod.rs
│       │   │   ├── auth.rs
│       │   │   ├── rate_limit.rs
│       │   │   └── sanitize_logs.rs   # PII sanitization
│       │   └── utils/
│       │       ├── mod.rs
│       │       ├── error_handler.rs
│       │       └── audit.rs
│       ├── Cargo.toml
│       ├── .env.example
│       ├── cert.pem
│       └── key.pem
│
├── frontend/                           # React + TypeScript UI
│   ├── src/
│   │   ├── main.tsx                   # React entry point
│   │   ├── App.tsx                    # Root component
│   │   ├── types.ts                   # TypeScript type definitions
│   │   ├── components/
│   │   │   ├── PoliceData.tsx         # Display suspects with flag toggle
│   │   │   └── HospitalData.tsx       # Display patients with auto-refresh
│   │   └── hooks/
│   │       └── usePolling.ts          # Custom polling hook with backoff
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
│   ├── TESTING.md                     # Backend testing guide (if exists)
│   └── security/
│       └── generate-cert-and-keys.md  # TLS certificate generation
│
├── testing/                            # Security testing tools
│   ├── README.md                      # Testing overview
│   ├── WARNING.md                     # Legal warnings
│   ├── nmap/                          # Network scanning
│   │   ├── instructions.md
│   │   └── run-all-scans.sh
│   └── zap/                           # OWASP ZAP testing
│       └── instructions.md
│
├── .gitignore                         # Git ignore patterns
├── README.md                          # Project overview and setup
├── ARCHITECTURE.md                    # This file
├── LICENSE                            # MIT License
└── SECURITY_NOTICE.md                 # Research context and warnings
```

---

## Technology Stack

### Backend Tech Stack

- **Language:** Rust (latest stable)
- **Web Framework:** Actix-web 4.x with rustls 0.21 for TLS
- **Database Driver:** SQLx 0.7 (async PostgreSQL driver with compile-time query verification)
- **Async Runtime:** Tokio 1.x
- **Serialization:** Serde 1.x with JSON support
- **CORS:** Actix-cors 0.7 with strict origin whitelisting
- **Logging:** env_logger 0.11, log 0.4
- **Environment:** dotenv 0.15
- **Security:**
  - `sha2` 0.10 - For hashing API keys and personal IDs
  - `subtle` 2.5 - Constant-time comparisons
  - `constant_time_eq` 0.3 - Timing attack prevention
  - `actix-governor` 0.5 - Rate limiting
  - `uuid` 1.10 - Correlation IDs for error tracking
- **Time:** chrono 0.4 with serde support
- **Validation:** regex 1.12, lazy_static 1.5

### Frontend Tech Stack

- **UI Library:** React 18.3.1
- **Language:** TypeScript 5.5.3
- **Build Tool:** Vite 5.4.0 with HMR
- **Features:**
  - Real-time data polling with exponential backoff
  - Flag toggle interface
  - Auto-refresh on visibility change
  - Error handling with retry logic

### Database Tech Stack

- **Database:** PostgreSQL 15+
- **Extensions:** postgres_fdw (Foreign Data Wrapper)
- **Connection Pooling:** SQLx managed pools (max 5 connections per service)
- **Query Safety:** Compile-time query verification via SQLx macros

### Development Tools

- **Rust:** Cargo (package manager and build tool)
- **Node.js:** npm (package manager)
- **macOS:** Homebrew (for PostgreSQL installation)
- **Testing:** nmap, OWASP ZAP (for security testing)

---

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────┐
│              React Frontend (Port: 3000)                    │
│  ┌────────────────────────────────────────────────────┐     │
│  │  PoliceData Component      HospitalData Component  │     │
│  │  - Display suspects        - Display patients      │     │
│  │  - Flag toggle UI          - Auto-refresh polling  │     │
│  │  - Error handling          - Loading states        │     │
│  └────────────────────────────────────────────────────┘     │
│           │ fetch API                │ fetch API            │
│           ▼                          ▼                       │
│  ┌──────────────────────────────────────────────────┐       │
│  │  Vite Dev Server - API Proxy                     │       │
│  │  /api/police/*  → http://localhost:8000          │       │
│  │  /api/hospital/* → http://localhost:8001         │       │
│  └──────────────────────────────────────────────────┘       │
└─────────────┬──────────────────────────┬────────────────────┘
              │                          │
              │ HTTP(S)                  │ HTTP(S)
              │ + API Key (shared)       │ + API Key (shared)
              ▼                          ▼
┌─────────────────────────────┐  ┌─────────────────────────────┐
│  Police Backend (Rust)      │  │  Hospital Backend (Rust)    │
│  Port: 8000                 │  │  Port: 8001                 │
│  ┌───────────────────────┐  │  │  ┌───────────────────────┐  │
│  │ Middleware Stack      │  │  │  │ Middleware Stack      │  │
│  │ - Logger              │  │  │  │ - Logger              │  │
│  │ - CORS (whitelist)    │  │  │  │ - CORS (whitelist)    │  │
│  │ - Rate Limiter (IP)   │  │  │  │ - Rate Limiter (IP)   │  │
│  │ - Security Headers    │  │  │  │ - Security Headers    │  │
│  └───────────────────────┘  │  │  └───────────────────────┘  │
│  ┌───────────────────────┐  │  │  ┌───────────────────────┐  │
│  │ API Routes            │  │  │  │ API Routes            │  │
│  │ /suspects             │  │  │  │ /patients             │  │
│  │ /suspects/flag (POST) │  │  │  │ /patients/flagged     │  │
│  │ /health               │  │  │  │ /health               │  │
│  └───────────────────────┘  │  │  └───────────────────────┘  │
│  ┌───────────────────────┐  │  │  ┌───────────────────────┐  │
│  │ Shared API (Auth)     │  │  │  │ Shared API (Auth)     │  │
│  │ /api/shared/suspects  │  │  │  │ /api/shared/patients  │  │
│  │ - API Key Required    │  │  │  │ - API Key Required    │  │
│  │ - Strict Rate Limit   │  │  │  │ - Strict Rate Limit   │  │
│  │ - Audit Logging       │  │  │  │ - Audit Logging       │  │
│  └───────────────────────┘  │  │  └───────────────────────┘  │
│            │                │  │            │                │
│       SQLx Pool             │  │       SQLx Pool             │
│    (max 5 connections)      │  │    (max 5 connections)      │
└────────────┬────────────────┘  └────────────┬────────────────┘
             │                                │
             │ postgres_fdw                   │
             │ Foreign Data Wrapper           │
             │ (Cross-database access)        │
             ▼                                │
      ┌──────────────────────────┐            │
      │   PostgreSQL Cluster     │            │
      ├──────────────────────────┤            │
      │  police_db               │            │
      │  ┌────────────────────┐  │            │
      │  │ suspects table     │  │            │
      │  │ - id (PK)          │  │            │
      │  │ - full_name        │  │            │
      │  │ - personal_id (UK) │  │            │
      │  │ - flag             │  │            │
      │  └────────────────────┘  │            │
      │  ┌────────────────────┐  │            │
      │  │ patients (FDW)     │◄─┼────────────┘
      │  │ - Foreign table    │  │
      │  │ - Read-only access │  │
      │  └────────────────────┘  │
      │                          │
      │  Trigger Function:       │
      │  sync_flag_to_hospital() │
      │  ├─ ON UPDATE OF flag    │
      │  └─ Updates patients.flag│
      ├──────────────────────────┤
      │  hospital_db             │
      │  ┌────────────────────┐  │
      │  │ patients table     │  │
      │  │ - id (PK)          │  │
      │  │ - full_name        │  │
      │  │ - personal_id (UK) │  │
      │  │ - flag (read-only) │  │
      │  └────────────────────┘  │
      └──────────────────────────┘
                 ▲
                 │
        Automatic flag sync
        via database trigger
```

---

## Backend Architecture

### Common Patterns (Both Systems)

Both police and hospital backends follow identical architectural patterns for consistency and maintainability.

#### 1. **Layered Architecture**

```
main.rs
  ├─ Configuration Loading (.env)
  ├─ Database Connection Pool
  ├─ Middleware Setup
  │   ├─ Logger
  │   ├─ CORS
  │   ├─ Rate Limiting
  │   └─ Security Headers
  ├─ Route Configuration
  │   ├─ Public Routes
  │   └─ Authenticated Routes (/api/shared)
  └─ HTTP Server Binding (HTTP or HTTPS)

api/
  ├─ suspects.rs / patients.rs
  │   └─ CRUD endpoints + flag update
  └─ shared.rs
      └─ Inter-system API (authenticated)

models/
  └─ suspect.rs / patient.rs
      ├─ Core data structure
      ├─ Create/Update structures
      └─ Validation logic

database/
  ├─ connection.rs
  │   └─ Connection pool management
  └─ queries.rs
      └─ Type-safe SQL queries (SQLx)

middleware/
  ├─ auth.rs
  │   └─ API key authentication
  └─ rate_limit.rs
      ├─ IP-based rate limiting
      └─ API-key-based rate limiting

utils/
  ├─ logging.rs
  │   └─ PII sanitization helpers
  ├─ error_handler.rs
  │   └─ Error responses with correlation IDs
  └─ audit.rs
      └─ Structured audit logging
```

#### 2. **Configuration Management**

**Environment Variables:**

```env
# Database
DATABASE_URL=postgresql://postgres@localhost/[db_name]

# Server
SERVER_PORT=[8000|8001]

# Security
API_KEY=<32+ character key>
[OTHER_SYSTEM]_API_KEY=<key for calling other system>
ALLOWED_ORIGINS=https://origin1,https://origin2

# TLS (Optional)
ENABLE_TLS=true|false
TLS_CERT_PATH=/path/to/cert.pem
TLS_KEY_PATH=/path/to/key.pem

# Rate Limiting
RATE_LIMIT_PER_SECOND=10
RATE_LIMIT_BURST=20
SHARED_API_RATE_LIMIT_PER_SECOND=1
SHARED_API_RATE_LIMIT_BURST=5
```

**Configuration Validation:**

- API keys must be ≥32 characters
- Production mode enforces HTTPS-only origins
- TLS paths validated when enabled
- Rate limits must be positive integers

#### 3. **Security Middleware Stack**

**Applied in Order:**

1. **Logger** - HTTP request/response logging
2. **CORS** - Strict origin whitelisting
3. **Rate Limiting** - IP-based (general) or API-key-based (shared)
4. **Security Headers** - Comprehensive HTTP security headers
5. **API Key Auth** - Applied only to `/api/shared/*` routes

**Security Headers:**

```rust
let mut security_headers = DefaultHeaders::new()
    .add(("X-Content-Type-Options", "nosniff"))
    .add(("X-Frame-Options", "DENY"))
    .add(("X-XSS-Protection", "1; mode=block"))
    .add(("Content-Security-Policy", "default-src 'none'"))
    .add(("Referrer-Policy", "no-referrer"))
    .add(("Permissions-Policy", "geolocation=(), microphone=(), camera=()"));

// Add HSTS only if TLS is enabled
if enable_tls {
    security_headers = security_headers.add((
        "Strict-Transport-Security",
        "max-age=31536000; includeSubDomains; preload"
    ));
}
```

#### 4. **API Key Authentication**

**Implementation:**

```rust
// Middleware extracts and validates API key
pub struct ApiKeyAuth {
    api_key: String,
}

impl ApiKeyAuth {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}

// Applied to /api/shared scope only
app.service(
    web::scope("/api/shared")
        .wrap(shared_api_rate_limiter)
        .wrap(ApiKeyAuth::new(api_key.clone()))
        .configure(api::configure_shared)
)
```

**Validation Features:**

- Constant-time comparison to prevent timing attacks
- Returns 401 Unauthorized for missing/invalid keys
- Logs all authentication attempts
- Hashes API keys in logs for privacy

#### 5. **Rate Limiting**

**Two-Tier System:**

**General Endpoints (IP-based):**

```rust
let rate_limiter = Governor::new(&GovernorConfigBuilder::default()
    .per_second(requests_per_second)
    .burst_size(burst_size)
    .finish()
    .unwrap()
);
```

**Shared API Endpoints (API-key-based):**

```rust
// Custom key extractor that hashes API keys
pub struct ApiKeyExtractor;

impl KeyExtractor for ApiKeyExtractor {
    type Key = String;

    fn extract(&self, req: &ServiceRequest) -> Result<Self::Key, Self::KeyExtractionError> {
        let api_key = req.headers()
            .get("X-API-Key")
            .and_then(|h| h.to_str().ok())
            .ok_or_else(|| ApiKeyError::new("Missing X-API-Key header"))?;

        // Hash for privacy
        let mut hasher = Sha256::new();
        hasher.update(api_key.as_bytes());
        let hash = format!("{:x}", hasher.finalize());
        Ok(hash[..32].to_string())
    }
}

let shared_api_rate_limiter = Governor::new(&GovernorConfigBuilder::default()
    .per_second(shared_api_rate_limit_per_second)
    .burst_size(shared_api_rate_limit_burst)
    .key_extractor(ApiKeyExtractor)
    .finish()
    .unwrap()
);
```

#### 6. **Audit Logging**

**Structured Logging Format:**

```rust
pub struct AuditLog {
    timestamp: DateTime<Utc>,
    event_type: EventType,  // SharedApiAccess, FlagUpdate, etc.
    actor: String,          // Hashed API key or "internal"
    action: Action,         // Read, Create, Update, Delete
    resource: String,       // Hashed identifier
    result: AuditResult,    // Success, Failure
    ip_address: Option<String>,
    details: Option<String>,
}
```

**Example Audit Entry:**

```json
{
  "timestamp": "2025-01-15T14:30:22Z",
  "event_type": "SHARED_API_ACCESS",
  "actor": "api_key:a3f5e8d2b1c4f7e9",
  "action": "READ",
  "resource": "patient:c8d9e2f1a5b3",
  "result": "SUCCESS",
  "ip_address": "192.168.1.100",
  "details": null
}
```

**Logged Events:**

- Shared API access attempts
- Flag updates
- Suspect/patient CRUD operations
- Authentication failures
- Rate limit violations

#### 7. **Error Handling**

**Correlation IDs:**

Every error generates a unique UUID for tracking:

```rust
pub fn handle_database_error<E: std::fmt::Display>(error: E, context: &str) -> HttpResponse {
    let correlation_id = Uuid::new_v4().to_string();

    // Log full error server-side
    log::error!("Database error [{}] in {}: {}", correlation_id, context, error);

    // Return generic error to client
    HttpResponse::InternalServerError().json(json!({
        "error": "Service temporarily unavailable",
        "correlation_id": correlation_id
    }))
}
```

**Benefits:**

- Clients never see sensitive error details
- Support can trace issues using correlation ID
- Full context preserved in server logs

#### 8. **Database Layer (SQLx)**

**Compile-Time Query Verification:**

```rust
pub async fn get_suspect_by_personal_id(
    pool: &PgPool,
    personal_id: &str,
) -> Result<Option<Suspect>, sqlx::Error> {
    let suspect = sqlx::query_as!(
        Suspect,
        "SELECT id, full_name, personal_id, flag FROM suspects WHERE personal_id = $1",
        personal_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(suspect)
}
```

**Features:**

- Type-safe queries verified at compile time
- Automatic parameter binding (SQL injection prevention)
- Async/await throughout
- Connection pooling

### Police System Specifics (Port 8000)

**Unique Responsibilities:**

1. **Flag Management:**

   - Primary source of truth for flag status
   - Triggers automatic sync to hospital database
   - POST `/suspects/flag` endpoint

2. **Shared API Endpoints:**

   - GET `/api/shared/suspects` - List all suspects
   - GET `/api/shared/suspects/{personal_id}` - Check specific suspect

3. **Environment Configuration:**
   ```env
   DATABASE_URL=postgresql://postgres@localhost/police_db
   SERVER_PORT=8000
   API_KEY=<key for authenticating hospital requests>
   HOSPITAL_API_KEY=<key for calling hospital>
   ```

### Hospital System Specifics (Port 8001)

**Unique Responsibilities:**

1. **Flag Synchronization Reception:**

   - Receives automatic flag updates from police database
   - Flag field is read-only via API
   - GET `/patients/flagged` endpoint

2. **Shared API Endpoints:**

   - GET `/api/shared/patients` - List all patients
   - GET `/api/shared/patients/flagged` - List flagged patients
   - GET `/api/shared/patients/{personal_id}` - Check specific patient

3. **PII Sanitization:**

   - Additional middleware to sanitize personal IDs in logs
   - Regex-based redaction: `19850312-2398` → `19850312-****`

4. **Environment Configuration:**
   ```env
   DATABASE_URL=postgresql://postgres@localhost/hospital_db
   SERVER_PORT=8001
   API_KEY=<key for authenticating police requests>
   ```

---

## Frontend Architecture

### Technology Stack

- **React 18.3.1** - UI library with concurrent rendering
- **TypeScript 5.5.3** - Strict type checking
- **Vite 5.4.0** - Fast build tool with HMR

### Component Structure

```
src/
├── main.tsx              # React entry point (StrictMode)
├── App.tsx               # Root component, layout
├── types.ts              # TypeScript type definitions
├── components/
│   ├── PoliceData.tsx    # Suspects display + flag toggle
│   └── HospitalData.tsx  # Patients display + auto-refresh
└── hooks/
    └── usePolling.ts     # Custom polling hook
```

### Key Features

#### 1. **API Proxy Configuration (Vite)**

```typescript
// vite.config.ts
export default defineConfig({
  plugins: [react()],
  server: {
    port: 3000,
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
    },
  },
});
```

**Benefits:**

- Avoids CORS issues during development
- Simplified API calls from frontend
- Cookie handling support

#### 2. **Type Definitions (types.ts)**

**Core Types:**

```typescript
export interface Suspect {
  id: number;
  full_name: string | null;
  personal_id: string | null; // YYYYMMDD-XXXX
  flag: boolean | null;
}

export interface Patient {
  id: number;
  full_name: string | null;
  personal_id: string | null; // YYYYMMDD-XXXX
  flag: boolean | null;
}

export interface FlagUpdateRequest {
  personal_id: string;
  flag: boolean;
}
```

**Benefits:**

- Compile-time type checking
- IntelliSense support
- Matches backend Rust models

#### 3. **Police Data Component**

**Features:**

- Displays all suspects in a table
- Flag toggle buttons per row
- Optimistic UI updates
- Error handling with user feedback
- One-time fetch on mount

**Key Implementation:**

```typescript
const toggleFlag = async (suspect: Suspect) => {
  if (!suspect.personal_id) {
    setToggleError("Cannot toggle flag: personal_id is missing");
    return;
  }

  const newFlagValue = !suspect.flag;

  try {
    setTogglingId(suspect.personal_id);
    setToggleError(null);

    const flagUpdate: FlagUpdateRequest = {
      personal_id: suspect.personal_id,
      flag: newFlagValue,
    };

    const response = await fetch("/api/police/suspects/flag", {
      method: "POST",
      credentials: "include",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(flagUpdate),
    });

    if (!response.ok) {
      throw new Error(`Failed to update flag: ${response.status}`);
    }

    const updatedSuspect: Suspect = await response.json();

    // Update local state
    setSuspects((prevSuspects) =>
      prevSuspects.map((s) =>
        s.personal_id === suspect.personal_id ? updatedSuspect : s
      )
    );
  } catch (err) {
    setToggleError(`Failed to toggle flag: ${err.message}`);
  } finally {
    setTogglingId(null);
  }
};
```

#### 4. **Hospital Data Component**

**Features:**

- Displays all patients in a table
- Auto-refresh polling (3 second interval)
- Exponential backoff on errors
- Pause when tab inactive
- Loading state indicators

**Key Implementation:**

Uses custom `usePolling` hook:

```typescript
const {
  data: patients,
  loading,
  error,
  isRefreshing,
} = usePolling<Patient[]>(fetchPatients, {
  enabled: true,
  interval: 3000,
  pauseOnInactive: true,
});
```

#### 5. **Custom Polling Hook**

**Features:**

- Configurable polling interval
- Exponential backoff on consecutive errors
- Pause/resume based on tab visibility
- Initial fetch + subsequent polls
- Loading vs refreshing states

**Error Handling:**

```typescript
const getCurrentInterval = (): number => {
  if (consecutiveErrors === 0) {
    return interval;
  }

  // Exponential backoff: baseInterval * 2^errorCount
  const backoffInterval = interval * Math.pow(2, consecutiveErrors);

  // Cap at maxBackoffInterval (default: 60s)
  return Math.min(backoffInterval, maxBackoffInterval);
};
```

**Visibility Handling:**

```typescript
useEffect(() => {
  const handleVisibilityChange = () => {
    isPageVisible.current = !document.hidden;

    if (pauseOnInactive) {
      if (document.hidden) {
        stopPolling();
      } else {
        void fetchData(true); // Refresh immediately when tab becomes visible
        restartPolling();
      }
    }
  };

  document.addEventListener("visibilitychange", handleVisibilityChange);
  return () => {
    document.removeEventListener("visibilitychange", handleVisibilityChange);
  };
}, [pauseOnInactive, consecutiveErrors]);
```

#### 6. **User Experience Features**

**Loading States:**

- Initial load: "Loading police/hospital data..."
- Refresh: "(refreshing...)" indicator
- Button states: "Updating..." during toggle

**Error Handling:**

- Display error messages to user
- Correlation IDs for support
- Retry logic with backoff

**Responsive Design:**

- Simple HTML tables (no framework)
- Minimal inline styling
- Mobile-friendly (basic)

### Current Limitations

- **Read-only (hospital):** No create/update/delete for patients
- **No routing:** Single-page application
- **No authentication UI:** API keys handled by backend
- **Basic styling:** Plain HTML tables, no CSS framework
- **No real-time WebSockets:** Uses polling instead

---

## Database Architecture

### Schema Design

**Design Principles:**

- Identical table structure in both databases
- Swedish personal ID as unique identifier
- Simple 4-field schema for proof-of-concept
- Boolean flag for synchronization

**Police Database (police_db):**

```sql
CREATE TABLE suspects (
    id SERIAL PRIMARY KEY,          -- Auto-incrementing primary key
    full_name TEXT,                 -- Swedish name
    personal_id TEXT UNIQUE,        -- YYYYMMDD-XXXX format, unique constraint
    flag BOOLEAN                    -- Flag status (triggers sync)
);
```

**Hospital Database (hospital_db):**

```sql
CREATE TABLE patients (
    id SERIAL PRIMARY KEY,          -- Auto-incrementing primary key
    full_name TEXT,                 -- Swedish name
    personal_id TEXT UNIQUE,        -- YYYYMMDD-XXXX format, unique constraint
    flag BOOLEAN                    -- Flag status (updated from police)
);
```

### Foreign Data Wrapper (FDW) Configuration

**Purpose:** Allow police database to directly read and write to hospital's `patients` table.

**Setup Steps:**

1. **Enable Extension:**

   ```sql
   CREATE EXTENSION IF NOT EXISTS postgres_fdw;
   ```

2. **Create Server Connection:**

   ```sql
   CREATE SERVER hospital_server
       FOREIGN DATA WRAPPER postgres_fdw
       OPTIONS (dbname 'hospital_db', host 'localhost');
   ```

3. **Create User Mapping:**

   ```sql
   CREATE USER MAPPING FOR CURRENT_USER
       SERVER hospital_server
       OPTIONS (user 'postgres', password '');
   ```

4. **Import Foreign Schema:**
   ```sql
   IMPORT FOREIGN SCHEMA public
       LIMIT TO (patients)
       FROM SERVER hospital_server
       INTO public;
   ```

**Result:** Police database can now query `SELECT * FROM patients` and `UPDATE patients`.

### Automatic Flag Synchronization

**Trigger Function:**

```sql
CREATE OR REPLACE FUNCTION sync_flag_to_hospital()
RETURNS TRIGGER AS $$
BEGIN
    -- Update matching patient record in hospital database
    UPDATE patients
    SET flag = NEW.flag
    WHERE personal_id = NEW.personal_id;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;
```

**Trigger Definition:**

```sql
CREATE TRIGGER trg_sync_flag
    AFTER UPDATE OF flag ON suspects
    FOR EACH ROW
    EXECUTE FUNCTION sync_flag_to_hospital();
```

**Behavior:**

1. Police updates `suspects.flag` for a specific `personal_id`
2. Trigger fires immediately after the UPDATE
3. Function executes `UPDATE patients SET flag = NEW.flag WHERE personal_id = NEW.personal_id`
4. Hospital database is updated in the same transaction

**Advantages:**

- **Immediate Sync:** No delay, happens in same transaction
- **Transactional Consistency:** If police UPDATE fails, hospital UPDATE doesn't happen
- **No API Coordination:** No need for HTTP calls between systems
- **Simple Logic:** Database handles synchronization automatically

### Data Flow Diagram

```
┌─────────────────────────────────────────────────────────┐
│ 1. Police Backend                                       │
│    POST /suspects/flag                                  │
│    {personal_id: "19850312-2398", flag: true}          │
└───────────────────────┬─────────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────────┐
│ 2. Police Database (police_db)                          │
│    UPDATE suspects SET flag = true                      │
│    WHERE personal_id = '19850312-2398'                  │
└───────────────────────┬─────────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────────┐
│ 3. Trigger: trg_sync_flag fires                         │
│    Executes: sync_flag_to_hospital()                    │
└───────────────────────┬─────────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────────┐
│ 4. Foreign Data Wrapper (postgres_fdw)                  │
│    Translates local UPDATE to remote UPDATE             │
└───────────────────────┬─────────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────────┐
│ 5. Hospital Database (hospital_db)                      │
│    UPDATE patients SET flag = true                      │
│    WHERE personal_id = '19850312-2398'                  │
└───────────────────────┬─────────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────────┐
│ 6. Frontend Polling                                     │
│    GET /api/hospital/patients                           │
│    Sees updated flag in response (within 3 seconds)     │
└─────────────────────────────────────────────────────────┘
```

### Sample Data

**Seed Data Characteristics:**

- **8 shared records** - Present in both police and hospital databases
- **2 police-only records** - Simon Nyberg and Carina Dahl (suspects without hospital records)
- **Swedish names and personal IDs**
- **Mixed flag statuses** for testing synchronization

**Example Records:**

| Full Name      | Personal ID   | Flag  | In Police | In Hospital |
| -------------- | ------------- | ----- | --------- | ----------- |
| Erik Andersson | 19850312-2398 | false | ✓         | ✓           |
| Anna Karlsson  | 19900204-1457 | true  | ✓         | ✓           |
| Simon Nyberg   | 19930808-4417 | true  | ✓         | ✗           |
| Carina Dahl    | 19870527-6675 | false | ✓         | ✗           |

### Database Performance Considerations

**Indexes:**

- `personal_id` has UNIQUE constraint (automatic index)
- Consider additional index on `flag` for flagged patient queries
- SERIAL primary keys are indexed by default

**Connection Pooling:**

- Max 5 connections per backend service
- SQLx manages pool automatically
- Connections reused efficiently

**Query Optimization:**

- Prepared statements via SQLx
- Compile-time query verification
- Async/await for non-blocking I/O

---

## Security Architecture

### Overview

Security is implemented in layers across the entire stack:

1. **Transport Layer:** TLS/HTTPS encryption
2. **Application Layer:** API key authentication, rate limiting
3. **Data Layer:** Input validation, sanitized logging
4. **Audit Layer:** Comprehensive logging of sensitive operations

### 1. Transport Layer Security (TLS)

**Configuration:**

```env
ENABLE_TLS=true
TLS_CERT_PATH=/path/to/cert.pem
TLS_KEY_PATH=/path/to/key.pem
```

**Implementation:**

```rust
if enable_tls {
    let tls_config = load_tls_config()?;

    server
        .bind_rustls_021(&server_address, tls_config)?
        .run()
        .await?;
} else {
    log::warn!("⚠️  TLS is DISABLED - This is only acceptable in development!");

    server
        .bind(&server_address)?
        .run()
        .await?;
}
```

**Certificate Management:**

**Development (Self-Signed):**

```bash
openssl req -x509 -newkey rsa:4096 -nodes \
  -keyout backend/police-system/key.pem \
  -out backend/police-system/cert.pem \
  -days 365 -subj "/CN=localhost"
```

**Production:**

- Use Let's Encrypt for free certificates
- Automated renewal with certbot
- Store certificates securely (not in git)

**Security Headers (TLS-Specific):**

```rust
if enable_tls {
    security_headers = security_headers.add((
        "Strict-Transport-Security",
        "max-age=31536000; includeSubDomains; preload"
    ));
}
```

### 2. Authentication Layer

**API Key Authentication:**

**Key Generation:**

```bash
# Generate secure 32-byte hex key
openssl rand -hex 32
# Output: a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6q7r8s9t0u1v2w3x4y5z6
```

**Configuration:**

```env
# Police System
API_KEY=<key for authenticating hospital requests>
HOSPITAL_API_KEY=<key for calling hospital API>

# Hospital System
API_KEY=<key for authenticating police requests>
```

**Middleware Implementation:**

```rust
pub struct ApiKeyAuth {
    api_key: String,
}

impl<S, B> Service<ServiceRequest> for ApiKeyAuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let api_key = self.api_key.clone();

        let provided_key = req.headers()
            .get("X-API-Key")
            .and_then(|h| h.to_str().ok())
            .map(|s| s.to_string());

        match provided_key {
            Some(key) if constant_time_eq(key.as_bytes(), api_key.as_bytes()) => {
                // Valid - proceed
                let fut = self.service.call(req);
                Box::pin(async move {
                    let res = fut.await?;
                    Ok(res.map_into_boxed_body())
                })
            }
            Some(_) => {
                // Invalid key
                Box::pin(async move {
                    Ok(req.into_response(
                        HttpResponse::Unauthorized()
                            .json(json!({"error": "Invalid API key"}))
                            .map_into_boxed_body()
                    ))
                })
            }
            None => {
                // Missing key
                Box::pin(async move {
                    Ok(req.into_response(
                        HttpResponse::Unauthorized()
                            .json(json!({"error": "API key required"}))
                            .map_into_boxed_body()
                    ))
                })
            }
        }
    }
}
```

**Security Features:**

- **Constant-Time Comparison:** Prevents timing side-channel attacks
- **No Key Logging:** API keys never appear in logs (hashed if needed)
- **Per-Route Application:** Only `/api/shared/*` requires authentication
- **Clear Error Messages:** Client knows what's wrong without revealing details

**Usage Example:**

```bash
# Authenticated request from hospital to police
curl http://localhost:8000/api/shared/suspects/19850312-2398 \
  -H "X-API-Key: a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6"
```

### 3. Rate Limiting

**Two-Tier System:**

**Tier 1: General Endpoints (IP-Based)**

```rust
let general_rate_limiter = GovernorConfigBuilder::default()
    .per_second(requests_per_second)  // e.g., 10
    .burst_size(burst_size)           // e.g., 20
    .finish()
    .unwrap();
```

**Purpose:**

- Protect public endpoints from abuse
- Allow burst traffic for normal use
- Key: Client IP address

**Tier 2: Shared API Endpoints (API-Key-Based)**

```rust
// Custom key extractor
pub struct ApiKeyExtractor;

impl KeyExtractor for ApiKeyExtractor {
    type Key = String;

    fn extract(&self, req: &ServiceRequest) -> Result<Self::Key, Self::KeyExtractionError> {
        let api_key = req.headers()
            .get("X-API-Key")
            .and_then(|h| h.to_str().ok())
            .ok_or_else(|| ApiKeyError::new("Missing X-API-Key header"))?;

        // Hash API key for privacy (don't store actual key)
        let mut hasher = Sha256::new();
        hasher.update(api_key.as_bytes());
        let hash = format!("{:x}", hasher.finalize());

        Ok(hash[..32].to_string())
    }
}

let shared_api_rate_limiter = GovernorConfigBuilder::default()
    .per_second(shared_api_rate_limit_per_second)  // e.g., 1
    .burst_size(shared_api_rate_limit_burst)       // e.g., 5
    .key_extractor(ApiKeyExtractor)
    .finish()
    .unwrap();
```

**Purpose:**

- Stricter limits for sensitive cross-system queries
- Per-API-key tracking (not per-IP)
- Prevents abuse even with valid credentials

**Configuration:**

```env
# General rate limiting
RATE_LIMIT_PER_SECOND=10
RATE_LIMIT_BURST=20

# Shared API rate limiting (stricter)
SHARED_API_RATE_LIMIT_PER_SECOND=1
SHARED_API_RATE_LIMIT_BURST=5
```

**Response on Rate Limit:**

```json
HTTP/1.1 429 Too Many Requests
Retry-After: 5

{
  "error": "Too many requests"
}
```

### 4. CORS (Cross-Origin Resource Sharing)

**Configuration:**

```rust
let mut cors = Cors::default()
    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
    .allowed_headers(vec![
        header::CONTENT_TYPE,
        header::AUTHORIZATION,
        HeaderName::from_static("x-api-key"),
    ])
    .expose_headers(vec![
        header::CONTENT_TYPE,
    ])
    .max_age(3600)
    .supports_credentials();

// Add each allowed origin explicitly (no wildcards)
for origin in &allowed_origins {
    cors = cors.allowed_origin(origin);
}
```

**Environment Configuration:**

```env
# Development
ALLOWED_ORIGINS=http://localhost:8000,http://localhost:8001,http://localhost:3000

# Production (HTTPS only)
ALLOWED_ORIGINS=https://police.example.com,https://hospital.example.com,https://frontend.example.com
```

**Production Validation:**

```rust
if !cfg!(debug_assertions) {
    for origin in &allowed_origins {
        if origin.starts_with("http://") {
            panic!("Production requires HTTPS origins only");
        }
        if origin.contains("localhost") || origin.contains("127.0.0.1") {
            panic!("Production cannot use localhost origins");
        }
    }
}
```

### 5. Input Validation

**Swedish Personal ID Validation:**

```rust
impl Suspect {
    pub fn validate_personal_id(personal_id: &str) -> bool {
        if personal_id.len() != 13 {
            return false;
        }

        let parts: Vec<&str> = personal_id.split('-').collect();
        if parts.len() != 2 {
            return false;
        }

        // Check date part (YYYYMMDD)
        if parts[0].len() != 8 || !parts[0].chars().all(|c| c.is_numeric()) {
            return false;
        }

        // Check suffix (XXXX)
        if parts[1].len() != 4 || !parts[1].chars().all(|c| c.is_numeric()) {
            return false;
        }

        true
    }
}
```

**Usage in Endpoints:**

```rust
async fn create_suspect(
    pool: web::Data<PgPool>,
    suspect: web::Json<CreateSuspect>,
) -> HttpResponse {
    let suspect_data = suspect.into_inner();

    // Validate before processing
    if !Suspect::validate_personal_id(&suspect_data.personal_id) {
        return handle_validation_error(
            "Invalid personal_id format",
            "create_suspect"
        );
    }

    // Proceed with creation...
}
```

**SQL Injection Prevention:**

SQLx provides compile-time query verification and automatic parameter binding:

```rust
// Safe - parameters are bound, not concatenated
let suspect = sqlx::query_as!(
    Suspect,
    "SELECT id, full_name, personal_id, flag FROM suspects WHERE personal_id = $1",
    personal_id  // Safely bound parameter
)
.fetch_optional(pool)
.await?;
```

### 6. Sanitized Logging

**Personal Data Hashing:**

```rust
pub fn hash_for_logging(personal_id: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(personal_id.as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)[..16].to_string()
}
```

**Usage:**

```rust
log::info!(
    "Processing request for personal_id hash: {}",
    hash_for_logging(&personal_id)
);
```

**Regex-Based Redaction (Hospital System):**

```rust
lazy_static! {
    static ref PERSONAL_ID_REGEX: Regex = Regex::new(r"\d{8}-\d{4}").unwrap();
}

pub fn sanitize_personal_id(message: &str) -> String {
    PERSONAL_ID_REGEX.replace_all(message, |caps: &regex::Captures| {
        let full_match = &caps[0];
        let date_part = &full_match[..8];
        format!("{}-****", date_part)
    }).to_string()
}
```

**Example:**

```
Before: Patient 19850312-2398 checked in
After:  Patient 19850312-**** checked in
```

### 7. Audit Logging

**Comprehensive Event Tracking:**

```rust
AuditLog::new(
    EventType::FlagUpdate,
    "internal",
    Action::Update,
    format!("suspect:{}", hash_for_logging(&personal_id)),
    AuditResult::Success,
)
.with_ip(req.peer_addr().map(|a| a.ip()))
.with_details(format!("Flag updated to {}", flag))
.write();
```

**Logged Information:**

- Timestamp (ISO 8601)
- Event type (flag update, API access, etc.)
- Actor (hashed API key or "internal")
- Action (read, create, update, delete)
- Resource (hashed identifier)
- Result (success/failure)
- IP address
- Additional details

**Storage:**

Logs are written to the `audit` log target and can be configured in `env_logger`:

```env
RUST_LOG=info,audit=info
```

**Example Audit Log:**

```json
{
  "timestamp": "2025-01-15T10:30:45Z",
  "event_type": "FLAG_UPDATE",
  "actor": "internal",
  "action": "UPDATE",
  "resource": "suspect:a3f5e8d2b1c4",
  "result": "SUCCESS",
  "ip_address": "127.0.0.1",
  "details": "Flag updated to true"
}
```

### 8. Security Headers

**Comprehensive HTTP Security Headers:**

```rust
let mut security_headers = DefaultHeaders::new()
    // Prevent MIME type sniffing
    .add(("X-Content-Type-Options", "nosniff"))

    // Prevent clickjacking
    .add(("X-Frame-Options", "DENY"))

    // Enable XSS protection (legacy browsers)
    .add(("X-XSS-Protection", "1; mode=block"))

    // Restrict resource loading
    .add(("Content-Security-Policy", "default-src 'none'"))

    // Control referrer information
    .add(("Referrer-Policy", "no-referrer"))

    // Disable browser features
    .add(("Permissions-Policy", "geolocation=(), microphone=(), camera=()"));

// Add HSTS only when TLS is enabled
if enable_tls {
    security_headers = security_headers.add((
        "Strict-Transport-Security",
        "max-age=31536000; includeSubDomains; preload"
    ));
}
```

**Benefits:**

- **X-Content-Type-Options:** Prevents MIME confusion attacks
- **X-Frame-Options:** Protects against clickjacking
- **X-XSS-Protection:** Legacy XSS protection
- **Content-Security-Policy:** Restricts content sources
- **Referrer-Policy:** Protects referrer information
- **Permissions-Policy:** Disables unnecessary browser features
- **Strict-Transport-Security:** Forces HTTPS (when TLS enabled)

### 9. Error Handling with Correlation IDs

**Unique Error Tracking:**

```rust
pub fn handle_database_error<E: std::fmt::Display>(error: E, context: &str) -> HttpResponse {
    let correlation_id = Uuid::new_v4().to_string();

    // Log full error server-side
    log::error!(
        "Database error [{}] in {}: {}",
        correlation_id,
        context,
        error
    );

    // Return generic error to client
    HttpResponse::InternalServerError().json(json!({
        "error": "Service temporarily unavailable",
        "correlation_id": correlation_id
    }))
}
```

**Client Response:**

```json
{
  "error": "Service temporarily unavailable",
  "correlation_id": "550e8400-e29b-41d4-a716-446655440000"
}
```

**Benefits:**

- Clients never see sensitive error details
- Support can trace issues using correlation ID
- Full error context preserved in server logs

---

## Development Workflow

### Starting Development

```bash
# 1. Start PostgreSQL
brew services start postgresql@15

# 2. Start police backend
cd backend/police-system
cargo run
# Logs: Server running at http://localhost:8000

# 3. Start hospital backend (new terminal)
cd backend/hospital-system
cargo run
# Logs: Server running at http://localhost:8001

# 4. Start frontend (new terminal)
cd frontend
npm run dev
# Logs: Server running at http://localhost:3000
```

### Testing Workflow

**Manual API Testing:**

```bash
# Health checks
curl http://localhost:8000/health
curl http://localhost:8001/health

# Get data
curl http://localhost:8000/suspects
curl http://localhost:8001/patients

# Test flag sync
curl -X POST http://localhost:8000/suspects/flag \
  -H "Content-Type: application/json" \
  -d '{"personal_id": "19850312-2398", "flag": true}'

# Verify sync (should show flag=true)
curl http://localhost:8001/patients/personal/19850312-2398

# Test authenticated endpoint
curl http://localhost:8000/api/shared/suspects/19850312-2398 \
  -H "X-API-Key: your-api-key-here"
```

**Security Testing:**

```bash
# Run nmap scans
cd testing/nmap
./run-all-scans.sh

# View results
ls -lh nmap-results/scan_*/

# OWASP ZAP (see testing/zap/instructions.md)
brew install --cask owasp-zap
open /Applications/ZAP.app
```

### Database Workflow

```bash
# Connect to police database
psql -U postgres -d police_db

# View suspects
SELECT * FROM suspects;

# Update flag (triggers sync)
UPDATE suspects SET flag = true WHERE personal_id = '19850312-2398';

# Check foreign table
SELECT * FROM patients WHERE personal_id = '19850312-2398';

# Connect to hospital database
\c hospital_db

# Verify sync
SELECT * FROM patients WHERE personal_id = '19850312-2398';
```

### Common Development Tasks

**Rebuild Rust binaries:**

```bash
cd backend/police-system
cargo build --release
```

**Reset databases:**

```bash
psql -U postgres -c "DROP DATABASE IF EXISTS police_db;"
psql -U postgres -c "DROP DATABASE IF EXISTS hospital_db;"
psql -U postgres -f shared/database-schemas/schema.sql
psql -U postgres -f shared/database-schemas/seed-data.sql
```

**Generate new API keys:**

```bash
openssl rand -hex 32
```

**Generate TLS certificates:**

```bash
cd backend/police-system
openssl req -x509 -newkey rsa:4096 -nodes \
  -keyout key.pem -out cert.pem \
  -days 365 -subj "/CN=localhost"
```

**View logs:**

```bash
# Enable debug logging
export RUST_LOG=debug,sqlx=info
cargo run

# Audit logs only
export RUST_LOG=audit=info
cargo run
```

---

## Contributing

This is an academic research project. If you wish to contribute:

1. **Follow Rust style guidelines:** Run `cargo fmt` before committing
2. **Add tests for new functionality:** Unit tests in relevant modules
3. **Update documentation:** Keep this file and README.md in sync
4. **Ensure all tests pass:** Run `cargo test` before submitting
5. **Use meaningful commit messages:** Describe what and why, not how
6. **Update ARCHITECTURE.md for significant changes**

---

## References

### Documentation

- [PostgreSQL Foreign Data Wrapper](https://www.postgresql.org/docs/current/postgres-fdw.html)
- [Actix-web Documentation](https://actix.rs/)
- [SQLx Documentation](https://github.com/launchbadge/sqlx)
- [React Documentation](https://react.dev/)
- [Vite Documentation](https://vitejs.dev/)
- [rustls Documentation](https://docs.rs/rustls/)

### Security Resources

- [OWASP Top 10](https://owasp.org/www-project-top-ten/)
- [OWASP ZAP User Guide](https://www.zaproxy.org/docs/)
- [nmap Reference Guide](https://nmap.org/book/man.html)
- [Mozilla Security Headers](https://infosec.mozilla.org/guidelines/web_security)

### Research Context

- **Institution:** Yrgo, Higher Vocational Education, Gothenburg
- **Program:** Web Development
- **Thesis Topic:** Security Analysis of AI-Generated Code
- **Status:** All research activities are concluded and the project is finished.
