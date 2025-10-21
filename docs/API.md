# API Documentation

Complete API reference for the Police and Hospital backend systems.

## Table of Contents

- [Overview](#overview)
- [Base URLs](#base-urls)
- [Authentication](#authentication)
- [Error Responses](#error-responses)
- [Data Models](#data-models)
- [Police System API](#police-system-api)
- [Hospital System API](#hospital-system-api)
- [Inter-System Communication](#inter-system-communication)
- [Common Use Cases](#common-use-cases)

---

## Overview

This project consists of two independent backend services that communicate with each other:

- **Police System** - Manages suspects and criminal records
- **Hospital System** - Manages patient records

Both systems use PostgreSQL with Foreign Data Wrapper (FDW) for automatic flag synchronization.

## Base URLs

| Service         | URL                     | Port |
| --------------- | ----------------------- | ---- |
| Police System   | `http://localhost:8000` | 8000 |
| Hospital System | `http://localhost:8001` | 8001 |

## Authentication

**Current Status:** No authentication required (development mode)

**Future Implementation:** JWT-based authentication

- Set `Authorization: Bearer <token>` header
- Token obtained via `/auth/login` endpoint (to be implemented)
- Admin vs regular user roles (to be implemented)

## Error Responses

All errors follow this JSON format:

```json
{
  "error": "Error message description"
}
```

### Common HTTP Status Codes

| Code | Meaning               | When Used                  |
| ---- | --------------------- | -------------------------- |
| 200  | OK                    | Successful GET/PUT request |
| 201  | Created               | Successful POST request    |
| 204  | No Content            | Successful DELETE request  |
| 404  | Not Found             | Resource doesn't exist     |
| 500  | Internal Server Error | Database or server error   |

---

## Data Models

### Suspect (Police System)

```json
{
  "id": 1,
  "full_name": "Erik Andersson",
  "personal_id": "19850312-2398",
  "flag": false
}
```

### Patient (Hospital System)

```json
{
  "id": 1,
  "full_name": "Erik Andersson",
  "personal_id": "19850312-2398",
  "flag": false
}
```

**Personal ID Format:** Swedish format `YYYYMMDD-XXXX`

- YYYY = Year (4 digits)
- MM = Month (2 digits)
- DD = Day (2 digits)
- XXXX = Unique identifier (4 digits)

---

## Police System API

Base URL: `http://localhost:8000`

### Health Check

Check if the police system is running.

**Endpoint:** `GET /health`

**Response:** `200 OK`

```json
{
  "status": "healthy",
  "service": "police-system"
}
```

**Example:**

```bash
curl http://localhost:8000/health
```

---

### Get All Suspects

Retrieve a list of all suspects in the database.

**Endpoint:** `GET /suspects`

**Response:** `200 OK`

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
]
```

**Example:**

```bash
curl http://localhost:8000/suspects
```

---

### Get Suspect by ID

Retrieve a specific suspect by their database ID.

**Endpoint:** `GET /suspects/{id}`

**Parameters:**

- `id` (path, integer) - Suspect's database ID

**Response:** `200 OK`

```json
{
  "id": 1,
  "full_name": "Erik Andersson",
  "personal_id": "19850312-2398",
  "flag": false
}
```

**Error Response:** `404 Not Found`

```json
{
  "error": "Suspect not found"
}
```

**Example:**

```bash
curl http://localhost:8000/suspects/1
```

---

### Get Suspect by Personal ID

Retrieve a suspect by their Swedish personal ID.

**Endpoint:** `GET /suspects/personal/{personal_id}`

**Parameters:**

- `personal_id` (path, string) - Swedish personal ID (YYYYMMDD-XXXX)

**Response:** `200 OK`

```json
{
  "id": 1,
  "full_name": "Erik Andersson",
  "personal_id": "19850312-2398",
  "flag": false
}
```

**Error Response:** `404 Not Found`

```json
{
  "error": "Suspect not found"
}
```

**Example:**

```bash
curl http://localhost:8000/suspects/personal/19850312-2398
```

---

### Create Suspect

Add a new suspect to the database.

**Endpoint:** `POST /suspects`

**Request Body:**

```json
{
  "full_name": "Anders Pettersson",
  "personal_id": "19920515-3421",
  "flag": false
}
```

**Response:** `201 Created`

```json
{
  "id": 11,
  "full_name": "Anders Pettersson",
  "personal_id": "19920515-3421",
  "flag": false
}
```

**Error Response:** `500 Internal Server Error`

```json
{
  "error": "Failed to create suspect"
}
```

**Example:**

```bash
curl -X POST http://localhost:8000/suspects \
  -H "Content-Type: application/json" \
  -d '{
    "full_name": "Anders Pettersson",
    "personal_id": "19920515-3421",
    "flag": false
  }'
```

---

### Update Suspect

Update an existing suspect's information.

**Endpoint:** `PUT /suspects/{id}`

**Parameters:**

- `id` (path, integer) - Suspect's database ID

**Request Body:**

```json
{
  "personal_id": "19920515-3421",
  "full_name": "Anders Gustav Pettersson",
  "flag": true
}
```

**Note:** Only `personal_id` is required. Other fields are optional.

**Response:** `200 OK`

```json
{
  "id": 11,
  "full_name": "Anders Gustav Pettersson",
  "personal_id": "19920515-3421",
  "flag": true
}
```

**Error Response:** `404 Not Found`

```json
{
  "error": "Suspect not found"
}
```

**Example:**

```bash
curl -X PUT http://localhost:8000/suspects/11 \
  -H "Content-Type: application/json" \
  -d '{
    "personal_id": "19920515-3421",
    "full_name": "Anders Gustav Pettersson",
    "flag": true
  }'
```

---

### Update Suspect Flag

Update a suspect's flag status. **This triggers automatic synchronization to the hospital database.**

**Endpoint:** `PUT /suspects/{personal_id}/flag`

**Parameters:**

- `personal_id` (path, string) - Swedish personal ID (YYYYMMDD-XXXX)

**Request Body:**

```json
{
  "flag": true
}
```

**Response:** `200 OK`

```json
{
  "id": 1,
  "full_name": "Erik Andersson",
  "personal_id": "19850312-2398",
  "flag": true
}
```

**Error Response:** `404 Not Found`

```json
{
  "error": "Suspect not found"
}
```

**Example:**

```bash
curl -X PUT http://localhost:8000/suspects/19850312-2398/flag \
  -H "Content-Type: application/json" \
  -d '{"flag": true}'
```

**Important:** When you update a flag in the police system, it automatically synchronizes to the hospital system via PostgreSQL triggers. No additional API calls are needed.

---

### Delete Suspect

Remove a suspect from the database.

**Endpoint:** `DELETE /suspects/{id}`

**Parameters:**

- `id` (path, integer) - Suspect's database ID

**Response:** `204 No Content`

**Error Response:** `404 Not Found`

```json
{
  "error": "Suspect not found"
}
```

**Example:**

```bash
curl -X DELETE http://localhost:8000/suspects/11
```

---

### Shared API: Get All Suspects (For Hospital)

**Endpoint:** `GET /api/shared/suspects`

This endpoint allows the hospital system to retrieve all suspects for cross-referencing.

**Response:** `200 OK`

```json
[
  {
    "id": 1,
    "full_name": "Erik Andersson",
    "personal_id": "19850312-2398",
    "flag": false
  }
]
```

**Example:**

```bash
curl http://localhost:8000/api/shared/suspects
```

---

### Shared API: Check Suspect Record (For Hospital)

**Endpoint:** `GET /api/shared/suspects/{personal_id}`

This endpoint allows the hospital system to check if a specific person has a police record.

**Parameters:**

- `personal_id` (path, string) - Swedish personal ID

**Response:** `200 OK` (has record)

```json
{
  "id": 1,
  "full_name": "Erik Andersson",
  "personal_id": "19850312-2398",
  "flag": true
}
```

**Response:** `404 Not Found` (no record)

```json
{
  "error": "No suspect record found",
  "personal_id": "19850312-2398"
}
```

**Example:**

```bash
curl http://localhost:8000/api/shared/suspects/19850312-2398
```

---

## Hospital System API

Base URL: `http://localhost:8001`

### Health Check

Check if the hospital system is running.

**Endpoint:** `GET /health`

**Response:** `200 OK`

```json
{
  "status": "healthy",
  "service": "hospital-system"
}
```

**Example:**

```bash
curl http://localhost:8001/health
```

---

### Get All Patients

Retrieve a list of all patients in the database.

**Endpoint:** `GET /patients`

**Response:** `200 OK`

```json
[
  {
    "id": 1,
    "full_name": "Erik Andersson",
    "personal_id": "19850312-2398",
    "flag": true
  },
  {
    "id": 2,
    "full_name": "Anna Karlsson",
    "personal_id": "19900204-1457",
    "flag": true
  }
]
```

**Example:**

```bash
curl http://localhost:8001/patients
```

---

### Get Patient by ID

Retrieve a specific patient by their database ID.

**Endpoint:** `GET /patients/{id}`

**Parameters:**

- `id` (path, integer) - Patient's database ID

**Response:** `200 OK`

```json
{
  "id": 1,
  "full_name": "Erik Andersson",
  "personal_id": "19850312-2398",
  "flag": true
}
```

**Error Response:** `404 Not Found`

```json
{
  "error": "Patient not found"
}
```

**Example:**

```bash
curl http://localhost:8001/patients/1
```

---

### Get Patient by Personal ID

Retrieve a patient by their Swedish personal ID.

**Endpoint:** `GET /patients/personal/{personal_id}`

**Parameters:**

- `personal_id` (path, string) - Swedish personal ID (YYYYMMDD-XXXX)

**Response:** `200 OK`

```json
{
  "id": 1,
  "full_name": "Erik Andersson",
  "personal_id": "19850312-2398",
  "flag": true
}
```

**Error Response:** `404 Not Found`

```json
{
  "error": "Patient not found"
}
```

**Example:**

```bash
curl http://localhost:8001/patients/personal/19850312-2398
```

---

### Get Flagged Patients

Retrieve all patients who have been flagged by the police system.

**Endpoint:** `GET /patients/flagged`

**Response:** `200 OK`

```json
[
  {
    "id": 2,
    "full_name": "Anna Karlsson",
    "personal_id": "19900204-1457",
    "flag": true
  },
  {
    "id": 4,
    "full_name": "Maria Svensson",
    "personal_id": "19891215-0912",
    "flag": true
  }
]
```

**Note:** Flags are automatically synchronized from the police database. You cannot manually flag patients through the API.

**Example:**

```bash
curl http://localhost:8001/patients/flagged
```

---

### Create Patient

Register a new patient in the hospital database.

**Endpoint:** `POST /patients`

**Request Body:**

```json
{
  "full_name": "Ingrid Lindberg",
  "personal_id": "19880623-7542",
  "flag": false
}
```

**Response:** `201 Created`

```json
{
  "id": 9,
  "full_name": "Ingrid Lindberg",
  "personal_id": "19880623-7542",
  "flag": false
}
```

**Error Response:** `500 Internal Server Error`

```json
{
  "error": "Failed to create patient"
}
```

**Example:**

```bash
curl -X POST http://localhost:8001/patients \
  -H "Content-Type: application/json" \
  -d '{
    "full_name": "Ingrid Lindberg",
    "personal_id": "19880623-7542",
    "flag": false
  }'
```

---

### Update Patient

Update an existing patient's information.

**Endpoint:** `PUT /patients/{id}`

**Parameters:**

- `id` (path, integer) - Patient's database ID

**Request Body:**

```json
{
  "personal_id": "19880623-7542",
  "full_name": "Ingrid Maria Lindberg",
  "flag": false
}
```

**Note:** Only `personal_id` is required. Other fields are optional. However, you typically should not manually change the `flag` field as it's managed by the police system.

**Response:** `200 OK`

```json
{
  "id": 9,
  "full_name": "Ingrid Maria Lindberg",
  "personal_id": "19880623-7542",
  "flag": false
}
```

**Error Response:** `404 Not Found`

```json
{
  "error": "Patient not found"
}
```

**Example:**

```bash
curl -X PUT http://localhost:8001/patients/9 \
  -H "Content-Type: application/json" \
  -d '{
    "personal_id": "19880623-7542",
    "full_name": "Ingrid Maria Lindberg"
  }'
```

---

### Delete Patient

Remove a patient from the database.

**Endpoint:** `DELETE /patients/{id}`

**Parameters:**

- `id` (path, integer) - Patient's database ID

**Response:** `204 No Content`

**Error Response:** `404 Not Found`

```json
{
  "error": "Patient not found"
}
```

**Example:**

```bash
curl -X DELETE http://localhost:8001/patients/9
```

---

### Shared API: Get All Patients (For Police)

**Endpoint:** `GET /api/shared/patients`

This endpoint allows the police system to retrieve all patients for cross-referencing.

**Response:** `200 OK`

```json
[
  {
    "id": 1,
    "full_name": "Erik Andersson",
    "personal_id": "19850312-2398",
    "flag": true
  }
]
```

**Example:**

```bash
curl http://localhost:8001/api/shared/patients
```

---

### Shared API: Get Flagged Patients (For Police)

**Endpoint:** `GET /api/shared/patients/flagged`

This endpoint allows the police system to see which patients have been flagged.

**Response:** `200 OK`

```json
[
  {
    "id": 2,
    "full_name": "Anna Karlsson",
    "personal_id": "19900204-1457",
    "flag": true
  }
]
```

**Example:**

```bash
curl http://localhost:8001/api/shared/patients/flagged
```

---

### Shared API: Check Patient Record (For Police)

**Endpoint:** `GET /api/shared/patients/{personal_id}`

This endpoint allows the police system to check if a specific person has hospital records.

**Parameters:**

- `personal_id` (path, string) - Swedish personal ID

**Response:** `200 OK` (has record)

```json
{
  "id": 1,
  "full_name": "Erik Andersson",
  "personal_id": "19850312-2398",
  "flag": true
}
```

**Response:** `404 Not Found` (no record)

```json
{
  "error": "No patient record found",
  "personal_id": "19850312-2398"
}
```

**Example:**

```bash
curl http://localhost:8001/api/shared/patients/19850312-2398
```

---

## Inter-System Communication

### How Hospital Queries Police Records

The hospital system can check if a patient has a police record using the shared API:

```bash
# Check if patient Erik Andersson (19850312-2398) has a police record
curl http://localhost:8000/api/shared/suspects/19850312-2398
```

**Use Case:** When admitting a patient, the hospital can query the police system to check for any criminal records.

**Response if found:**

```json
{
  "id": 1,
  "full_name": "Erik Andersson",
  "personal_id": "19850312-2398",
  "flag": true
}
```

**Response if not found:**

```json
{
  "error": "No suspect record found",
  "personal_id": "19850312-2398"
}
```

---

### How Police Queries Hospital Records

The police system can check if a suspect has hospital records using the shared API:

```bash
# Check if suspect Anna Karlsson (19900204-1457) has medical records
curl http://localhost:8001/api/shared/patients/19900204-1457
```

**Use Case:** During an investigation, police can query the hospital system to verify if a suspect has medical records.

**Response if found:**

```json
{
  "id": 2,
  "full_name": "Anna Karlsson",
  "personal_id": "19900204-1457",
  "flag": true
}
```

**Response if not found:**

```json
{
  "error": "No patient record found",
  "personal_id": "19900204-1457"
}
```

---

### Automatic Flag Synchronization

When police flag a suspect, the flag automatically synchronizes to the hospital database via PostgreSQL triggers:

```bash
# 1. Police flags a suspect
curl -X PUT http://localhost:8000/suspects/19850312-2398/flag \
  -H "Content-Type: application/json" \
  -d '{"flag": true}'

# 2. Flag automatically syncs to hospital database (no API call needed!)

# 3. Hospital can now see the flagged patient
curl http://localhost:8001/patients/flagged
```

**Important:** No additional API calls are needed for flag synchronization. The database handles this automatically using Foreign Data Wrapper (FDW) and triggers.

---

## Common Use Cases

### 1. Register New Suspect and Check Hospital Records

```bash
# Step 1: Create suspect in police system
curl -X POST http://localhost:8000/suspects \
  -H "Content-Type: application/json" \
  -d '{
    "full_name": "Test Person",
    "personal_id": "19900101-1234",
    "flag": false
  }'

# Step 2: Check if this person has hospital records
curl http://localhost:8001/api/shared/patients/19900101-1234
```

---

### 2. Flag a Suspect and Verify Hospital Synchronization

```bash
# Step 1: Flag the suspect in police system
curl -X PUT http://localhost:8000/suspects/19850312-2398/flag \
  -H "Content-Type: application/json" \
  -d '{"flag": true}'

# Step 2: Verify the flag synced to hospital
curl http://localhost:8001/patients/personal/19850312-2398

# Step 3: Get all flagged patients in hospital
curl http://localhost:8001/patients/flagged
```

---

### 3. Hospital Checks Patient for Police Records

```bash
# Step 1: Get patient from hospital
curl http://localhost:8001/patients/personal/19900204-1457

# Step 2: Check if patient has police record
curl http://localhost:8000/api/shared/suspects/19900204-1457

# Response indicates patient is in police database
```

---

### 4. Cross-Reference All Records

```bash
# Get all suspects from police
curl http://localhost:8000/suspects

# Get all patients from hospital
curl http://localhost:8001/patients

# Find overlapping records by matching personal_id
```

---

## Development Notes

### CORS Configuration

Both systems have CORS enabled for cross-origin requests:

- Police system accepts requests from `http://localhost:8001`
- Hospital system accepts requests from `http://localhost:8000`
- Both accept requests from any origin in development mode

**Production:** Remove `allow_any_origin()` and only allow specific trusted origins.

### Database Synchronization

Flag synchronization happens at the database level using:

- **PostgreSQL Foreign Data Wrapper (FDW)** - Allows cross-database queries
- **Triggers** - Automatically propagate flag changes from police to hospital

No API-level synchronization is needed.

### Future Enhancements

- JWT authentication with role-based access control
- Rate limiting per IP address
- API versioning (e.g., `/api/v1/suspects`)
- Audit logging for all data access
- Pagination for list endpoints
- Search and filtering capabilities
- Webhook notifications for flag changes

---

## Testing

### Quick Test Script

```bash
#!/bin/bash

# Test Police System
echo "Testing Police System..."
curl -s http://localhost:8000/health | jq
curl -s http://localhost:8000/suspects | jq

# Test Hospital System
echo "Testing Hospital System..."
curl -s http://localhost:8001/health | jq
curl -s http://localhost:8001/patients | jq

# Test Inter-System Communication
echo "Testing Inter-System Communication..."
curl -s http://localhost:8000/api/shared/suspects/19850312-2398 | jq
curl -s http://localhost:8001/api/shared/patients/19850312-2398 | jq

# Test Flag Synchronization
echo "Testing Flag Synchronization..."
curl -s -X PUT http://localhost:8000/suspects/19850312-2398/flag \
  -H "Content-Type: application/json" \
  -d '{"flag": true}' | jq

sleep 1

curl -s http://localhost:8001/patients/personal/19850312-2398 | jq
```

Save as `test-api.sh`, make executable with `chmod +x test-api.sh`, and run with `./test-api.sh`.

---

## Support

For issues or questions:

- Check server logs for detailed error messages
- Verify PostgreSQL is running: `brew services list`
- Confirm environment variables are set correctly in `.env` files
- Test database connectivity: `psql -U postgres -d police_db`

---

_Last Updated: 2025_
