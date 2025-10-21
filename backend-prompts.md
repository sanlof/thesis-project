# Backend Code Generation Prompts

Use these prompts sequentially to generate the complete backend for your thesis project. Each prompt builds upon previous work.

---

## Phase 1: Foundation Setup

### Prompt 1: Police System Cargo.toml

```
Create a complete Cargo.toml file for a Rust backend service with the following requirements:

Package name: police-system
Version: 0.1.0
Edition: 2021

Required dependencies:
- actix-web = "4" (web framework)
- actix-cors = "0.7" (CORS support)
- sqlx with features: ["runtime-tokio-native-tls", "postgres", "macros"]
- tokio with features: ["full"]
- serde with features: ["derive"]
- serde_json
- dotenv = "0.15"
- env_logger = "0.11"
- log = "0.4"

The service will run on port 8000 and connect to a PostgreSQL database.
```

### Prompt 2: Hospital System Cargo.toml

```
Create a complete Cargo.toml file for a Rust backend service with the following requirements:

Package name: hospital-system
Version: 0.1.0
Edition: 2021

Required dependencies:
- actix-web = "4" (web framework)
- actix-cors = "0.7" (CORS support)
- sqlx with features: ["runtime-tokio-native-tls", "postgres", "macros"]
- tokio with features: ["full"]
- serde with features: ["derive"]
- serde_json
- dotenv = "0.15"
- env_logger = "0.11"
- log = "0.4"

The service will run on port 8001 and connect to a PostgreSQL database.
```

---

## Phase 2: Data Models

### Prompt 3: Police System - Suspect Model

```
Create a Rust model file for a suspect with the following requirements:

File: backend/police-system/src/models/suspect.rs

Database table structure:
- id: SERIAL PRIMARY KEY
- full_name: TEXT
- personal_id: TEXT UNIQUE
- flag: BOOLEAN

Requirements:
1. Use serde for JSON serialization/deserialization
2. Derive Debug, Clone, Serialize, Deserialize
3. Use sqlx::FromRow for database mapping
4. Create a struct called Suspect with all fields
5. Create a CreateSuspect struct for POST requests (without id)
6. Create an UpdateSuspect struct for PUT requests (all fields optional except personal_id)
7. Add appropriate Option<T> types for nullable fields

Make sure the personal_id follows Swedish format (YYYYMMDD-XXXX).
```

### Prompt 4: Hospital System - Patient Model

```
Create a Rust model file for a patient with the following requirements:

File: backend/hospital-system/src/models/patient.rs

Database table structure:
- id: SERIAL PRIMARY KEY
- full_name: TEXT
- personal_id: TEXT UNIQUE
- flag: BOOLEAN

Requirements:
1. Use serde for JSON serialization/deserialization
2. Derive Debug, Clone, Serialize, Deserialize
3. Use sqlx::FromRow for database mapping
4. Create a struct called Patient with all fields
5. Create a CreatePatient struct for POST requests (without id)
6. Create an UpdatePatient struct for PUT requests (all fields optional except personal_id)
7. Add appropriate Option<T> types for nullable fields

Make sure the personal_id follows Swedish format (YYYYMMDD-XXXX).
```

### Prompt 5: Police System - Models Module

```
Create the models module file for the police system.

File: backend/police-system/src/models/mod.rs

Requirements:
- Declare the suspect module
- Re-export all structs from suspect module (Suspect, CreateSuspect, UpdateSuspect)

Keep it simple and idiomatic.
```

### Prompt 6: Hospital System - Models Module

```
Create the models module file for the hospital system.

File: backend/hospital-system/src/models/mod.rs

Requirements:
- Declare the patient module
- Re-export all structs from patient module (Patient, CreatePatient, UpdatePatient)

Keep it simple and idiomatic.
```

---

## Phase 3: Database Layer

### Prompt 7: Police System - Database Connection

```
Create a database connection module for the police system with PostgreSQL.

File: backend/police-system/src/database/connection.rs

Requirements:
1. Use sqlx::postgres::PgPool
2. Create a public async function called establish_connection() that:
   - Reads DATABASE_URL from environment variables
   - Creates a connection pool with PgPool::connect()
   - Returns Result<PgPool, sqlx::Error>
   - Includes error handling with informative messages
3. Add proper logging using the log crate
4. The function should be reusable and called from main.rs

Database name: police_db
Environment variable: DATABASE_URL
```

### Prompt 8: Hospital System - Database Connection

```
Create a database connection module for the hospital system with PostgreSQL.

File: backend/hospital-system/src/database/connection.rs

Requirements:
1. Use sqlx::postgres::PgPool
2. Create a public async function called establish_connection() that:
   - Reads DATABASE_URL from environment variables
   - Creates a connection pool with PgPool::connect()
   - Returns Result<PgPool, sqlx::Error>
   - Includes error handling with informative messages
3. Add proper logging using the log crate
4. The function should be reusable and called from main.rs

Database name: hospital_db
Environment variable: DATABASE_URL
```

### Prompt 9: Police System - Database Queries

```
Create database query functions for the police system.

File: backend/police-system/src/database/queries.rs

Database table: suspects (id, full_name, personal_id, flag)

Requirements:
1. Import necessary types (PgPool, Suspect, CreateSuspect, UpdateSuspect)
2. Implement these async functions:
   - get_all_suspects(pool: &PgPool) -> Result<Vec<Suspect>, sqlx::Error>
   - get_suspect_by_id(pool: &PgPool, id: i32) -> Result<Option<Suspect>, sqlx::Error>
   - get_suspect_by_personal_id(pool: &PgPool, personal_id: &str) -> Result<Option<Suspect>, sqlx::Error>
   - create_suspect(pool: &PgPool, suspect: CreateSuspect) -> Result<Suspect, sqlx::Error>
   - update_suspect(pool: &PgPool, id: i32, suspect: UpdateSuspect) -> Result<Option<Suspect>, sqlx::Error>
   - delete_suspect(pool: &PgPool, id: i32) -> Result<bool, sqlx::Error>
   - update_flag(pool: &PgPool, personal_id: &str, flag: bool) -> Result<Option<Suspect>, sqlx::Error>

3. Use sqlx::query_as! macro for type-safe queries
4. Return None for not found cases, not errors
5. Add proper error handling
```

### Prompt 10: Hospital System - Database Queries

```
Create database query functions for the hospital system.

File: backend/hospital-system/src/database/queries.rs

Database table: patients (id, full_name, personal_id, flag)

Requirements:
1. Import necessary types (PgPool, Patient, CreatePatient, UpdatePatient)
2. Implement these async functions:
   - get_all_patients(pool: &PgPool) -> Result<Vec<Patient>, sqlx::Error>
   - get_patient_by_id(pool: &PgPool, id: i32) -> Result<Option<Patient>, sqlx::Error>
   - get_patient_by_personal_id(pool: &PgPool, personal_id: &str) -> Result<Option<Patient>, sqlx::Error>
   - create_patient(pool: &PgPool, patient: CreatePatient) -> Result<Patient, sqlx::Error>
   - update_patient(pool: &PgPool, id: i32, patient: UpdatePatient) -> Result<Option<Patient>, sqlx::Error>
   - delete_patient(pool: &PgPool, id: i32) -> Result<bool, sqlx::Error>
   - get_flagged_patients(pool: &PgPool) -> Result<Vec<Patient>, sqlx::Error>

3. Use sqlx::query_as! macro for type-safe queries
4. Return None for not found cases, not errors
5. Add proper error handling
```

### Prompt 11: Police System - Database Module

```
Create the database module file for the police system.

File: backend/police-system/src/database/mod.rs

Requirements:
- Declare connection and queries modules
- Re-export the establish_connection function from connection
- Re-export all query functions from queries module

Keep it clean and organized.
```

### Prompt 12: Hospital System - Database Module

```
Create the database module file for the hospital system.

File: backend/hospital-system/src/database/mod.rs

Requirements:
- Declare connection and queries modules
- Re-export the establish_connection function from connection
- Re-export all query functions from queries module

Keep it clean and organized.
```

---

## Phase 4: API Endpoints

### Prompt 13: Police System - Suspects API Endpoints

```
Create REST API endpoints for suspect management in the police system.

File: backend/police-system/src/api/suspects.rs

Requirements:
1. Use actix_web (HttpResponse, web::Data, web::Json, web::Path)
2. Use sqlx::PgPool as shared state
3. Implement these handler functions:

   GET /suspects - get_all_suspects
   GET /suspects/{id} - get_suspect_by_id
   GET /suspects/personal/{personal_id} - get_suspect_by_personal_id
   POST /suspects - create_suspect (accepts CreateSuspect JSON)
   PUT /suspects/{id} - update_suspect (accepts UpdateSuspect JSON)
   DELETE /suspects/{id} - delete_suspect
   PUT /suspects/{personal_id}/flag - update_flag (accepts JSON with flag: bool)

4. Return appropriate HTTP status codes:
   - 200 OK for successful GET/PUT
   - 201 Created for POST
   - 204 No Content for DELETE
   - 404 Not Found when resource doesn't exist
   - 500 Internal Server Error for database errors

5. Include error logging
6. Create a public function configure_suspects(cfg: &mut web::ServiceConfig) to register all routes
```

### Prompt 14: Hospital System - Patients API Endpoints

```
Create REST API endpoints for patient management in the hospital system.

File: backend/hospital-system/src/api/patients.rs

Requirements:
1. Use actix_web (HttpResponse, web::Data, web::Json, web::Path)
2. Use sqlx::PgPool as shared state
3. Implement these handler functions:

   GET /patients - get_all_patients
   GET /patients/{id} - get_patient_by_id
   GET /patients/personal/{personal_id} - get_patient_by_personal_id
   POST /patients - create_patient (accepts CreatePatient JSON)
   PUT /patients/{id} - update_patient (accepts UpdatePatient JSON)
   DELETE /patients/{id} - delete_patient
   GET /patients/flagged - get_flagged_patients

4. Return appropriate HTTP status codes:
   - 200 OK for successful GET/PUT
   - 201 Created for POST
   - 204 No Content for DELETE
   - 404 Not Found when resource doesn't exist
   - 500 Internal Server Error for database errors

5. Include error logging
6. Create a public function configure_patients(cfg: &mut web::ServiceConfig) to register all routes
```

### Prompt 15: Police System - Shared API Endpoints

```
Create shared/inter-system API endpoints for the police system.

File: backend/police-system/src/api/shared.rs

Requirements:
1. These endpoints are for the hospital system to query police data
2. Use actix_web and sqlx::PgPool
3. Implement these handler functions:

   GET /api/shared/suspects/{personal_id} - get_shared_suspect_info
   GET /api/shared/suspects - get_all_shared_suspects

4. Return suspect data in JSON format
5. Return 404 if not found, 500 for errors
6. Add CORS support for cross-origin requests from localhost:8001
7. Create a public function configure_shared(cfg: &mut web::ServiceConfig)

These endpoints allow the hospital system to check if a patient has a police record.
```

### Prompt 16: Hospital System - Shared API Endpoints

```
Create shared/inter-system API endpoints for the hospital system.

File: backend/hospital-system/src/api/shared.rs

Requirements:
1. These endpoints are for the police system to query hospital data
2. Use actix_web and sqlx::PgPool
3. Implement these handler functions:

   GET /api/shared/patients/{personal_id} - get_shared_patient_info
   GET /api/shared/patients - get_all_shared_patients
   GET /api/shared/patients/flagged - get_shared_flagged_patients

4. Return patient data in JSON format
5. Return 404 if not found, 500 for errors
6. Add CORS support for cross-origin requests from localhost:8000
7. Create a public function configure_shared(cfg: &mut web::ServiceConfig)

These endpoints allow the police system to check if a suspect has medical records.
```

### Prompt 17: Police System - API Module

```
Create the API module file for the police system.

File: backend/police-system/src/api/mod.rs

Requirements:
- Declare suspects and shared modules
- Re-export configure functions from both modules
- Keep it simple and clean
```

### Prompt 18: Hospital System - API Module

```
Create the API module file for the hospital system.

File: backend/hospital-system/src/api/mod.rs

Requirements:
- Declare patients and shared modules
- Re-export configure functions from both modules
- Keep it simple and clean
```

---

## Phase 5: Main Application Files

### Prompt 19: Police System - Main Application

```
Create the main application file for the police system.

File: backend/police-system/src/main.rs

Requirements:
1. Load environment variables using dotenv
2. Initialize env_logger for logging
3. Establish database connection pool
4. Create actix-web HttpServer with:
   - CORS middleware (allow localhost:8001 and all origins for development)
   - Shared PgPool state
   - Suspects routes at /suspects
   - Shared API routes at /api/shared
5. Bind to address from SERVER_PORT env variable (default: 8000)
6. Use actix_web::middleware::Logger for request logging
7. Include proper error handling and informative startup messages

The server should log:
- Database connection status
- Server address
- Available routes

Make it production-ready with proper error handling.
```

### Prompt 20: Hospital System - Main Application

```
Create the main application file for the hospital system.

File: backend/hospital-system/src/main.rs

Requirements:
1. Load environment variables using dotenv
2. Initialize env_logger for logging
3. Establish database connection pool
4. Create actix-web HttpServer with:
   - CORS middleware (allow localhost:8000 and all origins for development)
   - Shared PgPool state
   - Patients routes at /patients
   - Shared API routes at /api/shared
5. Bind to address from SERVER_PORT env variable (default: 8001)
6. Use actix_web::middleware::Logger for request logging
7. Include proper error handling and informative startup messages

The server should log:
- Database connection status
- Server address
- Available routes

Make it production-ready with proper error handling.
```

---

## Phase 6: Testing & Documentation

### Prompt 21: Create API Documentation

```
Create a comprehensive API documentation file for both systems.

File: docs/API.md

Include for both Police System (port 8000) and Hospital System (port 8001):

1. All endpoints with:
   - HTTP method and path
   - Request body schema (JSON)
   - Response body schema (JSON)
   - Status codes
   - Example requests using curl

2. Authentication notes (currently none, future: JWT)

3. Error response format

4. Inter-system communication examples:
   - How hospital queries police records
   - How police queries hospital records

Make it clear, professional, and ready for other developers to use.
```

### Prompt 22: Create Testing Guide

```
Create a testing guide for the backend services.

File: docs/TESTING.md

Include:
1. How to start both services
2. Sample curl commands for all endpoints
3. Test scenarios:
   - Creating a suspect/patient
   - Updating records
   - Testing flag synchronization
   - Cross-system queries
4. Expected responses
5. Common error scenarios
6. How to verify database changes using psql

Make it practical and easy to follow for manual testing.
```

---

## Usage Instructions

1. **Run prompts sequentially** - Each builds on previous code
2. **Save generated code** to the exact file paths specified
3. **Create .env files** by copying .env.example and adding your database password
4. **After all code is generated**, run:

   ```bash
   cd backend/police-system
   cargo build
   cargo run

   # In another terminal
   cd backend/hospital-system
   cargo build
   cargo run
   ```

5. **Test the services** using the curl commands from the testing guide

---

## Tips for Best Results

- Provide each prompt to an AI code assistant one at a time
- Review the generated code before moving to the next prompt
- If you get an error, provide the error message to the AI for fixes
- The prompts are designed to work with Claude, ChatGPT, or similar AI assistants
- Each prompt is self-contained but references the overall architecture

---

## Quick Verification

After generation, verify:

- [ ] All Cargo.toml files compile
- [ ] Database connections work
- [ ] All endpoints return proper responses
- [ ] Flag synchronization works via database triggers
- [ ] CORS allows cross-origin requests
- [ ] Logging shows useful information

---

_Total: 22 prompts to generate complete backend infrastructure_
