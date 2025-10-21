# Backend Testing Guide

A comprehensive manual testing guide for the Police and Hospital backend services.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Starting the Services](#starting-the-services)
- [Basic Health Checks](#basic-health-checks)
- [Test Scenario 1: CRUD Operations](#test-scenario-1-crud-operations)
- [Test Scenario 2: Flag Synchronization](#test-scenario-2-flag-synchronization)
- [Test Scenario 3: Inter-System Communication](#test-scenario-3-inter-system-communication)
- [Test Scenario 4: Swedish Personal ID Validation](#test-scenario-4-swedish-personal-id-validation)
- [Database Verification](#database-verification)
- [Common Error Scenarios](#common-error-scenarios)
- [Cleanup](#cleanup)
- [Automated Test Script](#automated-test-script)

---

## Prerequisites

Before testing, ensure you have:

- [x] PostgreSQL installed and running
- [x] Databases created (police_db and hospital_db)
- [x] Seed data loaded
- [x] Both Rust services compiled
- [x] Environment variables configured in `.env` files
- [x] `curl` and `jq` installed (jq for pretty JSON output)

### Install jq (if needed)

```bash
# macOS
brew install jq

# Ubuntu/Debian
sudo apt-get install jq

# Or test without jq by removing "| jq" from commands
```

---

## Starting the Services

### 1. Start PostgreSQL

```bash
# Check if PostgreSQL is running
brew services list

# Start if not running
brew services start postgresql@15
```

### 2. Verify Databases

```bash
# Connect to police database
psql -U postgres -d police_db -c "SELECT COUNT(*) FROM suspects;"

# Connect to hospital database
psql -U postgres -d hospital_db -c "SELECT COUNT(*) FROM patients;"
```

Expected output: You should see the count of records from seed data (10 suspects, 8 patients).

### 3. Start Police System (Terminal 1)

```bash
cd backend/police-system
cargo run
```

**Expected output:**

```
🚔 Police System Starting...
Connecting to database...
✅ Database connection established
📋 Configuring routes:
   - GET    /suspects
   - POST   /suspects
   ...
🚀 Starting HTTP server at http://127.0.0.1:8000
🔗 CORS enabled for hospital system at http://localhost:8001
```

### 4. Start Hospital System (Terminal 2)

```bash
cd backend/hospital-system
cargo run
```

**Expected output:**

```
🏥 Hospital System Starting...
Connecting to database...
✅ Database connection established
📋 Configuring routes:
   - GET    /patients
   - POST   /patients
   ...
🚀 Starting HTTP server at http://127.0.0.1:8001
🔗 CORS enabled for police system at http://localhost:8000
```

---

## Basic Health Checks

Open a new terminal (Terminal 3) for testing.

### Test 1: Police System Health

```bash
curl http://localhost:8000/health | jq
```

**Expected Response:**

```json
{
  "status": "healthy",
  "service": "police-system"
}
```

### Test 2: Hospital System Health

```bash
curl http://localhost:8001/health | jq
```

**Expected Response:**

```json
{
  "status": "healthy",
  "service": "hospital-system"
}
```

**✅ Pass Criteria:** Both services return status "healthy"

---

## Test Scenario 1: CRUD Operations

### Police System CRUD

#### 1.1 List All Suspects

```bash
curl http://localhost:8000/suspects | jq
```

**Expected Response:** Array of 10 suspects from seed data

**Sample:**

```json
[
  {
    "id": 1,
    "full_name": "Erik Andersson",
    "personal_id": "19850312-2398",
    "flag": false
  },
  {
    "id": 2,
    "full_name": "Anna Karlsson",
    "personal_id": "19900204-1457",
    "flag": true
  }
  // ... more records
]
```

#### 1.2 Get Specific Suspect by ID

```bash
curl http://localhost:8000/suspects/1 | jq
```

**Expected Response:**

```json
{
  "id": 1,
  "full_name": "Erik Andersson",
  "personal_id": "19850312-2398",
  "flag": false
}
```

#### 1.3 Get Suspect by Personal ID

```bash
curl http://localhost:8000/suspects/personal/19850312-2398 | jq
```

**Expected Response:** Same as above

#### 1.4 Create New Suspect

```bash
curl -X POST http://localhost:8000/suspects \
  -H "Content-Type: application/json" \
  -d '{
    "full_name": "Test Testsson",
    "personal_id": "19950101-1111",
    "flag": false
  }' | jq
```

**Expected Response:** `201 Created`

```json
{
  "id": 11,
  "full_name": "Test Testsson",
  "personal_id": "19950101-1111",
  "flag": false
}
```

**Note:** Save the returned `id` for next steps. In this example, it's `11`.

#### 1.5 Update Suspect

```bash
curl -X PUT http://localhost:8000/suspects/11 \
  -H "Content-Type: application/json" \
  -d '{
    "personal_id": "19950101-1111",
    "full_name": "Test Updated Testsson"
  }' | jq
```

**Expected Response:** `200 OK`

```json
{
  "id": 11,
  "full_name": "Test Updated Testsson",
  "personal_id": "19950101-1111",
  "flag": false
}
```

#### 1.6 Delete Suspect

```bash
curl -X DELETE http://localhost:8000/suspects/11 -v
```

**Expected Response:** `204 No Content` (empty body)

**Verification:**

```bash
# This should return 404
curl http://localhost:8000/suspects/11 | jq
```

**Expected:**

```json
{
  "error": "Suspect not found"
}
```

**✅ Pass Criteria:** All CRUD operations complete successfully with correct status codes

---

### Hospital System CRUD

#### 1.7 List All Patients

```bash
curl http://localhost:8001/patients | jq
```

**Expected Response:** Array of 8 patients from seed data

#### 1.8 Get Specific Patient by ID

```bash
curl http://localhost:8001/patients/1 | jq
```

**Expected Response:**

```json
{
  "id": 1,
  "full_name": "Erik Andersson",
  "personal_id": "19850312-2398",
  "flag": false
}
```

#### 1.9 Get Patient by Personal ID

```bash
curl http://localhost:8001/patients/personal/19850312-2398 | jq
```

**Expected Response:** Same as above

#### 1.10 Create New Patient

```bash
curl -X POST http://localhost:8001/patients \
  -H "Content-Type: application/json" \
  -d '{
    "full_name": "Patient Patientsson",
    "personal_id": "19960202-2222",
    "flag": false
  }' | jq
```

**Expected Response:** `201 Created`

```json
{
  "id": 9,
  "full_name": "Patient Patientsson",
  "personal_id": "19960202-2222",
  "flag": false
}
```

#### 1.11 Update Patient

```bash
curl -X PUT http://localhost:8001/patients/9 \
  -H "Content-Type: application/json" \
  -d '{
    "personal_id": "19960202-2222",
    "full_name": "Patient Updated Patientsson"
  }' | jq
```

**Expected Response:** `200 OK` with updated data

#### 1.12 Delete Patient

```bash
curl -X DELETE http://localhost:8001/patients/9 -v
```

**Expected Response:** `204 No Content`

**✅ Pass Criteria:** All CRUD operations complete successfully with correct status codes

---

## Test Scenario 2: Flag Synchronization

This is the **most important test** as it validates the core feature of the system: automatic flag synchronization from police to hospital via PostgreSQL FDW.

### 2.1 Check Initial State

First, verify the current flag status in both systems for Erik Andersson (personal_id: 19850312-2398).

**Police System:**

```bash
curl http://localhost:8000/suspects/personal/19850312-2398 | jq .flag
```

**Hospital System:**

```bash
curl http://localhost:8001/patients/personal/19850312-2398 | jq .flag
```

**Expected:** Both should show `false` initially (from seed data)

### 2.2 Flag the Suspect in Police System

```bash
curl -X PUT http://localhost:8000/suspects/19850312-2398/flag \
  -H "Content-Type: application/json" \
  -d '{"flag": true}' | jq
```

**Expected Response:** `200 OK`

```json
{
  "id": 1,
  "full_name": "Erik Andersson",
  "personal_id": "19850312-2398",
  "flag": true
}
```

**Check Server Logs:**
Look at Terminal 1 (Police System). You should see:

```
INFO  police_system: Updated flag to true for suspect with personal_id 19850312-2398 (will auto-sync to hospital)
```

### 2.3 Verify Automatic Synchronization

**IMMEDIATELY check Hospital System (no delay needed):**

```bash
curl http://localhost:8001/patients/personal/19850312-2398 | jq
```

**Expected Response:**

```json
{
  "id": 1,
  "full_name": "Erik Andersson",
  "personal_id": "19850312-2398",
  "flag": true
}
```

**Note:** The flag should be `true` - automatically synchronized!

### 2.4 Get All Flagged Patients

```bash
curl http://localhost:8001/patients/flagged | jq
```

**Expected Response:** Array of patients with `flag: true`, including Erik Andersson

### 2.5 Test Unflagging

```bash
curl -X PUT http://localhost:8000/suspects/19850312-2398/flag \
  -H "Content-Type: application/json" \
  -d '{"flag": false}' | jq
```

**Verify in Hospital:**

```bash
curl http://localhost:8001/patients/personal/19850312-2398 | jq .flag
```

**Expected:** `false`

### 2.6 Verify Database-Level Sync

Connect to both databases to confirm:

```bash
# Check police database
psql -U postgres -d police_db -c \
  "SELECT full_name, personal_id, flag FROM suspects WHERE personal_id = '19850312-2398';"

# Check hospital database
psql -U postgres -d hospital_db -c \
  "SELECT full_name, personal_id, flag FROM patients WHERE personal_id = '19850312-2398';"
```

**Expected:** Both should show the same flag value

**✅ Pass Criteria:**

- Flag changes in police system appear immediately in hospital system
- No API calls to hospital system needed
- Database verification confirms synchronization

---

## Test Scenario 3: Inter-System Communication

These tests verify that each system can query the other's data through shared API endpoints.

### 3.1 Hospital Checks Police Records

**Scenario:** Hospital admits a patient and wants to check if they have a police record.

```bash
# Hospital queries police system
curl http://localhost:8000/api/shared/suspects/19900204-1457 | jq
```

**Expected Response:** `200 OK`

```json
{
  "id": 2,
  "full_name": "Anna Karlsson",
  "personal_id": "19900204-1457",
  "flag": true
}
```

**Interpretation:** Patient Anna Karlsson has a police record and is flagged.

### 3.2 Hospital Checks for Non-Existent Police Record

```bash
# Query someone not in police database
curl http://localhost:8000/api/shared/suspects/19990909-9999 | jq
```

**Expected Response:** `404 Not Found`

```json
{
  "error": "No suspect record found",
  "personal_id": "19990909-9999"
}
```

**Interpretation:** Person has no police record (this is normal and expected).

### 3.3 Police Checks Hospital Records

**Scenario:** Police investigating a suspect wants to check if they have medical records.

```bash
# Police queries hospital system
curl http://localhost:8001/api/shared/patients/19850312-2398 | jq
```

**Expected Response:** `200 OK`

```json
{
  "id": 1,
  "full_name": "Erik Andersson",
  "personal_id": "19850312-2398",
  "flag": false
}
```

**Interpretation:** Suspect Erik Andersson has hospital records.

### 3.4 Police Checks for Non-Existent Hospital Record

```bash
# Query someone not in hospital database (e.g., Simon Nyberg - police-only)
curl http://localhost:8001/api/shared/patients/19930808-4417 | jq
```

**Expected Response:** `404 Not Found`

```json
{
  "error": "No patient record found",
  "personal_id": "19930808-4417"
}
```

**Interpretation:** Suspect has no hospital records.

### 3.5 Get All Suspects (Hospital View)

```bash
curl http://localhost:8000/api/shared/suspects | jq
```

**Expected Response:** Array of all suspects

### 3.6 Get All Patients (Police View)

```bash
curl http://localhost:8001/api/shared/patients | jq
```

**Expected Response:** Array of all patients

### 3.7 Get Flagged Patients (Police View)

```bash
curl http://localhost:8001/api/shared/patients/flagged | jq
```

**Expected Response:** Array of flagged patients

**Check Server Logs:**

- Terminal 1 (Police): Should show "Shared API: Hospital system querying..."
- Terminal 2 (Hospital): Should show "Shared API: Police system querying..."

**✅ Pass Criteria:**

- Both systems can query each other successfully
- 404 responses for non-existent records (not errors!)
- Server logs show inter-system communication

---

## Test Scenario 4: Swedish Personal ID Validation

Test how the system handles invalid Swedish personal ID formats.

### 4.1 Test Valid Format

```bash
curl -X POST http://localhost:8000/suspects \
  -H "Content-Type: application/json" \
  -d '{
    "full_name": "Valid Person",
    "personal_id": "20000101-1234",
    "flag": false
  }' | jq
```

**Expected:** `201 Created` with new suspect

### 4.2 Test Invalid Format - Too Short

```bash
curl -X POST http://localhost:8000/suspects \
  -H "Content-Type: application/json" \
  -d '{
    "full_name": "Invalid Person",
    "personal_id": "2000010-123",
    "flag": false
  }' | jq
```

**Expected:** `500 Internal Server Error` (database constraint violation)

### 4.3 Test Invalid Format - Wrong Separator

```bash
curl -X POST http://localhost:8000/suspects \
  -H "Content-Type: application/json" \
  -d '{
    "full_name": "Invalid Person",
    "personal_id": "20000101+1234",
    "flag": false
  }' | jq
```

**Expected:** `201 Created` (accepted by database, but validation could be improved)

### 4.4 Test Duplicate Personal ID

```bash
# Try to create suspect with existing personal_id
curl -X POST http://localhost:8000/suspects \
  -H "Content-Type: application/json" \
  -d '{
    "full_name": "Duplicate Person",
    "personal_id": "19850312-2398",
    "flag": false
  }' | jq
```

**Expected:** `500 Internal Server Error`

```json
{
  "error": "Failed to create suspect"
}
```

**Server Log:** Should show unique constraint violation

**✅ Pass Criteria:**

- Valid formats are accepted
- Duplicate personal_ids are rejected
- Database constraints work correctly

---

## Database Verification

Sometimes you need to verify data directly in the database.

### Connect to Police Database

```bash
psql -U postgres -d police_db
```

**Useful queries:**

```sql
-- List all suspects
SELECT * FROM suspects;

-- Count total suspects
SELECT COUNT(*) FROM suspects;

-- Find flagged suspects
SELECT full_name, personal_id, flag FROM suspects WHERE flag = true;

-- Check specific person
SELECT * FROM suspects WHERE personal_id = '19850312-2398';

-- Exit psql
\q
```

### Connect to Hospital Database

```bash
psql -U postgres -d hospital_db
```

**Useful queries:**

```sql
-- List all patients
SELECT * FROM patients;

-- Count total patients
SELECT COUNT(*) FROM patients;

-- Find flagged patients
SELECT full_name, personal_id, flag FROM patients WHERE flag = true;

-- Check specific person
SELECT * FROM patients WHERE personal_id = '19850312-2398';

-- Compare with police database (using FDW)
-- Note: This query runs from police_db
\q
```

### Verify Flag Synchronization at Database Level

From police database, you can query the hospital's patients table directly via FDW:

```bash
psql -U postgres -d police_db
```

```sql
-- Query hospital's patients table from police database
SELECT personal_id, flag FROM patients WHERE personal_id = '19850312-2398';

-- Compare with local suspects table
SELECT personal_id, flag FROM suspects WHERE personal_id = '19850312-2398';

-- Both should show the same flag value!
\q
```

**✅ Pass Criteria:** Flag values match in both databases for the same personal_id

---

## Common Error Scenarios

### Error 1: Connection Refused

**Symptom:**

```bash
curl: (7) Failed to connect to localhost port 8000: Connection refused
```

**Cause:** Service not running

**Solution:**

```bash
# Check if services are running
ps aux | grep police-system
ps aux | grep hospital-system

# Start the service
cd backend/police-system
cargo run
```

### Error 2: Database Connection Failed

**Server Log:**

```
ERROR Failed to connect to database: ...
Please verify DATABASE_URL is correct and PostgreSQL is running
```

**Solution:**

```bash
# Check PostgreSQL status
brew services list

# Start if needed
brew services start postgresql@15

# Verify database exists
psql -U postgres -l | grep police_db
```

### Error 3: 404 Not Found

**Response:**

```json
{
  "error": "Suspect not found"
}
```

**This is NOT an error!** It's a valid response indicating the person doesn't exist in the database. This is expected behavior for queries about non-existent records.

### Error 4: 500 Internal Server Error

**Response:**

```json
{
  "error": "Failed to create suspect"
}
```

**Common Causes:**

- Duplicate personal_id (unique constraint violation)
- Invalid data format
- Database connection lost

**Troubleshooting:**

1. Check server logs for detailed error message
2. Verify personal_id is unique
3. Check PostgreSQL is still running

### Error 5: CORS Errors (Browser)

If testing from a browser or frontend:

**Error:** "CORS policy: No 'Access-Control-Allow-Origin' header"

**Cause:** Requests from unauthorized origin

**Solution:** Ensure CORS is configured in main.rs (it should be by default for localhost:8000/8001)

### Error 6: Port Already in Use

**Server Log:**

```
ERROR Failed to bind server to 127.0.0.1:8000: Address already in use
```

**Solution:**

```bash
# Find process using port 8000
lsof -i :8000

# Kill the process (use PID from above command)
kill -9 <PID>

# Or use a different port in .env
SERVER_PORT=8002
```

---

## Cleanup

After testing, you may want to reset the database to its original state.

### Reset Database Data

```bash
# Option 1: Truncate and re-seed (preserves structure)
psql -U postgres -d hospital_db -c "TRUNCATE patients RESTART IDENTITY CASCADE;"
psql -U postgres -d police_db -c "TRUNCATE suspects RESTART IDENTITY CASCADE;"
psql -U postgres -f shared/database-schemas/seed-data.sql

# Option 2: Drop and recreate everything (complete reset)
psql -U postgres -c "DROP DATABASE IF EXISTS police_db;"
psql -U postgres -c "DROP DATABASE IF EXISTS hospital_db;"
psql -U postgres -f shared/database-schemas/schema.sql
psql -U postgres -f shared/database-schemas/seed-data.sql
```

### Stop Services

```bash
# In each terminal window running cargo run:
# Press Ctrl+C

# Or find and kill processes:
ps aux | grep "police-system\|hospital-system" | awk '{print $2}' | xargs kill
```

### Stop PostgreSQL (Optional)

```bash
brew services stop postgresql@15
```

---

## Automated Test Script

Save this as `test-all.sh` in the project root:

```bash
#!/bin/bash

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo "🧪 Starting Backend Integration Tests"
echo "======================================"

# Check if services are running
echo -e "\n${YELLOW}Checking services...${NC}"
if ! curl -s http://localhost:8000/health > /dev/null; then
    echo -e "${RED}❌ Police system not running on port 8000${NC}"
    exit 1
fi

if ! curl -s http://localhost:8001/health > /dev/null; then
    echo -e "${RED}❌ Hospital system not running on port 8001${NC}"
    exit 1
fi

echo -e "${GREEN}✅ Both services are running${NC}"

# Test 1: Health Checks
echo -e "\n${YELLOW}Test 1: Health Checks${NC}"
POLICE_HEALTH=$(curl -s http://localhost:8000/health | jq -r .status)
HOSPITAL_HEALTH=$(curl -s http://localhost:8001/health | jq -r .status)

if [ "$POLICE_HEALTH" = "healthy" ] && [ "$HOSPITAL_HEALTH" = "healthy" ]; then
    echo -e "${GREEN}✅ Health checks passed${NC}"
else
    echo -e "${RED}❌ Health checks failed${NC}"
    exit 1
fi

# Test 2: List Records
echo -e "\n${YELLOW}Test 2: List Records${NC}"
SUSPECT_COUNT=$(curl -s http://localhost:8000/suspects | jq '. | length')
PATIENT_COUNT=$(curl -s http://localhost:8001/patients | jq '. | length')

echo "Suspects: $SUSPECT_COUNT"
echo "Patients: $PATIENT_COUNT"

if [ "$SUSPECT_COUNT" -gt 0 ] && [ "$PATIENT_COUNT" -gt 0 ]; then
    echo -e "${GREEN}✅ Records retrieved successfully${NC}"
else
    echo -e "${RED}❌ Failed to retrieve records${NC}"
    exit 1
fi

# Test 3: Create Suspect
echo -e "\n${YELLOW}Test 3: Create Suspect${NC}"
CREATE_RESPONSE=$(curl -s -X POST http://localhost:8000/suspects \
    -H "Content-Type: application/json" \
    -d '{"full_name":"Test Person","personal_id":"19990101-9999","flag":false}')

SUSPECT_ID=$(echo $CREATE_RESPONSE | jq -r .id)

if [ "$SUSPECT_ID" != "null" ]; then
    echo -e "${GREEN}✅ Suspect created with ID: $SUSPECT_ID${NC}"
else
    echo -e "${RED}❌ Failed to create suspect${NC}"
    exit 1
fi

# Test 4: Flag Synchronization
echo -e "\n${YELLOW}Test 4: Flag Synchronization${NC}"

# Flag the suspect
curl -s -X PUT http://localhost:8000/suspects/19850312-2398/flag \
    -H "Content-Type: application/json" \
    -d '{"flag":true}' > /dev/null

# Check hospital
PATIENT_FLAG=$(curl -s http://localhost:8001/patients/personal/19850312-2398 | jq -r .flag)

if [ "$PATIENT_FLAG" = "true" ]; then
    echo -e "${GREEN}✅ Flag synchronized successfully${NC}"
else
    echo -e "${RED}❌ Flag synchronization failed${NC}"
    exit 1
fi

# Test 5: Inter-System Communication
echo -e "\n${YELLOW}Test 5: Inter-System Communication${NC}"

# Hospital checks police records
POLICE_RECORD=$(curl -s http://localhost:8000/api/shared/suspects/19900204-1457 | jq -r .full_name)

if [ "$POLICE_RECORD" = "Anna Karlsson" ]; then
    echo -e "${GREEN}✅ Hospital can query police records${NC}"
else
    echo -e "${RED}❌ Hospital query to police failed${NC}"
    exit 1
fi

# Police checks hospital records
HOSPITAL_RECORD=$(curl -s http://localhost:8001/api/shared/patients/19850312-2398 | jq -r .full_name)

if [ "$HOSPITAL_RECORD" = "Erik Andersson" ]; then
    echo -e "${GREEN}✅ Police can query hospital records${NC}"
else
    echo -e "${RED}❌ Police query to hospital failed${NC}"
    exit 1
fi

# Test 6: Delete Test Suspect
echo -e "\n${YELLOW}Test 6: Delete Test Record${NC}"
DELETE_STATUS=$(curl -s -o /dev/null -w "%{http_code}" -X DELETE http://localhost:8000/suspects/$SUSPECT_ID)

if [ "$DELETE_STATUS" = "204" ]; then
    echo -e "${GREEN}✅ Test suspect deleted successfully${NC}"
else
    echo -e "${RED}❌ Failed to delete test suspect${NC}"
    exit 1
fi

# Reset flag for Erik Andersson
curl -s -X PUT http://localhost:8000/suspects/19850312-2398/flag \
    -H "Content-Type: application/json" \
    -d '{"flag":false}' > /dev/null

# Summary
echo -e "\n${GREEN}======================================"
echo "✅ All tests passed!"
echo -e "======================================${NC}"
```

**Make it executable:**

```bash
chmod +x test-all.sh
```

**Run tests:**

```bash
./test-all.sh
```

**Expected Output:**

```
🧪 Starting Backend Integration Tests
======================================

Checking services...
✅ Both services are running

Test 1: Health Checks
✅ Health checks passed

Test 2: List Records
Suspects: 10
Patients: 8
✅ Records retrieved successfully

Test 3: Create Suspect
✅ Suspect created with ID: 11

Test 4: Flag Synchronization
✅ Flag synchronized successfully

Test 5: Inter-System Communication
✅ Hospital can query police records
✅ Police can query hospital records

Test 6: Delete Test Record
✅ Test suspect deleted successfully

======================================
✅ All tests passed!
======================================
```

---

## Quick Test Checklist

Use this checklist for rapid verification:

- [ ] Both services start without errors
- [ ] Health endpoints return 200 OK
- [ ] Can list suspects and patients
- [ ] Can create new records
- [ ] Can update existing records
- [ ] Can delete records
- [ ] Flag changes in police sync to hospital
- [ ] Hospital can query police records
- [ ] Police can query hospital records
- [ ] Duplicate personal_ids are rejected
- [ ] 404 returned for non-existent records

---

## Performance Testing (Optional)

For basic load testing:

```bash
# Install Apache Bench (if not already installed)
brew install httpd

# Test GET endpoint (100 requests, 10 concurrent)
ab -n 100 -c 10 http://localhost:8000/suspects

# Test with POST (requires payload file)
echo '{"full_name":"Load Test","personal_id":"19990101-0001","flag":false}' > post_data.json
ab -n 100 -c 10 -p post_data.json -T application/json http://localhost:8000/suspects
```

---

## Troubleshooting Tips

1. **Always check server logs first** - They contain detailed error messages
2. **Use `| jq` for readable JSON output** - Makes responses easier to understand
3. **Verify PostgreSQL is running** - Most errors come from database issues
4. **Check .env files** - Ensure DATABASE_URL and ports are correct
5. **Test one thing at a time** - Isolate issues by testing incrementally
6. **Use psql to verify data** - When in doubt, check the database directly
7. **Restart services if behavior is weird** - Sometimes state issues occur

---

## Next Steps

After successful testing:

1. **Document any bugs found** - Create issues for tracking
2. **Add more test scenarios** - Cover edge cases specific to your thesis
3. **Set up CI/CD** - Automate tests with GitHub Actions
4. **Add integration tests** - Write Rust tests with `#[actix_web::test]`
5. **Monitor performance** - Use logging to identify bottlenecks
6. **Implement authentication** - Add JWT for production

---

_Happy Testing! 🧪_
