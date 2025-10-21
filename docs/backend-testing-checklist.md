# Backend Testing Checklist

A comprehensive guide to test both police and hospital backend systems.

## Prerequisites

Ensure both services are running:

- Police system: `http://localhost:8000`
- Hospital system: `http://localhost:8001`
- PostgreSQL databases are seeded with sample data

---

## 1. Database Connectivity Test

### Check Police Database

```bash
psql -U postgres -d police_db -c "SELECT COUNT(*) FROM cases;"
```

**Expected:** Should return a count (at least 2 from seed data)

### Check Hospital Database

```bash
psql -U postgres -d hospital_db -c "SELECT COUNT(*) FROM patients;"
```

**Expected:** Should return a count (at least 2 from seed data)

---

## 2. Health Check Endpoints

### Police System Health

```bash
curl -X GET http://localhost:8000/health
```

**Expected Response:**

```json
{ "status": "ok", "service": "police-system" }
```

### Hospital System Health

```bash
curl -X GET http://localhost:8001/health
```

**Expected Response:**

```json
{ "status": "ok", "service": "hospital-system" }
```

---

## 3. Police System API Tests

### 3.1 Get All Cases

```bash
curl -X GET http://localhost:8000/api/cases
```

**Expected:** JSON array with at least 2 cases (from seed data)

### 3.2 Get Specific Case

```bash
curl -X GET http://localhost:8000/api/cases/1
```

**Expected:** Single case object with fields: `id`, `case_number`, `status`, `description`, `created_at`

### 3.3 Create New Case

```bash
curl -X POST http://localhost:8000/api/cases \
  -H "Content-Type: application/json" \
  -d '{
    "case_number": "P-2024-TEST-001",
    "status": "active",
    "description": "Test case for verification"
  }'
```

**Expected:** HTTP 201 Created with the newly created case object

### 3.4 Get Suspects for a Case

```bash
curl -X GET http://localhost:8000/api/cases/1/suspects
```

**Expected:** JSON array (may be empty if no suspects added yet)

### 3.5 Add Suspect to Case

```bash
curl -X POST http://localhost:8000/api/cases/1/suspects \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Test Suspect",
    "personal_id": "19900101-1234"
  }'
```

**Expected:** HTTP 201 Created with the newly created suspect object

### 3.6 Error Handling - Invalid Case ID

```bash
curl -X GET http://localhost:8000/api/cases/9999
```

**Expected:** HTTP 404 with error message: `{"error":"Case not found"}`

---

## 4. Hospital System API Tests

### 4.1 Get All Patients

```bash
curl -X GET http://localhost:8001/api/patients
```

**Expected:** JSON array with at least 2 patients (from seed data)

### 4.2 Get Specific Patient

```bash
curl -X GET http://localhost:8001/api/patients/1
```

**Expected:** Single patient object with fields: `id`, `patient_id`, `name`, `personal_id`, `created_at`

### 4.3 Create New Patient

```bash
curl -X POST http://localhost:8001/api/patients \
  -H "Content-Type: application/json" \
  -d '{
    "patient_id": "H-2024-TEST-001",
    "name": "Test Patient",
    "personal_id": "19950505-5678"
  }'
```

**Expected:** HTTP 201 Created with the newly created patient object

### 4.4 Get Medical Records for a Patient

```bash
curl -X GET http://localhost:8001/api/patients/1/records
```

**Expected:** JSON array (may be empty if no records added yet)

### 4.5 Add Medical Record to Patient

```bash
curl -X POST http://localhost:8001/api/patients/1/records \
  -H "Content-Type: application/json" \
  -d '{
    "diagnosis": "Common cold",
    "treatment": "Rest and fluids"
  }'
```

**Expected:** HTTP 201 Created with the newly created medical record object

### 4.6 Error Handling - Invalid Patient ID

```bash
curl -X GET http://localhost:8001/api/patients/9999
```

**Expected:** HTTP 404 with error message: `{"error":"Patient not found"}`

---

## 5. CORS Verification

### Test CORS from Browser Console

Open your browser's developer console and run:

```javascript
fetch("http://localhost:8000/api/cases")
  .then((res) => res.json())
  .then((data) => console.log("Police API:", data))
  .catch((err) => console.error("Error:", err));

fetch("http://localhost:8001/api/patients")
  .then((res) => res.json())
  .then((data) => console.log("Hospital API:", data))
  .catch((err) => console.error("Error:", err));
```

**Expected:** Both requests should succeed without CORS errors

### Test CORS Headers with curl

```bash
curl -X OPTIONS http://localhost:8000/api/cases -i
```

**Expected:** Response should include CORS headers like `access-control-allow-origin: *`

---

## 6. Inter-System API Communication Tests

### 6.1 Hospital Queries Police System (WITH API Key)

```bash
curl -X GET http://localhost:8000/api/shared/suspect/19900101-1234 \
  -H "X-API-Key: dev_key_12345"
```

**Expected:** HTTP 200 with suspect info:

```json
{
  "name": "Test Suspect",
  "personal_id": "19900101-1234",
  "has_active_cases": true
}
```

### 6.2 Hospital Queries Police System (WITHOUT API Key)

```bash
curl -X GET http://localhost:8000/api/shared/suspect/19900101-1234
```

**Expected:** HTTP 403 Forbidden:

```json
{ "error": "Unauthorized - Invalid or missing API key" }
```

### 6.3 Police Queries Hospital System (WITH API Key)

```bash
curl -X GET http://localhost:8001/api/shared/patient/19850615-1234 \
  -H "X-API-Key: dev_key_12345"
```

**Expected:** HTTP 200 with patient info:

```json
{
  "name": "John Doe",
  "personal_id": "19850615-1234",
  "has_records": false
}
```

### 6.4 Police Queries Hospital System (WITHOUT API Key)

```bash
curl -X GET http://localhost:8001/api/shared/patient/19850615-1234
```

**Expected:** HTTP 403 Forbidden:

```json
{ "error": "Unauthorized - Invalid or missing API key" }
```

### 6.5 Query Non-Existent Personal ID

```bash
curl -X GET http://localhost:8000/api/shared/suspect/00000000-0000 \
  -H "X-API-Key: dev_key_12345"
```

**Expected:** HTTP 404:

```json
{ "error": "Suspect not found" }
```

---

## 7. Complete End-to-End Test Scenario

Run this complete test to verify the entire workflow:

```bash
# 1. Create a new case
CASE_RESPONSE=$(curl -s -X POST http://localhost:8000/api/cases \
  -H "Content-Type: application/json" \
  -d '{"case_number":"P-2024-E2E-001","status":"active","description":"End-to-end test"}')
echo "Created Case: $CASE_RESPONSE"

# 2. Extract case ID (requires jq - install with: brew install jq)
CASE_ID=$(echo $CASE_RESPONSE | jq -r '.id')
echo "Case ID: $CASE_ID"

# 3. Add suspect to the case
SUSPECT_RESPONSE=$(curl -s -X POST http://localhost:8000/api/cases/$CASE_ID/suspects \
  -H "Content-Type: application/json" \
  -d '{"name":"Erik Andersson","personal_id":"19850312-2398"}')
echo "Created Suspect: $SUSPECT_RESPONSE"

# 4. Query hospital about this person
PATIENT_INFO=$(curl -s -X GET http://localhost:8001/api/shared/patient/19850312-2398 \
  -H "X-API-Key: dev_key_12345")
echo "Patient Info from Hospital: $PATIENT_INFO"

# 5. Create a patient record
PATIENT_RESPONSE=$(curl -s -X POST http://localhost:8001/api/patients \
  -H "Content-Type: application/json" \
  -d '{"patient_id":"H-2024-E2E-001","name":"Test Patient E2E","personal_id":"19990909-9999"}')
echo "Created Patient: $PATIENT_RESPONSE"

# 6. Extract patient ID
PATIENT_ID=$(echo $PATIENT_RESPONSE | jq -r '.id')
echo "Patient ID: $PATIENT_ID"

# 7. Add medical record
RECORD_RESPONSE=$(curl -s -X POST http://localhost:8001/api/patients/$PATIENT_ID/records \
  -H "Content-Type: application/json" \
  -d '{"diagnosis":"Test diagnosis","treatment":"Test treatment"}')
echo "Created Medical Record: $RECORD_RESPONSE"

# 8. Query police about this person
SUSPECT_INFO=$(curl -s -X GET http://localhost:8000/api/shared/suspect/19990909-9999 \
  -H "X-API-Key: dev_key_12345")
echo "Suspect Info from Police: $SUSPECT_INFO"

echo "✅ End-to-end test completed!"
```

**Note:** This script requires `jq` for JSON parsing. Install it with `brew install jq` on macOS.

---

## 8. Performance & Load Test (Optional)

### Simple Load Test with curl

```bash
# Send 10 requests rapidly
for i in {1..10}; do
  curl -s http://localhost:8000/api/cases > /dev/null &
done
wait
echo "Load test completed"
```

---

## 9. Quick Verification Checklist

- [ ] Both services start without errors
- [ ] Both databases are accessible
- [ ] Health endpoints respond correctly
- [ ] Can retrieve lists of cases/patients
- [ ] Can create new cases/patients
- [ ] Can retrieve individual records by ID
- [ ] Can create suspects/medical records
- [ ] 404 errors work for invalid IDs
- [ ] CORS headers are present
- [ ] Inter-system API requires authentication
- [ ] Inter-system API works with correct key
- [ ] Inter-system API blocks requests without key

---

## 10. Troubleshooting

### Service Not Responding

```bash
# Check if services are running
lsof -i :8000  # Police system
lsof -i :8001  # Hospital system

# Check server logs for errors
cd backend/police-system && cargo run
cd backend/hospital-system && cargo run
```

### Database Connection Issues

```bash
# Verify PostgreSQL is running
brew services list | grep postgresql

# Test database connection
psql -U postgres -d police_db -c "SELECT 1;"
psql -U postgres -d hospital_db -c "SELECT 1;"
```

### Reset Databases

```bash
# Drop and recreate databases
psql -U postgres -c "DROP DATABASE IF EXISTS police_db;"
psql -U postgres -c "DROP DATABASE IF EXISTS hospital_db;"
psql -U postgres -f shared/database-schemas/police-schema.sql
psql -U postgres -f shared/database-schemas/hospital-schema.sql
```

---

## Expected Output Summary

✅ **All tests passing means:**

- Both backend services are running correctly
- Database connections are established
- All CRUD operations work
- CORS is properly configured
- Inter-system authentication is working
- Error handling is functional

🎉 **Your backend is ready for frontend integration!**
