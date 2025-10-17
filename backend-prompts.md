# AI Prompts for Backend Code Generation

## Prompt Series 1: Database Layer Implementation

### Prompt 1.1 - Police Database Connection

```
Create a complete Rust database connection module for the police system backend.

Requirements:
- File: backend/police-system/src/database/connection.rs
- Use sqlx with PostgreSQL
- Create a connection pool using PgPoolOptions
- Read DATABASE_URL from environment variables
- Include proper error handling
- Export a function that returns the pool
- Keep it simple and production-ready

Dependencies available: sqlx 0.7 with postgres feature, tokio
```

### Prompt 1.2 - Hospital Database Connection

```
Create a complete Rust database connection module for the hospital system backend.

Requirements:
- File: backend/hospital-system/src/database/connection.rs
- Use sqlx with PostgreSQL
- Create a connection pool using PgPoolOptions
- Read DATABASE_URL from environment variables
- Include proper error handling
- Export a function that returns the pool
- Make it identical in structure to the police system connection module

Dependencies available: sqlx 0.7 with postgres feature, tokio
```

## Prompt Series 2: Data Models

### Prompt 2.1 - Police Models

```
Create Rust data models for the police system.

Requirements:
- File 1: backend/police-system/src/models/case.rs
  - Struct: Case with fields: id (i32), case_number (String), status (String), description (Option<String>), created_at (chrono::NaiveDateTime)
  - Derive: Serialize, Deserialize, sqlx::FromRow

- File 2: backend/police-system/src/models/suspect.rs
  - Struct: Suspect with fields: id (i32), case_id (i32), name (String), personal_id (Option<String>), created_at (chrono::NaiveDateTime)
  - Derive: Serialize, Deserialize, sqlx::FromRow

- File 3: backend/police-system/src/models/mod.rs
  - Export both Case and Suspect structs

Add chrono to Cargo.toml if needed. Use serde for JSON serialization.
```

### Prompt 2.2 - Hospital Models

```
Create Rust data models for the hospital system.

Requirements:
- File 1: backend/hospital-system/src/models/patient.rs
  - Struct: Patient with fields: id (i32), patient_id (String), name (String), personal_id (Option<String>), created_at (chrono::NaiveDateTime)
  - Derive: Serialize, Deserialize, sqlx::FromRow

- File 2: backend/hospital-system/src/models/record.rs
  - Struct: MedicalRecord with fields: id (i32), patient_id (i32), diagnosis (Option<String>), treatment (Option<String>), created_at (chrono::NaiveDateTime)
  - Derive: Serialize, Deserialize, sqlx::FromRow

- File 3: backend/hospital-system/src/models/mod.rs
  - Export both Patient and MedicalRecord structs

Use the same pattern as the police system models.
```

## Prompt Series 3: Database Queries

### Prompt 3.1 - Police Queries

```
Create database query functions for the police system.

Requirements:
- File: backend/police-system/src/database/queries.rs
- Use sqlx for async queries
- Implement these functions:
  1. get_all_cases(pool: &PgPool) -> Result<Vec<Case>, sqlx::Error>
  2. get_case_by_id(pool: &PgPool, id: i32) -> Result<Case, sqlx::Error>
  3. create_case(pool: &PgPool, case_number: String, status: String, description: Option<String>) -> Result<Case, sqlx::Error>
  4. get_suspects_by_case(pool: &PgPool, case_id: i32) -> Result<Vec<Suspect>, sqlx::Error>
  5. create_suspect(pool: &PgPool, case_id: i32, name: String, personal_id: Option<String>) -> Result<Suspect, sqlx::Error>

Use sqlx::query_as! macro for type-safe queries. Include proper error handling.
```

### Prompt 3.2 - Hospital Queries

```
Create database query functions for the hospital system.

Requirements:
- File: backend/hospital-system/src/database/queries.rs
- Use sqlx for async queries
- Implement these functions:
  1. get_all_patients(pool: &PgPool) -> Result<Vec<Patient>, sqlx::Error>
  2. get_patient_by_id(pool: &PgPool, id: i32) -> Result<Patient, sqlx::Error>
  3. create_patient(pool: &PgPool, patient_id: String, name: String, personal_id: Option<String>) -> Result<Patient, sqlx::Error>
  4. get_records_by_patient(pool: &PgPool, patient_id: i32) -> Result<Vec<MedicalRecord>, sqlx::Error>
  5. create_record(pool: &PgPool, patient_id: i32, diagnosis: Option<String>, treatment: Option<String>) -> Result<MedicalRecord, sqlx::Error>

Use the same pattern as police queries with sqlx::query_as! macro.
```

### Prompt 3.3 - Database Module Exports

```
Create mod.rs files to export database modules.

Requirements:
- File 1: backend/police-system/src/database/mod.rs
  - Export connection and queries modules publicly

- File 2: backend/hospital-system/src/database/mod.rs
  - Export connection and queries modules publicly

Keep it minimal - just module declarations and pub use statements.
```

## Prompt Series 4: API Endpoints

### Prompt 4.1 - Police Case API

```
Create REST API endpoints for police case management.

Requirements:
- File: backend/police-system/src/api/cases.rs
- Use actix-web framework
- Implement these endpoints:
  1. GET /api/cases - list all cases
  2. GET /api/cases/{id} - get specific case
  3. POST /api/cases - create new case (accepts JSON: case_number, status, description)

- All handlers should:
  - Accept PgPool as web::Data<PgPool>
  - Call appropriate database queries
  - Return JSON responses
  - Handle errors properly (return 404, 500 as needed)

Use actix-web's web::Json for request/response bodies.
```

### Prompt 4.2 - Police Suspect API

```
Create REST API endpoints for police suspect management.

Requirements:
- File: backend/police-system/src/api/suspects.rs
- Use actix-web framework
- Implement these endpoints:
  1. GET /api/cases/{case_id}/suspects - list suspects for a case
  2. POST /api/cases/{case_id}/suspects - add suspect to case (accepts JSON: name, personal_id)

Follow the same pattern as cases.rs with proper error handling and JSON responses.
```

### Prompt 4.3 - Hospital Patient API

```
Create REST API endpoints for hospital patient management.

Requirements:
- File: backend/hospital-system/src/api/patients.rs
- Use actix-web framework
- Implement these endpoints:
  1. GET /api/patients - list all patients
  2. GET /api/patients/{id} - get specific patient
  3. POST /api/patients - create new patient (accepts JSON: patient_id, name, personal_id)

Use the same structure as police case endpoints.
```

### Prompt 4.4 - Hospital Records API

```
Create REST API endpoints for medical records management.

Requirements:
- File: backend/hospital-system/src/api/records.rs
- Use actix-web framework
- Implement these endpoints:
  1. GET /api/patients/{patient_id}/records - list records for a patient
  2. POST /api/patients/{patient_id}/records - add record (accepts JSON: diagnosis, treatment)

Follow the same pattern as other API modules.
```

## Prompt Series 5: Inter-System Communication

### Prompt 5.1 - Police Shared API

```
Create inter-system API endpoints for the police system to share data with hospital.

Requirements:
- File: backend/police-system/src/api/shared.rs
- Implement endpoint:
  - GET /api/shared/suspect/{personal_id} - lookup suspect by personal ID
  - Should return minimal information: name, personal_id, has_active_cases (boolean)

- This endpoint will be called by the hospital system
- Include basic authentication check (for now, just accept an API key in header)
- Return 404 if not found, 403 if unauthorized

Keep it simple for now - full security will be added later.
```

### Prompt 5.2 - Hospital Shared API

```
Create inter-system API endpoints for the hospital system to share data with police.

Requirements:
- File: backend/hospital-system/src/api/shared.rs
- Implement endpoint:
  - GET /api/shared/patient/{personal_id} - lookup patient by personal ID
  - Should return minimal information: name, personal_id, has_records (boolean)

- This endpoint will be called by the police system
- Include basic authentication check (for now, just accept an API key in header)
- Return 404 if not found, 403 if unauthorized

Mirror the structure of the police shared API.
```

### Prompt 5.3 - API Module Exports

```
Create mod.rs files to export all API modules.

Requirements:
- File 1: backend/police-system/src/api/mod.rs
  - Declare and export: cases, suspects, shared modules

- File 2: backend/hospital-system/src/api/mod.rs
  - Declare and export: patients, records, shared modules

Just module declarations - keep it minimal.
```

## Prompt Series 6: Main Application Setup

### Prompt 6.1 - Police Main Application

```
Complete the main.rs file for the police system backend.

Requirements:
- File: backend/police-system/src/main.rs
- Set up actix-web server on port 8000
- Initialize database connection pool
- Configure CORS (use Cors::permissive() for development)
- Register all API routes:
  - /health (health check endpoint)
  - /api/cases and /api/cases/{id}
  - /api/cases/{case_id}/suspects
  - /api/shared/suspect/{personal_id}

- Load environment variables with dotenv
- Add proper error handling for server startup
- Include logging initialization (env_logger)

Add env_logger to Cargo.toml. Keep the main function clean and well-structured.
```

### Prompt 6.2 - Hospital Main Application

```
Complete the main.rs file for the hospital system backend.

Requirements:
- File: backend/hospital-system/src/main.rs
- Set up actix-web server on port 8001
- Initialize database connection pool
- Configure CORS (use Cors::permissive() for development)
- Register all API routes:
  - /health (health check endpoint)
  - /api/patients and /api/patients/{id}
  - /api/patients/{patient_id}/records
  - /api/shared/patient/{personal_id}

- Load environment variables with dotenv
- Add proper error handling for server startup
- Include logging initialization (env_logger)

Mirror the structure of the police main.rs exactly.
```

## Prompt Series 7: Configuration Files

### Prompt 7.1 - Update Police Cargo.toml

```
Update the Cargo.toml for the police system with all required dependencies.

Requirements:
- File: backend/police-system/Cargo.toml
- Add these additional dependencies:
  - chrono = { version = "0.4", features = ["serde"] }
  - env_logger = "0.11"

Keep existing dependencies. Organize them alphabetically.
```

### Prompt 7.2 - Update Hospital Cargo.toml

```
Update the Cargo.toml for the hospital system with all required dependencies.

Requirements:
- File: backend/hospital-system/Cargo.toml
- Add these additional dependencies:
  - chrono = { version = "0.4", features = ["serde"] }
  - env_logger = "0.11"

Keep existing dependencies. Organize them alphabetically.
```

### Prompt 7.3 - Environment Files

```
Create actual .env files (not examples) for both systems.

Requirements:
- File 1: backend/police-system/.env
  - DATABASE_URL=postgresql://postgres@localhost/police_db
  - SERVER_PORT=8000
  - HOSPITAL_API_URL=http://localhost:8001
  - API_KEY=dev_key_12345

- File 2: backend/hospital-system/.env
  - DATABASE_URL=postgresql://postgres@localhost/hospital_db
  - SERVER_PORT=8001
  - POLICE_API_URL=http://localhost:8000
  - API_KEY=dev_key_12345

Note: These should NOT be committed to git (already in .gitignore)
```

## Verification Prompt

```
Create a simple test script or checklist to verify the backend is working correctly.

Requirements:
- Test endpoints with curl commands
- Check database connectivity
- Verify CORS is working
- Test inter-system API calls
- Provide expected responses for each test

Make it copy-paste friendly for quick testing during development.
```

---

## Usage Instructions

1. Run prompts in sequence (1.1 → 7.3)
2. After each prompt, save the generated code to the specified file
3. Test incrementally after completing each series
4. Run the verification prompt at the end
5. Adjust prompts as needed based on errors or requirements
