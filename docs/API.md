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

### Shared API Endpoints

Endpoints under `/api/shared/*` require API key authentication:

- **Header:** `X-API-Key`
- **Value:** 32+ character secure key (configured via environment variables)
- **Method:** All `/api/shared/*` routes validate the API key using constant-time comparison

**Example:**

```bash
curl http://localhost:8000/api/shared/suspects \
  -H "X-API-Key: your-api-key-here"
```

### Public Endpoints

Standard CRUD endpoints (e.g., `/suspects`, `/patients`) do not require authentication in the current implementation.

## Error Responses

All errors follow this JSON format:

```json
{
  "error": "Error message description",
  "correlation_id": "550e8400-e29b-41d4-a716-446655440000"
}
```

### Common HTTP Status Codes

| Code | Meaning               | When Used                         |
| ---- | --------------------- | --------------------------------- |
| 200  | OK                    | Successful GET/PUT request        |
| 201  | Created               | Successful POST request           |
| 204  | No Content            | Successful DELETE request         |
| 400  | Bad Request           | Invalid request format/validation |
| 401  | Unauthorized          | Missing or invalid API key        |
| 404  | Not Found             | Resource doesn't exist            |
| 429  | Too Many Requests     | Rate limit exceeded               |
| 500  | Internal Server Error | Database or server error          |

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
  "error": "Resource not found",
  "correlation_id": "550e8400-e29b-41d4-a716-446655440000"
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
  "error": "Resource not found",
  "correlation_id": "550e8400-e29b-41d4-a716-446655440000"
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

**Error Response:** `400 Bad Request`

```json
{
  "error": "Invalid request format",
  "correlation_id": "550e8400-e29b-41d4-a716-446655440000"
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

**Note:** `personal_id` is required. Other fields are optional.

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
  "error": "Resource not found",
  "correlation_id": "550e8400-e29b-41d4-a716-446655440000"
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

**Endpoint:** `POST /suspects/flag`

**Request Body:**

```json
{
  "personal_id": "19850312-2398",
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
  "error": "Resource not found",
  "correlation_id": "550e8400-e29b-41d4-a716-446655440000"
}
```

**Error Response:** `400 Bad Request`

```json
{
  "error": "Invalid request format",
  "correlation_id": "550e8400-e29b-41d4-a716-446655440000"
}
```

**Example:**

```bash
curl -X POST http://localhost:8000/suspects/flag \
  -H "Content-Type: application/json" \
  -d '{"personal_id": "19850312-2398", "flag": true}'
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
  "error": "Resource not found",
  "correlation_id": "550e8400-e29b-41d4-a716-446655440000"
}
```

**Example:**

```bash
curl -X DELETE http://localhost:8000/suspects/11
```

---

### Shared API: Get All Suspects

**Authentication Required:** API Key

**Endpoint:** `GET /api/shared/suspects`

**Headers:**

- `X-API-Key: <your-api-key>`

**Rate Limit:** 1 request/second, burst: 5

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

**Error Response:** `401 Unauthorized`

```json
{
  "error": "API key required"
}
```

**Error Response:** `429 Too Many Requests`

```json
{
  "error": "Too many requests"
}
```

**Example:**

```bash
curl http://localhost:8000/api/shared/suspects \
  -H "X-API-Key: your-api-key-here"
```

---

### Shared API: Check Suspect Record

**Authentication Required:** API Key

**Endpoint:** `GET /api/shared/suspects/{personal_id}`

**Headers:**

- `X-API-Key: <your-api-key>`

**Rate Limit:** 1 request/second, burst: 5

**Parameters:**

- `personal_id` (path, string) - Swedish personal ID

This endpoint allows the hospital system to check if a specific person has a police record.

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
  "error": "Resource not found",
  "correlation_id": "550e8400-e29b-41d4-a716-446655440000"
}
```

**Error Response:** `401 Unauthorized`

```json
{
  "error": "Invalid API key"
}
```

**Example:**

```bash
curl http://localhost:8000/api/shared/suspects/19850312-2398 \
  -H "X-API-Key: your-api-key-here"
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
  "error": "Resource not found",
  "correlation_id": "550e8400-e29b-41d4-a716-446655440000"
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
  "error": "Resource not found",
  "correlation_id": "550e8400-e29b-41d4-a716-446655440000"
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

**Error Response:** `400 Bad Request`

```json
{
  "error": "Invalid request format",
  "correlation_id": "550e8400-e29b-41d4-a716-446655440000"
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

**Note:** `personal_id` is required. Other fields are optional. However, you typically should not manually change the `flag` field as it's managed by the police system.

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
  "error": "Resource not found",
  "correlation_id": "550e8400-e29b-41d4-a716-446655440000"
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
  "error": "Resource not found",
  "correlation_id": "550e8400-e29b-41d4-a716-446655440000"
}
```

**Example:**

```bash
curl -X DELETE http://localhost:8001/patients/9
```

---

### Shared API: Get All Patients

**Authentication Required:** API Key

**Endpoint:** `GET /api/shared/patients`

**Headers:**

- `X-API-Key: <your-api-key>`

**Rate Limit:** 1 request/second, burst: 5

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

**Error Response:** `401 Unauthorized`

```json
{
  "error": "API key required"
}
```

**Example:**

```bash
curl http://localhost:8001/api/shared/patients \
  -H "X-API-Key: your-api-key-here"
```

---

### Shared API: Get Flagged Patients

**Authentication Required:** API Key

**Endpoint:** `GET /api/shared/patients/flagged`

**Headers:**

- `X-API-Key: <your-api-key>`

**Rate Limit:** 1 request/second, burst: 5

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

**Error Response:** `401 Unauthorized`

```json
{
  "error": "Invalid API key"
}
```

**Example:**

```bash
curl http://localhost:8001/api/shared/patients/flagged \
  -H "X-API-Key: your-api-key-here"
```

---

### Shared API: Check Patient Record

**Authentication Required:** API Key

**Endpoint:** `GET /api/shared/patients/{personal_id}`

**Headers:**

- `X-API-Key: <your-api-key>`

**Rate Limit:** 1 request/second, burst: 5

**Parameters:**

- `personal_id` (path, string) - Swedish personal ID

This endpoint allows the police system to check if a specific person has hospital records.

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
  "error": "Resource not found",
  "correlation_id": "550e8400-e29b-41d4-a716-446655440000"
}
```

**Error Response:** `401 Unauthorized`

```json
{
  "error": "API key required"
}
```

**Example:**

```bash
curl http://localhost:8001/api/shared/patients/19850312-2398 \
  -H "X-API-Key: your-api-key-here"
```

---

## Inter-System Communication

### How Hospital Queries Police Records

The hospital system can check if a patient has a police record using the shared API:

```bash
# Check if patient Erik Andersson (19850312-2398) has a police record
curl http://localhost:8000/api/shared/suspects/19850312-2398 \
  -H "X-API-Key: your-hospital-api-key"
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
  "error": "Resource not found",
  "correlation_id": "550e8400-e29b-41d4-a716-446655440000"
}
```

---

### How Police Queries Hospital Records

The police system can check if a suspect has hospital records using the shared API:

```bash
# Check if suspect Anna Karlsson (19900204-1457) has medical records
curl http://localhost:8001/api/shared/patients/19900204-1457 \
  -H "X-API-Key: your-police-api-key"
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
  "error": "Resource not found",
  "correlation_id": "550e8400-e29b-41d4-a716-446655440000"
}
```

---

### Automatic Flag Synchronization

When police flag a suspect, the flag automatically synchronizes to the hospital database via PostgreSQL triggers:

```bash
# 1. Police flags a suspect
curl -X POST http://localhost:8000/suspects/flag \
  -H "Content-Type: application/json" \
  -d '{"personal_id": "19850312-2398", "flag": true}'

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
curl http://localhost:8001/api/shared/patients/19900101-1234 \
  -H "X-API-Key: your-police-api-key"
```

---

### 2. Flag a Suspect and Verify Hospital Synchronization

```bash
# Step 1: Flag the suspect in police system
curl -X POST http://localhost:8000/suspects/flag \
  -H "Content-Type: application/json" \
  -d '{"personal_id": "19850312-2398", "flag": true}'

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
curl http://localhost:8000/api/shared/suspects/19900204-1457 \
  -H "X-API-Key: your-hospital-api-key"

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

### Rate Limiting

Both systems implement two-tier rate limiting:

**General Endpoints (IP-based):**

- Default: 10 requests/second, burst: 20
- Configurable via `RATE_LIMIT_PER_SECOND` and `RATE_LIMIT_BURST`

**Shared API Endpoints (API-key-based):**

- Default: 1 request/second, burst: 5
- Configurable via `SHARED_API_RATE_LIMIT_PER_SECOND` and `SHARED_API_RATE_LIMIT_BURST`

When rate limited, you'll receive:

```json
HTTP/1.1 429 Too Many Requests
Retry-After: 5

{
  "error": "Too many requests"
}
```

### CORS Configuration

Both systems have CORS enabled for cross-origin requests:

- Police system accepts requests from origins specified in `ALLOWED_ORIGINS`
- Hospital system accepts requests from origins specified in `ALLOWED_ORIGINS`
- Development: HTTP localhost origins allowed
- Production: Only HTTPS origins allowed (enforced at startup)

### Database Synchronization

Flag synchronization happens at the database level using:

- **PostgreSQL Foreign Data Wrapper (FDW)** - Allows cross-database queries
- **Triggers** - Automatically propagate flag changes from police to hospital

No API-level synchronization is needed.

### Security Features

- **TLS Support:** Optional HTTPS (configure via `ENABLE_TLS`)
- **API Key Authentication:** Required for `/api/shared/*` endpoints
- **Rate Limiting:** Two-tier system (general + strict for shared APIs)
- **Security Headers:** Comprehensive HTTP security headers automatically applied
- **Audit Logging:** All sensitive operations logged with correlation IDs
- **Input Validation:** Swedish personal ID format validation
- **Error Handling:** Generic error messages to clients, detailed logs server-side

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
curl -s http://localhost:8000/api/shared/suspects/19850312-2398 \
  -H "X-API-Key: your-api-key-here" | jq
curl -s http://localhost:8001/api/shared/patients/19850312-2398 \
  -H "X-API-Key: your-api-key-here" | jq

# Test Flag Synchronization
echo "Testing Flag Synchronization..."
curl -s -X POST http://localhost:8000/suspects/flag \
  -H "Content-Type: application/json" \
  -d '{"personal_id": "19850312-2398", "flag": true}' | jq

sleep 1

curl -s http://localhost:8001/patients/personal/19850312-2398 | jq
```

Save as `test-api.sh`, make executable with `chmod +x test-api.sh`, and run with `./test-api.sh`.

---

## Support

For issues or questions:

- Check server logs for detailed error messages (includes correlation IDs)
- Verify PostgreSQL is running: `brew services list`
- Confirm environment variables are set correctly in `.env` files
- Test database connectivity: `psql -U postgres -d police_db`

---

_Last Updated: November 2025_
