# Bug Fixes

### General fixes

Provided errors to AI followed by following prompt:

````
For each changed file, provide a unified diff (git-style) or the full new file content framed with triple backticks and the path as a filename: ```path/to/file
<full file content>
````

## Full security audit

## 🔴 Critical Vulnerabilities

### Prompt 1: Enable TLS/HTTPS for All Services

````
I have a Rust Actix-web application that currently only supports HTTP. I need to add TLS/HTTPS support for production deployment while maintaining HTTP for development.

Current setup:
- Two Actix-web backend services (police-system on port 8000, hospital-system on port 8001)
- Both bind to HTTP using: `HttpServer::new(...).bind(&server_address)?`
- Environment configuration loaded from .env files
- No TLS currently implemented

Requirements:
1. Add conditional TLS support based on environment variable `ENABLE_TLS`
2. Load certificate and private key from paths specified in `TLS_CERT_PATH` and `TLS_KEY_PATH`
3. Use `rustls` for TLS implementation (not OpenSSL)
4. When TLS is disabled, log a warning but continue with HTTP
5. Update both police-system and hospital-system backends
6. Add necessary dependencies to Cargo.toml
7. Update .env.example files with TLS configuration options
8. Include error handling for missing/invalid certificates

Files to modify:
- backend/police-system/src/main.rs
- backend/police-system/Cargo.toml
- backend/police-system/.env.example
- backend/hospital-system/src/main.rs
- backend/hospital-system/Cargo.toml
- backend/hospital-system/.env.example

Please provide the complete implementation with code for all files.

For each changed file, provide a unified diff (git-style) or the full new file content framed with triple backticks and the path as a filename: ```path/to/file
<full file content>
````

---

### Prompt 2: Implement API Key Authentication for Shared Endpoints

````
I need to add API key authentication to inter-system API endpoints in my Rust Actix-web application. These endpoints allow my police system to query the hospital system and vice versa.

Current situation:
- Hospital system has partial API key middleware at `backend/hospital-system/src/middleware/auth.rs`
- Police system has API key verification function at `backend/police-system/src/middleware/auth.rs`
- Shared endpoints at `/api/shared/*` currently have NO authentication
- Both systems need to authenticate each other using X-API-Key header

Requirements:
1. Complete the API key authentication for hospital-system's shared endpoints:
   - `/api/shared/patients`
   - `/api/shared/patients/flagged`
   - `/api/shared/patients/{personal_id}`

2. Add API key authentication to police-system's shared endpoints:
   - `/api/shared/suspects`
   - `/api/shared/suspects/{personal_id}`

3. Use constant-time comparison to prevent timing attacks
4. Read API keys from environment variables:
   - Police system uses `HOSPITAL_API_KEY` to authenticate requests TO hospital
   - Hospital system uses `POLICE_API_KEY` to validate requests FROM police

5. Return 401 Unauthorized with JSON error if:
   - X-API-Key header is missing
   - X-API-Key header is invalid

6. Log authentication failures with IP address (if available)

7. Update .env.example files with API key placeholders

8. Ensure the middleware is applied BEFORE the handler logic runs

Files to modify:
- backend/hospital-system/src/api/shared.rs
- backend/hospital-system/src/middleware/auth.rs
- backend/hospital-system/.env.example
- backend/police-system/src/api/shared.rs
- backend/police-system/src/middleware/auth.rs
- backend/police-system/.env.example

Provide complete code for all authentication middleware and endpoint modifications.

For each changed file, provide a unified diff (git-style) or the full new file content framed with triple backticks and the path as a filename: ```path/to/file
<full file content>
````

---

### Prompt 3: Restrict CORS to Specific Origins

````
My Rust Actix-web applications currently have permissive CORS policies that allow any origin in development mode. I need to restrict CORS to only allow specific, whitelisted origins.

Current situation:
- Police system (port 8000) currently allows localhost:8001 and localhost:3000
- Hospital system (port 8001) currently allows localhost:8000 and localhost:3000
- Both use Actix-cors middleware
- Development mode has relaxed CORS but still needs to be more restrictive

Requirements:
1. Remove all wildcard CORS configurations (no `Access-Control-Allow-Origin: *`)

2. Read allowed origins from environment variable `ALLOWED_ORIGINS` (comma-separated list)

3. Default allowed origins:
   - Police system: `http://localhost:8001,http://localhost:3000,http://127.0.0.1:8001,http://127.0.0.1:3000`
   - Hospital system: `http://localhost:8000,http://localhost:3000,http://127.0.0.1:8000,http://127.0.0.1:3000`

4. Configure CORS to:
   - Allow methods: GET, POST, PUT, DELETE, OPTIONS
   - Allow headers: Content-Type, Authorization, X-API-Key
   - Set max age: 3600 seconds
   - Allow credentials: true

5. For production, environment variable should contain HTTPS URLs only

6. Log the configured allowed origins on startup

7. Update .env.example files with production-ready examples

Files to modify:
- backend/police-system/src/main.rs
- backend/police-system/.env.example
- backend/hospital-system/src/main.rs
- backend/hospital-system/.env.example

Provide complete CORS configuration code for both services.

For each changed file, provide a unified diff (git-style) or the full new file content framed with triple backticks and the path as a filename: ```path/to/file
<full file content>
````

---

### Prompt 4: Move Sensitive Data from URL Paths to Request Bodies

````
My Rust Actix-web API currently exposes Swedish personal IDs (format: YYYYMMDD-XXXX) in URL paths, which causes them to be logged in browser history and server logs. I need to move this sensitive data to POST request bodies.

Current situation:
- Police system has endpoint: `PUT /suspects/{personal_id}/flag`
- This exposes personal_id in the URL path
- Frontend uses this endpoint to toggle suspect flags
- The database trigger requires personal_id to sync flags between systems

Requirements:
1. Change the endpoint from:
   `PUT /suspects/{personal_id}/flag`
   to:
   `POST /suspects/flag`

2. Create a request body struct that includes both personal_id and flag:
   ```rust
   struct FlagUpdateRequest {
       personal_id: String,
       flag: bool,
   }

3. Update the handler to:

   - Accept JSON request body with personal_id and flag
   - Validate personal_id format using existing `Suspect::validate_personal_id()`
   - Return appropriate errors (400 for invalid format, 404 for not found)
   - Maintain the same response format as before

4. Update the route configuration in `configure_suspects()`

5. Update the frontend TypeScript code to use the new endpoint:

   - Change from PUT to POST
   - Include personal_id in request body instead of URL

6. Ensure the database flag synchronization (via postgres_fdw trigger) still works

Files to modify:

- backend/police-system/src/api/suspects.rs (handler and route)
- frontend/src/components/PoliceData.tsx (fetch call)
- frontend/src/types.ts (update FlagUpdate interface if needed)

Provide complete code for all three files with the changes.

For each changed file, provide a unified diff (git-style) or the full new file content framed with triple backticks and the path as a filename: ```path/to/file
<full file content>
````

---

## 🟠 High-Priority Vulnerabilities

### Prompt 5: Add CSRF Protection Middleware

````

I need to add CSRF (Cross-Site Request Forgery) protection to my Rust Actix-web application for all state-changing operations (PUT, POST, DELETE).

Current situation:

- Two Actix-web services: police-system (port 8000) and hospital-system (port 8001)
- React frontend on port 3000 makes requests to both backends
- No CSRF protection currently implemented
- Endpoints that need protection:
  - POST /suspects/flag (police)
  - POST /suspects (police)
  - PUT /suspects/{id} (police)
  - DELETE /suspects/{id} (police)
  - POST /patients (hospital)
  - PUT /patients/{id} (hospital)
  - DELETE /patients/{id} (hospital)

Requirements:

1. Add CSRF middleware to both backend services using a Rust CSRF library

2. Configure CSRF tokens to be:

   - Sent as cookies (HttpOnly, Secure in production, SameSite=Strict)
   - Validated from X-CSRF-Token header

3. Exempt certain endpoints from CSRF:

   - All GET requests
   - /health endpoint
   - /api/shared/\* endpoints (these use API key auth instead)

4. Frontend changes:

   - Add a utility function to extract CSRF token from cookies
   - Include CSRF token in X-CSRF-Token header for all POST/PUT/DELETE requests
   - Handle 403 CSRF validation failures by refreshing the token

5. Return 403 Forbidden with JSON error if CSRF validation fails

6. Cookie configuration should respect ENABLE_TLS environment variable (Secure flag only with TLS)

Files to modify:

- backend/police-system/Cargo.toml (add CSRF dependency)
- backend/police-system/src/main.rs (add middleware)
- backend/hospital-system/Cargo.toml (add CSRF dependency)
- backend/hospital-system/src/main.rs (add middleware)
- frontend/src/utils/csrf.ts (new file - utility functions)
- frontend/src/components/PoliceData.tsx (add CSRF to requests)

Provide implementation code for all files. If actix-web-csrf doesn't exist or is outdated, use an alternative approach with custom middleware.

For each changed file, provide a unified diff (git-style) or the full new file content framed with triple backticks and the path as a filename: ```path/to/file
<full file content>

````

---

### Prompt 6: Implement Rate Limiting Per API Key

````

I need to add rate limiting to my Rust Actix-web APIs, with stricter limits for inter-system API endpoints that are authenticated with API keys.

Current situation:

- Global rate limiting exists using actix-governor
- Hospital system: 60 requests per minute (configured in config.rs)
- Police system: 10 requests per second with burst of 20
- Shared endpoints (/api/shared/\*) need stricter, per-API-key rate limiting

Requirements:

1. Keep existing global rate limiting as-is for general endpoints

2. Add stricter rate limiting for /api/shared/\* endpoints:

   - 1 request per second per API key
   - Burst size of 5 requests
   - Rate limit based on the API key (not IP address)

3. Create a custom key extractor that:

   - Extracts the X-API-Key header value
   - Uses it as the rate limit key
   - Returns error if no API key present

4. Apply the stricter rate limiter specifically to the /api/shared scope

5. Return 429 Too Many Requests with JSON error and Retry-After header

6. Log rate limit violations with the API key hash (not plaintext)

7. Make rate limits configurable via environment:
   - `SHARED_API_RATE_LIMIT_PER_SECOND` (default: 1)
   - `SHARED_API_RATE_LIMIT_BURST` (default: 5)

Files to modify:

- backend/hospital-system/src/main.rs
- backend/hospital-system/src/middleware/rate_limit.rs
- backend/hospital-system/src/config.rs
- backend/hospital-system/.env.example
- backend/police-system/src/main.rs
- backend/police-system/src/middleware/rate_limit.rs (create if doesn't exist)
- backend/police-system/.env.example

Provide complete code with the custom key extractor and rate limiter configuration.

For each changed file, provide a unified diff (git-style) or the full new file content framed with triple backticks and the path as a filename: ```path/to/file
<full file content>

````

---

### Prompt 7: Sanitize Error Messages and Remove Information Disclosure

````

My Rust Actix-web API currently returns detailed error messages that expose internal implementation details. I need to sanitize these error responses while maintaining detailed server-side logging.

Current situation:

- Database errors are logged and sometimes returned to clients
- Error responses may contain stack traces, SQL details, or connection strings
- Logging uses log::error! but errors are also exposed via HTTP responses

Requirements:

1. Update all API endpoint handlers in both systems to:

   - Log full error details server-side (keep existing log::error! calls)
   - Return generic error messages to clients
   - Never expose database errors, file paths, or internal details

2. Standard error responses:

   - 400 Bad Request: "Invalid request format"
   - 404 Not Found: "Resource not found"
   - 500 Internal Server Error: "Service temporarily unavailable"
   - 503 Service Unavailable: "Service temporarily unavailable"

3. Add a correlation ID to all error responses for debugging:

   ```json
   {
     "error": "Service temporarily unavailable",
     "correlation_id": "550e8400-e29b-41d4-a716-446655440000"
   }
   ```

4. Log the correlation ID server-side with the full error details

5. Update ALL handlers in these files:

   - backend/police-system/src/api/suspects.rs
   - backend/police-system/src/api/shared.rs
   - backend/hospital-system/src/api/patients.rs
   - backend/hospital-system/src/api/shared.rs

6. Create a helper module for error handling:

   - backend/police-system/src/utils/error_handler.rs
   - backend/hospital-system/src/utils/error_handler.rs

7. Example helper functions:
   - `handle_database_error()` - returns 500 with correlation ID
   - `handle_not_found()` - returns 404 with generic message
   - `handle_validation_error()` - returns 400 with safe error info

Provide complete code for the error handler modules and updated handlers for ALL endpoints in suspects.rs, shared.rs, and patients.rs.

For each changed file, provide a unified diff (git-style) or the full new file content framed with triple backticks and the path as a filename: ```path/to/file
<full file content>

````

---

## 🟡 Medium-Priority

### Prompt 8: Add Comprehensive Security Headers

````

I need to add security headers to all HTTP responses in my Rust Actix-web applications.

Current situation:

- Hospital system has some security headers in main.rs
- Police system has minimal security headers
- Need comprehensive security headers for defense-in-depth

Requirements:

1. Add security headers middleware to both services with:

   - X-Content-Type-Options: nosniff
   - X-Frame-Options: DENY
   - X-XSS-Protection: 1; mode=block
   - Strict-Transport-Security: max-age=31536000; includeSubDomains; preload (only if TLS enabled)
   - Content-Security-Policy: default-src 'none'
   - Referrer-Policy: no-referrer
   - Permissions-Policy: geolocation=(), microphone=(), camera=()

2. Make HSTS conditional on ENABLE_TLS environment variable

3. Configure headers using actix_web::middleware::DefaultHeaders

Files to modify:

- backend/police-system/src/main.rs
- backend/hospital-system/src/main.rs

Provide the complete security headers configuration code.

For each changed file, provide a unified diff (git-style) or the full new file content framed with triple backticks and the path as a filename: ```path/to/file
<full file content>

````

---

### Prompt 9: Implement Structured Audit Logging

````

I need to implement structured audit logging for all sensitive operations in my Rust Actix-web application.

Requirements:

1. Create an audit logging module that logs events as JSON

2. Audit log fields:

   - timestamp (RFC3339 format)
   - event_type (e.g., "FLAG_UPDATE", "PATIENT_ACCESS")
   - actor (API key hash or "internal")
   - action (e.g., "UPDATE", "READ", "DELETE")
   - resource (e.g., "suspect:hash123", "patient:hash456")
   - result ("SUCCESS" or "FAILURE")
   - ip_address (optional)

3. Add audit logging to these operations:

   - Flag updates in police system
   - All /api/shared/\* endpoint accesses
   - Create/Update/Delete operations

4. Use a separate log target called "audit"

5. Hash personal IDs before logging using existing hash_for_logging()

Files to create/modify:

- backend/police-system/src/utils/audit.rs (new file)
- backend/police-system/src/api/suspects.rs (add audit calls)
- backend/police-system/src/api/shared.rs (add audit calls)
- backend/hospital-system/src/utils/audit.rs (new file)
- backend/hospital-system/src/api/shared.rs (add audit calls)

Provide complete audit logging implementation and integration code.

For each changed file, provide a unified diff (git-style) or the full new file content framed with triple backticks and the path as a filename: ```path/to/file
<full file content>

````

---

### Prompt 10: Add Exponential Backoff to Frontend Polling

````

My React frontend polls the backend every 3 seconds, but this doesn't handle errors gracefully. I need to implement exponential backoff on consecutive errors.

Current situation:

- Custom polling hook at frontend/src/hooks/usePolling.ts
- Fixed 3-second interval
- No backoff on errors
- Polls hospital data automatically

Requirements:

1. Track consecutive error count in state

2. Calculate dynamic polling interval:

   - No errors: use configured interval (3s)
   - 1 error: 6s (2^1 \* 3s)
   - 2 errors: 12s (2^2 \* 3s)
   - 3 errors: 24s (2^3 \* 3s)
   - Max: 60s

3. Reset error count on successful fetch

4. Update the interval dynamically based on error count

5. Maintain all existing functionality (pause on inactive, refresh indicator)

Files to modify:

- frontend/src/hooks/usePolling.ts

Provide the complete updated usePolling hook with exponential backoff.

For each changed file, provide a unified diff (git-style) or the full new file content framed with triple backticks and the path as a filename: ```path/to/file
<full file content>

````

---

### Generate Secrets Properly

```bash
# For API keys (Prompt 2)
openssl rand -hex 32

# For TLS certificates (Prompt 1)
# Development:
mkcert localhost 127.0.0.1 ::1

# Production:
certbot certonly --standalone -d yourdomain.com
```

**Environment Variables**: Never commit `.env` files with real secrets.
