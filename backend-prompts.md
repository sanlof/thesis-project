# AI Prompts for Backend Code Generation

Here's a systematic series of prompts to generate all backend code for this thesis project:

---

## **Prompt 1: Police System - Cargo.toml**

```
Create a Cargo.toml file for the police backend system with the following requirements:

Project name: police-system
Edition: 2021

Dependencies needed:
- actix-web = "4.4" with "rustls" feature
- actix-cors = "0.7"
- sqlx with features: "runtime-tokio-rustls", "postgres", "chrono"
- tokio with features: "full"
- serde with "derive" feature
- serde_json
- chrono with "serde" feature
- dotenv
- env_logger

The service will run on port 8000 and connect to a PostgreSQL database.
```

---

## **Prompt 2: Police System - Database Models**

```
Create Rust data models for the police system based on this database schema:

Tables:
1. cases: id (serial), case_number (text), status (text), description (text), created_at (timestamp)
2. suspects: id (serial), case_id (integer FK), name (text), personal_id (text), created_at (timestamp)

Create three files:

**models/mod.rs**: Export both models

**models/case.rs**:
- Case struct with all fields
- NewCase struct for creation (without id and created_at)
- Derive: Debug, Clone, Serialize, Deserialize, sqlx::FromRow

**models/suspect.rs**:
- Suspect struct with all fields
- NewSuspect struct for creation (without id and created_at)
- Derive: Debug, Clone, Serialize, Deserialize, sqlx::FromRow

Use chrono::NaiveDateTime for timestamps.
```

---

## **Prompt 3: Police System - Database Connection**

```
Create database connection and query modules for the police system.

**database/mod.rs**: Export connection and queries modules

**database/connection.rs**:
- Create establish_connection() async function
- Use sqlx::postgres::PgPoolOptions
- Read DATABASE_URL from environment
- Max connections: 5
- Return Result<PgPool, sqlx::Error>
- Add proper error handling

**database/queries.rs**:
Implement these async functions using sqlx queries:

Cases:
- get_all_cases(pool: &PgPool) -> Result<Vec<Case>>
- get_case_by_id(pool: &PgPool, id: i32) -> Result<Option<Case>>
- create_case(pool: &PgPool, new_case: NewCase) -> Result<Case>

Suspects:
- get_suspects_by_case(pool: &PgPool, case_id: i32) -> Result<Vec<Suspect>>
- create_suspect(pool: &PgPool, case_id: i32, new_suspect: NewSuspect) -> Result<Suspect>
- get_suspect_by_personal_id(pool: &PgPool, personal_id: &str) -> Result<Option<Suspect>>

Use sqlx::query_as! macro for type safety.
```

---

## **Prompt 4: Police System - API Endpoints (Cases)**

```
Create REST API endpoints for case management in api/cases.rs.

Using actix-web, implement:

1. GET /api/cases
   - List all cases
   - Return JSON array

2. GET /api/cases/{id}
   - Get single case by ID
   - Return 404 if not found
   - Return JSON object

3. POST /api/cases
   - Create new case
   - Accept JSON body with NewCase structure
   - Return 201 Created with case object

Use actix_web::{web, HttpResponse, Responder}
Extract PgPool from app_data using web::Data<PgPool>
Proper error handling with appropriate HTTP status codes
```

---

## **Prompt 5: Police System - API Endpoints (Suspects)**

```
Create REST API endpoints for suspect management in api/suspects.rs.

Using actix-web, implement:

1. GET /api/cases/{case_id}/suspects
   - List all suspects for a case
   - Return JSON array

2. POST /api/cases/{case_id}/suspects
   - Add suspect to a case
   - Accept JSON body with NewSuspect structure
   - Return 201 Created with suspect object

Use actix_web::{web, HttpResponse, Responder}
Extract PgPool from app_data
Path parameters: web::Path<i32> for case_id
```

---

## **Prompt 6: Police System - Shared API (Inter-system)**

```
Create secure inter-system API endpoint in api/shared.rs.

Implement:

GET /api/shared/suspect/{personal_id}
- Requires X-API-Key header authentication
- Valid key: "dev_key_12345"
- Return 403 Forbidden if key missing or invalid
- Look up suspect by personal_id
- Return 404 if not found
- Return JSON:
  {
    "name": "string",
    "personal_id": "string",
    "has_active_cases": boolean
  }

To check for active cases: query if any case with this suspect has status "active"

Use actix_web::HttpRequest to access headers
Header validation middleware or manual check in handler
```

---

## **Prompt 7: Police System - API Module & Main Server**

```
Create the API module file and main server application.

**api/mod.rs**:
- Declare and export: cases, suspects, shared modules
- Create config function to register all routes:
  - /api/cases routes
  - /api/cases/{id}/suspects routes
  - /api/shared/suspect/{personal_id} route

**main.rs**:
Set up actix-web server:
- Load environment with dotenv
- Initialize env_logger
- Establish database connection pool
- Create HttpServer on 0.0.0.0:8000
- Configure CORS to allow all origins
- Add /health endpoint returning {"status":"ok","service":"police-system"}
- Register API routes using api::config
- Share PgPool using app_data
- Bind to SERVER_PORT from env (default 8000)

Use #[actix_web::main] macro
```

---

## **Prompt 8: Hospital System - Cargo.toml**

```
Create a Cargo.toml file for the hospital backend system.

Same dependencies as police system:
- actix-web = "4.4" with "rustls" feature
- actix-cors = "0.7"
- sqlx with features: "runtime-tokio-rustls", "postgres", "chrono"
- tokio with features: "full"
- serde with "derive" feature
- serde_json
- chrono with "serde" feature
- dotenv
- env_logger

Project name: hospital-system
Edition: 2021
Will run on port 8001
```

---

## **Prompt 9: Hospital System - Database Models**

```
Create Rust data models for the hospital system.

Tables:
1. patients: id (serial), patient_id (text), name (text), personal_id (text), created_at (timestamp)
2. medical_records: id (serial), patient_id (integer FK), diagnosis (text), treatment (text), created_at (timestamp)

Create three files:

**models/mod.rs**: Export both models

**models/patient.rs**:
- Patient struct with all fields
- NewPatient struct for creation (without id and created_at)
- Derive: Debug, Clone, Serialize, Deserialize, sqlx::FromRow

**models/record.rs**:
- MedicalRecord struct with all fields
- NewMedicalRecord struct (without id and created_at)
- Derive: Debug, Clone, Serialize, Deserialize, sqlx::FromRow

Use chrono::NaiveDateTime for timestamps.
```

---

## **Prompt 10: Hospital System - Database Layer**

```
Create database connection and queries for hospital system.

**database/mod.rs**: Export connection and queries modules

**database/connection.rs**:
- establish_connection() async function
- Use sqlx::postgres::PgPoolOptions
- Read DATABASE_URL from environment
- Max connections: 5
- Return Result<PgPool, sqlx::Error>

**database/queries.rs**:
Implement these async functions:

Patients:
- get_all_patients(pool: &PgPool) -> Result<Vec<Patient>>
- get_patient_by_id(pool: &PgPool, id: i32) -> Result<Option<Patient>>
- create_patient(pool: &PgPool, new_patient: NewPatient) -> Result<Patient>
- get_patient_by_personal_id(pool: &PgPool, personal_id: &str) -> Result<Option<Patient>>

Medical Records:
- get_records_by_patient(pool: &PgPool, patient_id: i32) -> Result<Vec<MedicalRecord>>
- create_record(pool: &PgPool, patient_id: i32, new_record: NewMedicalRecord) -> Result<MedicalRecord>

Use sqlx::query_as! for type-safe queries.
```

---

## **Prompt 11: Hospital System - API Endpoints (Patients)**

```
Create REST API endpoints for patient management in api/patients.rs.

Using actix-web, implement:

1. GET /api/patients
   - List all patients
   - Return JSON array

2. GET /api/patients/{id}
   - Get single patient by ID
   - Return 404 if not found

3. POST /api/patients
   - Create new patient
   - Accept JSON body with NewPatient
   - Return 201 Created

4. GET /api/patients/{patient_id}/records
   - Get all medical records for a patient
   - Return JSON array

5. POST /api/patients/{patient_id}/records
   - Add medical record to patient
   - Accept JSON body with NewMedicalRecord
   - Return 201 Created

Use web::Data<PgPool>, proper error handling.
```

---

## **Prompt 12: Hospital System - Shared API & Main Server**

```
Create two files:

**api/shared.rs**:
Implement GET /api/shared/patient/{personal_id}
- Requires X-API-Key header: "dev_key_12345"
- Return 403 if unauthorized
- Look up patient by personal_id
- Return 404 if not found
- Return JSON:
  {
    "name": "string",
    "personal_id": "string",
    "has_records": boolean
  }

Check has_records by querying if patient has any medical_records

**api/mod.rs**:
- Export patients and shared modules
- config function to register all routes

**main.rs**:
Set up actix-web server:
- Load dotenv, init logger
- Database connection
- HttpServer on 0.0.0.0:8001
- CORS allow all
- /health endpoint: {"status":"ok","service":"hospital-system"}
- Register API routes
- Share PgPool
- Bind to port 8001 (from SERVER_PORT env)
```

---

## **Prompt 13: Testing & Verification Script**

```
Create a bash script: test-backends.sh

The script should:
1. Check if both services are running (curl health endpoints)
2. Test police system:
   - GET all cases
   - POST create new case
   - GET case by id
   - POST add suspect to case
3. Test hospital system:
   - GET all patients
   - POST create new patient
   - GET patient by id
   - POST add medical record
4. Test inter-system communication:
   - Hospital queries police (with and without API key)
   - Police queries hospital (with and without API key)
5. Output success/failure for each test
6. Use jq for JSON parsing if available

Include proper error handling and colored output (green for success, red for failure).
```

---

## **Usage Instructions**

Use these prompts in sequence with an AI assistant. After each prompt:

1. Review the generated code
2. Save to the appropriate file path
3. Test compilation with `cargo check`
4. Move to the next prompt

After all prompts:

1. Create `.env` files from `.env.example` templates
2. Run database schema and seed files
3. Start both backend services
4. Run the test script to verify everything works

This systematic approach ensures all backend components are generated with proper structure, error handling, and inter-system communication.
