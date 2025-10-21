# Database & Backend Fix Prompts

A series of AI prompts to fix schema mismatches, security issues, and documentation inconsistencies in the thesis project.

---

## Series 1: Database Schema Fixes

### Prompt 1.1 - Create Unified Schema File

```
Create a unified PostgreSQL schema file that sets up both the police and hospital databases in a single script.

Requirements:
- File: shared/database-schemas/unified-schema.sql
- Create two databases: police_db and hospital_db
- Police database tables:
  - cases: id (serial), case_number (varchar 50, unique), status (varchar 20), description (text), created_at (timestamp)
  - suspects: id (serial), case_id (foreign key to cases), name (varchar 100), personal_id (varchar 20), created_at (timestamp)
- Hospital database tables:
  - patients: id (serial), patient_id (varchar 50, unique), name (varchar 100), personal_id (varchar 20), created_at (timestamp)
  - medical_records: id (serial), patient_id (foreign key to patients), diagnosis (text), treatment (text), created_at (timestamp)
- Add CASCADE DELETE on all foreign key relationships
- Create indexes on:
  - suspects.personal_id
  - cases.status
  - patients.personal_id
- Use \c to switch between databases in the script
- Make it copy-paste friendly with no comments
```

### Prompt 1.2 - Create Unified Seed Data

```
Create a PostgreSQL seed data file that populates both databases with realistic test data.

Requirements:
- File: shared/database-schemas/unified-seed-data.sql
- Use Swedish names and personal ID format (YYYYMMDD-XXXX)
- Police database (police_db):
  - Insert 4-5 cases with mix of statuses: active, closed, pending
  - Insert 4-5 suspects linked to different cases
  - Use personal_id format: YYYYMMDD-XXXX
- Hospital database (hospital_db):
  - Insert 6-8 patients
  - Some patients should have the SAME personal_id as suspects (for testing inter-system queries)
  - Insert 5-8 medical records linked to different patients
  - Include variety of diagnoses and treatments
- Use \c to switch between databases
- Make it realistic but simple enough for testing
- No comments, copy-paste friendly
```

---

## Series 2: Security Improvements

### Prompt 2.1 - Update Police API Key Verification

```
Update the API key verification function in the police system to use environment variables instead of hardcoded values.

Requirements:
- File: backend/police-system/src/api/shared.rs
- Modify the verify_api_key function to:
  - Read API_KEY from environment variables using std::env::var()
  - Fall back to "dev_key_12345" if environment variable is not set
  - Keep the same header checking logic (X-API-Key)
- Maintain the same function signature: fn verify_api_key(req: &HttpRequest) -> bool
- Keep error handling simple
- No additional dependencies needed

Current function signature:
fn verify_api_key(req: &HttpRequest) -> bool {
    if let Some(api_key) = req.headers().get("X-API-Key") {
        if let Ok(key_str) = api_key.to_str() {
            return key_str == "dev_key_12345";
        }
    }
    false
}
```

### Prompt 2.2 - Update Hospital API Key Verification

```
Update the API key verification function in the hospital system to use environment variables instead of hardcoded values.

Requirements:
- File: backend/hospital-system/src/api/shared.rs
- Modify the verify_api_key function to:
  - Read API_KEY from environment variables using std::env::var()
  - Fall back to "dev_key_12345" if environment variable is not set
  - Keep the same header checking logic (X-API-Key)
- Maintain the same function signature: fn verify_api_key(req: &HttpRequest) -> bool
- Keep error handling simple
- No additional dependencies needed

Use the exact same pattern as the police system for consistency.
```

### Prompt 2.3 - Create Actual .env Files

```
Create actual .env files (not examples) for both backend systems with secure default values.

Requirements:
- File 1: backend/police-system/.env
  - DATABASE_URL=postgresql://postgres@localhost/police_db
  - SERVER_PORT=8000
  - HOSPITAL_API_URL=http://localhost:8001
  - API_KEY=secure_dev_key_change_in_production
  - RUST_LOG=info

- File 2: backend/hospital-system/.env
  - DATABASE_URL=postgresql://postgres@localhost/hospital_db
  - SERVER_PORT=8001
  - POLICE_API_URL=http://localhost:8000
  - API_KEY=secure_dev_key_change_in_production
  - RUST_LOG=info

Note: These files are already in .gitignore and will not be committed.
Make sure both systems use the SAME API_KEY for inter-system communication.
```

---

## Series 3: Input Validation (Optional Enhancement)

### Prompt 3.1 - Create Validation Utilities Module

```
Create a validation utilities module for the police system backend.

Requirements:
- File: backend/police-system/src/utils/mod.rs
  - Declare validators module

- File: backend/police-system/src/utils/validators.rs
  - Function: validate_personal_id(id: &str) -> bool
    - Validates format: YYYYMMDD-XXXX (8 digits, dash, 4 digits)
  - Function: validate_case_number(num: &str) -> bool
    - Validates format: P-YYYY-XXX (P, dash, 4 digits, dash, 3 digits)

- Use the regex crate (already available)
- Keep functions simple and focused
- Return true if valid, false if invalid

Add to Cargo.toml if needed:
regex = "1.10"
```

### Prompt 3.2 - Create Validation Utilities for Hospital

```
Create a validation utilities module for the hospital system backend.

Requirements:
- File: backend/hospital-system/src/utils/mod.rs
  - Declare validators module

- File: backend/hospital-system/src/utils/validators.rs
  - Function: validate_personal_id(id: &str) -> bool
    - Validates format: YYYYMMDD-XXXX (8 digits, dash, 4 digits)
  - Function: validate_patient_id(num: &str) -> bool
    - Validates format: H-YYYY-XXX (H, dash, 4 digits, dash, 3 digits)

Use the same pattern as the police system validators.
```

### Prompt 3.3 - Add Validation to Police Case Creation

```
Update the create_case endpoint to validate input before database insertion.

Requirements:
- File: backend/police-system/src/api/cases.rs
- In the create_case function, add validation:
  - Validate case_number format using validators::validate_case_number()
  - If invalid, return HTTP 400 Bad Request with JSON error message
  - Only proceed to database insertion if validation passes

- Import the validators module at the top
- Keep existing error handling for database errors
- Maintain the same function signature

Error response format:
{"error": "Invalid case number format. Expected: P-YYYY-XXX"}
```

### Prompt 3.4 - Add Validation to Suspect Creation

```
Update the create_suspect endpoint to validate personal_id if provided.

Requirements:
- File: backend/police-system/src/api/suspects.rs
- In the create_suspect function, add validation:
  - If personal_id is Some, validate using validators::validate_personal_id()
  - If invalid, return HTTP 400 Bad Request with JSON error message
  - Allow None values (personal_id is optional)
  - Only proceed to database insertion if validation passes

- Import the validators module at the top
- Keep existing error handling
- Maintain the same function signature

Error response format:
{"error": "Invalid personal ID format. Expected: YYYYMMDD-XXXX"}
```

### Prompt 3.5 - Add Validation to Hospital Patient Creation

```
Update the create_patient endpoint to validate input before database insertion.

Requirements:
- File: backend/hospital-system/src/api/patients.rs
- In the create_patient function, add validation:
  - Validate patient_id format using validators::validate_patient_id()
  - If personal_id is Some, validate using validators::validate_personal_id()
  - If either is invalid, return HTTP 400 Bad Request with appropriate error message
  - Only proceed to database insertion if validation passes

- Import the validators module at the top
- Keep existing error handling
- Maintain the same function signature

Error response format:
{"error": "Invalid patient ID format. Expected: H-YYYY-XXX"}
or
{"error": "Invalid personal ID format. Expected: YYYYMMDD-XXXX"}
```

### Prompt 3.6 - Update Cargo.toml for Regex

```
Add the regex dependency to both Cargo.toml files if validation is being used.

Requirements:
- File 1: backend/police-system/Cargo.toml
  - Add: regex = "1.10"
  - Keep all existing dependencies
  - Maintain alphabetical order

- File 2: backend/hospital-system/Cargo.toml
  - Add: regex = "1.10"
  - Keep all existing dependencies
  - Maintain alphabetical order
```

### Prompt 3.7 - Update Main.rs to Include Utils Module

```
Update the main.rs files in both systems to declare the utils module.

Requirements:
- File 1: backend/police-system/src/main.rs
  - Add: mod utils; (below the other mod declarations)

- File 2: backend/hospital-system/src/main.rs
  - Add: mod utils; (below the other mod declarations)

Keep all other code unchanged.
```

---

## Series 4: Documentation Updates

### Prompt 4.1 - Update PostgreSQL Guide

```
Update the PostgreSQL setup guide to reference the new unified schema files.

Requirements:
- File: docs/psql-guide.md
- Update Section "Setting Up Your Database Schema"
- Step 2 should now read:
  "Execute your unified schema file to create both databases:"
  psql -U postgres -f shared/database-schemas/unified-schema.sql

- Step 3 should now read:
  "After creating the schema, populate your databases with seed data:"
  psql -U postgres -f shared/database-schemas/unified-seed-data.sql

- Remove any references to schema.sql or seed-data.sql (without unified prefix)
- Remove the "Testing Flag Synchronization" section entirely (no longer applicable)
- Update the "Resetting Your Database" section to use unified-schema.sql and unified-seed-data.sql

Keep the rest of the documentation unchanged.
```

### Prompt 4.2 - Update Main README

```
Update the main README to reference the correct schema files.

Requirements:
- File: README.md
- Update the "Database Setup" section:
  Replace schema file references with:
  psql -U postgres -f shared/database-schemas/unified-schema.sql
  psql -U postgres -f shared/database-schemas/unified-seed-data.sql

- Add a note about environment variables:
  "Create .env files in both backend directories (examples provided in .env.example)"

Keep the rest of the README unchanged. Keep it concise.
```

### Prompt 4.3 - Update Backend Testing Checklist

```
Update the backend testing checklist to use the new unified schema files.

Requirements:
- File: docs/backend-testing-checklist.md
- Update Section "1. Database Connectivity Test"
  - Change queries to match new table structure
  - Update counts to match new seed data

- Update Section "7. Complete End-to-End Test Scenario"
  - Use realistic data that matches seed data format
  - Update personal_id examples to match Swedish format

- Update Section "9. Quick Verification Checklist"
  - Add check: "Environment variables are configured correctly"
  - Add check: "API keys match between systems"

- Update Section "10. Troubleshooting - Reset Databases"
  - Change file references to unified-schema.sql and unified-seed-data.sql

Keep all curl commands and test structure the same.
```

### Prompt 4.4 - Create Migration Guide

```
Create a new document explaining how to migrate from the old schema to the new unified schema.

Requirements:
- File: docs/migration-guide.md
- Title: "Migration Guide: Old Schema to Unified Schema"
- Sections:
  1. Why the migration is needed (schema mismatch issues)
  2. What changed (list old files vs new files)
  3. Step-by-step migration instructions:
     - Backup existing data (if any)
     - Drop old databases
     - Run new unified-schema.sql
     - Run new unified-seed-data.sql
     - Update .env files with API_KEY
     - Restart backend services
  4. Verification steps
  5. Rollback procedure (if needed)

Make it clear, professional, and beginner-friendly.
```

---

## Series 5: Verification & Testing

### Prompt 5.1 - Create Database Verification Script

```
Create a bash script to verify database setup is correct.

Requirements:
- File: scripts/verify-database.sh
- Script should:
  1. Check if PostgreSQL is running
  2. Check if police_db exists
  3. Check if hospital_db exists
  4. Verify table counts in each database
  5. Verify seed data was loaded (count rows)
  6. Test personal_id overlap between systems

- Use psql commands with proper error handling
- Print clear success/failure messages with colors:
  - Green checkmarks for success
  - Red X for failures

- Exit with code 0 if all checks pass, 1 if any fail
- Make it executable (chmod +x)

Example output format:
✓ PostgreSQL is running
✓ police_db exists
✓ hospital_db exists
✓ police_db has 2 tables
✓ hospital_db has 2 tables
✓ Found 4 cases in police_db
✓ Found 6 patients in hospital_db
✓ Personal ID overlap detected (2 matches)
```

### Prompt 5.2 - Create Backend Test Script

```
Create a bash script to test backend APIs are working correctly with the new setup.

Requirements:
- File: scripts/test-backend.sh
- Script should test:
  1. Health endpoints for both systems
  2. List all cases (police)
  3. List all patients (hospital)
  4. Create new case with validation
  5. Create new patient with validation
  6. Test inter-system query WITH API key (should succeed)
  7. Test inter-system query WITHOUT API key (should fail)
  8. Test invalid data format (should return 400)

- Use curl commands
- Parse JSON responses (using jq if available)
- Print test results with pass/fail indicators
- Set API_KEY from environment or use default
- Include timing information for each test

Make it robust and informative for debugging.
```

### Prompt 5.3 - Create Complete Setup Verification

```
Create a master verification checklist document that can be used after following all fix prompts.

Requirements:
- File: docs/setup-verification.md
- Title: "Setup Verification Checklist"
- Sections:
  1. Prerequisites Check
     - PostgreSQL installed and running
     - Rust installed
     - Node.js installed
     - All .env files created

  2. Database Verification
     - Databases created
     - Tables exist with correct structure
     - Seed data loaded
     - Indexes created

  3. Backend Verification
     - Both services compile without errors
     - Both services start without errors
     - Health endpoints respond
     - API routes work
     - Validation works
     - Inter-system communication works

  4. Security Verification
     - API keys loaded from environment
     - Invalid API keys rejected
     - CORS configured

  5. Common Issues & Solutions
     - Database connection errors
     - Port conflicts
     - Missing environment variables
     - Validation errors

Include copy-paste friendly commands for each check.
```

---

## Series 6: Cleanup & Organization

### Prompt 6.1 - Create Archive Directory

```
Create a simple instruction file for archiving deprecated schema files.

Requirements:
- File: docs/archive-old-files.md
- Explain that old schema files are deprecated
- Provide commands to move old files to an archive directory:

  mkdir -p shared/database-schemas/archive
  mv shared/database-schemas/schema.sql shared/database-schemas/archive/
  mv shared/database-schemas/seed-data.sql shared/database-schemas/archive/

- Add a README.md in the archive explaining why these files were archived
- Note that the archive is kept for reference only and should not be used

Keep it simple and clear.
```

### Prompt 6.2 - Update .gitignore if Needed

```
Review and update .gitignore to ensure proper files are excluded.

Requirements:
- File: .gitignore
- Ensure these are included:
  - .env (both backend directories)
  - target/ (Rust build output)
  - node_modules/
  - *.db and *.sqlite files

- Do NOT ignore:
  - .env.example files
  - Cargo.toml files
  - package.json files

Review existing .gitignore and only add what's missing. Keep existing entries.
```

---

## Usage Instructions

1. **Run prompts in order by series**:

   - Series 1 first (database fixes)
   - Series 2 second (security)
   - Series 3 optional (validation)
   - Series 4 (documentation)
   - Series 5 (verification)
   - Series 6 (cleanup)

2. **Test after each series**:

   - After Series 1: Test database setup
   - After Series 2: Test API key authentication
   - After Series 3: Test input validation
   - After Series 4: Review all documentation
   - After Series 5: Run verification scripts
   - After Series 6: Ensure clean repository

3. **Critical Order**:

   - Must complete Series 1 before testing backends
   - Must complete Series 2 before inter-system testing
   - Series 3 can be skipped if validation not needed
   - Complete Series 4 before sharing project

4. **Testing Between Series**:

   ```bash
   # After Series 1
   psql -U postgres -f shared/database-schemas/unified-schema.sql
   psql -U postgres -f shared/database-schemas/unified-seed-data.sql

   # After Series 2
   cd backend/police-system && cargo run &
   cd backend/hospital-system && cargo run &
   curl http://localhost:8000/health
   curl http://localhost:8001/health

   # After Series 5
   bash scripts/verify-database.sh
   bash scripts/test-backend.sh
   ```

---

## Expected Final State

After completing all prompts:

- ✅ Single, unified database schema that matches backend
- ✅ Realistic seed data for testing
- ✅ Environment-based API key configuration
- ✅ Optional input validation on all endpoints
- ✅ Updated, consistent documentation
- ✅ Automated verification scripts
- ✅ Clean repository with archived deprecated files
- ✅ Ready for frontend integration

---

## Notes

- All prompts are designed to be used with AI assistants (Claude, GPT-4, etc.)
- Each prompt is self-contained and includes all necessary context
- Prompts specify exact file paths and requirements
- No manual coding required - just copy/paste prompt outputs
- Scripts are bash-based for macOS/Linux (Windows users may need WSL)
