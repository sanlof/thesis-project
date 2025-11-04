# Security Testing Guide: Data Transfer Security

## ⚠️ EDUCATIONAL RESEARCH PROJECT

**This guide documents penetration testing conducted on AI-generated code for academic thesis research.**

### Critical Legal Notice

**Before proceeding, understand that:**

1. **Unauthorized security testing is illegal** - Using these tools without permission can violate:

   - Computer Fraud and Abuse Act (USA)
   - Computer Misuse Act (UK)
   - Similar laws in most countries

2. **Our research was authorized:**

   - ✅ Testing on localhost only
   - ✅ Systems we own and control
   - ✅ Fictitious data only

3. **This guide is for:**
   - ✅ Understanding our research methodology
   - ✅ Academic review and replication
   - ✅ Educational purposes in authorized environments
   - ❌ NOT for testing systems you don't own

### Ethical Use Requirement

If you use OWASP ZAP or similar tools based on this guide, you must:

- Have explicit written authorization
- Only test systems you own or have permission to test
- Comply with all applicable laws
- Follow responsible disclosure practices

**Proceed only if you understand and accept these responsibilities.**

---

A focused guide for testing data-in-transit security in the Police & Hospital System thesis project using OWASP ZAP.

## Table of Contents

- [Overview](#overview)
- [Prerequisites](#prerequisites)
- [OWASP ZAP Setup](#owasp-zap-setup)
- [Scenario 1: Frontend-to-Backend Security Testing](#scenario-1-frontend-to-backend-security-testing)
- [Scenario 2: Backend-to-Backend Security Testing](#scenario-2-backend-to-backend-security-testing)
- [Key Vulnerabilities to Test](#key-vulnerabilities-to-test)
- [Interpreting Results](#interpreting-results)
- [Documentation Template](#documentation-template)

---

## Overview

This guide focuses on testing **data-in-transit security** for two communication scenarios:

1. **Frontend → Backend**: Browser (localhost:3000) communicating with Police API (localhost:8000) and Hospital API (localhost:8001)
2. **Backend → Backend**: Police system (localhost:8000) querying Hospital system (localhost:8001) via shared API endpoints

**Scope**: Encryption, sensitive data exposure, authentication mechanisms  
**Out of Scope**: Database security, server configuration, code vulnerabilities

---

## Prerequisites

### Required Software

- **OWASP ZAP Desktop**: Download from [zaproxy.org](https://www.zaproxy.org/download/)
- **Running System Components**:

  ```bash
  # Terminal 1: Police backend
  cd backend/police-system && cargo run

  # Terminal 2: Hospital backend
  cd backend/hospital-system && cargo run

  # Terminal 3: Frontend
  cd frontend && npm run dev
  ```

### Test Environment

- All services running on localhost (HTTP only)
- Swedish personal IDs (YYYYMMDD-XXXX format) used as test data
- No authentication implemented (development mode)

---

## OWASP ZAP Setup

### Initial Configuration

1. **Launch OWASP ZAP**

   - Choose "No, I do not want to persist this session" for testing
   - Select "Standard Mode" for full features

2. **Configure Local Proxy**

   - Go to: `Tools` → `Options` → `Local Proxies`
   - Set Address: `localhost`
   - Set Port: `8090` (or any available port)
   - Enable "Break on all requests" and "Break on all responses" (optional for manual testing)

3. **Browser Proxy Configuration**

   - **Firefox (Recommended)**:
     - Settings → Network Settings → Manual proxy configuration
     - HTTP Proxy: `localhost`, Port: `8090`
     - Check "Use this proxy server for all protocols"
   - **Chrome**: Use browser extension like "Proxy SwitchyOmega"

4. **Trust ZAP's Root CA Certificate**
   - In ZAP: `Tools` → `Options` → `Dynamic SSL Certificates`
   - Click "Save" to export `owasp_zap_root_ca.cer`
   - Import into browser:
     - Firefox: Settings → Privacy & Security → Certificates → View Certificates → Import
     - Trust for "Identify websites"

---

## Scenario 1: Frontend-to-Backend Security Testing

### Objective

Test security of HTTP requests from React frontend to Rust backend APIs.

### ZAP Configuration

1. **Add Target Applications**

   - In Sites tree, add: `http://localhost:8000` (Police API)
   - In Sites tree, add: `http://localhost:8001` (Hospital API)
   - In Sites tree, add: `http://localhost:3000` (Frontend)

2. **Context Setup**
   - Right-click `http://localhost:8000` → `Include in Context` → `New Context`
   - Name: "Police API"
   - Repeat for Hospital API with name "Hospital API"

### Test Procedures

#### Test 1.1: Passive Scanning for Sensitive Data Exposure

**Purpose**: Detect unencrypted sensitive data in HTTP traffic

**Steps**:

1. Open browser (configured with ZAP proxy) to `http://localhost:3000`
2. In ZAP, enable passive scanning: `Tools` → `Options` → `Passive Scanner` → Enable all rules
3. Interact with the frontend:
   - View suspects table (triggers GET `/api/police/suspects`)
   - View patients table (triggers GET `/api/hospital/patients`)
   - Toggle a flag (triggers PUT `/api/police/suspects/{personal_id}/flag`)
4. Wait 30 seconds for passive scan to complete
5. Review alerts in `Alerts` tab

**Expected Findings**:

- **HIGH**: Unencrypted HTTP transmission (No TLS/SSL)
- **MEDIUM**: Sensitive data in URL (personal IDs in path parameters)
- **MEDIUM**: Missing security headers (HSTS, X-Content-Type-Options)
- **LOW**: Information disclosure (detailed error messages)

**Pass Criteria for Development**:

- Personal IDs visible in responses (expected in dev)
- No credentials in plain text (N/A - no auth implemented)
- API responses contain only necessary data fields

#### Test 1.2: Active Scanning for API Vulnerabilities

**Purpose**: Probe for common API security issues

**Steps**:

1. In Sites tree, right-click `http://localhost:8000/suspects` → `Attack` → `Active Scan`
2. Configure scan:
   - Policy: "API Minimal" (if available) or "Default Policy"
   - Uncheck injection tests (out of scope for this guide)
   - Enable: "Information Disclosure", "Server Security", "Authentication"
3. Start scan and wait for completion (5-10 minutes)
4. Review results in `Alerts` tab, filter by "Police API" context

**Expected Findings**:

- **MEDIUM**: Missing Anti-CSRF tokens (PUT/POST requests)
- **MEDIUM**: Insufficient authentication (no API keys/tokens required)
- **LOW**: Application error disclosure (stack traces in 500 errors)

**To Test Manually**:

```bash
# Test 1: Check if API rejects malformed requests appropriately
curl -X PUT http://localhost:8000/suspects/INVALID-ID/flag \
  -H "Content-Type: application/json" \
  -d '{"flag": "not_a_boolean"}'

# Expected: 400 Bad Request or 422 Unprocessable Entity
# Finding: Check if error message leaks internal details

# Test 2: Verify CORS policy
curl -X GET http://localhost:8000/suspects \
  -H "Origin: http://malicious-site.com" \
  -v

# Expected: CORS headers should restrict to localhost:3000, localhost:8001
# Finding: Check Access-Control-Allow-Origin header
```

#### Test 1.3: Data Leakage in Responses

**Purpose**: Ensure API responses don't expose unnecessary information

**Steps**:

1. In ZAP History tab, find: `GET http://localhost:8000/suspects`
2. Right-click → `Open/Resend with Request Editor`
3. Send request and examine response body
4. Look for:
   - Database IDs (acceptable)
   - Personal IDs (expected for this system)
   - Internal system paths
   - Software version numbers
   - Database error messages

**Expected Response Structure**:

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

**Findings to Document**:

- ✅ **Acceptable**: All fields are necessary for functionality
- ⚠️ **Warning**: Personal IDs transmitted over HTTP (expected in dev, must use HTTPS in production)

---

## Scenario 2: Backend-to-Backend Security Testing

### Objective

Test security of API calls between Police and Hospital backend services.

### ZAP Configuration

1. **Configure ZAP as Upstream Proxy for Backend**

   - Since Rust backends don't natively support proxy configuration, we'll use ZAP in manual mode
   - Alternative: Use ZAP API mode (advanced)

2. **Manual Request Testing Approach**
   - We'll use ZAP's Manual Request Editor to simulate backend-to-backend calls

### Test Procedures

#### Test 2.1: Shared API Endpoint Security

**Purpose**: Test inter-system API calls for data exposure and access control

**Steps**:

1. In ZAP, go to: `Tools` → `Manual Request Editor`
2. Test Police querying Hospital system:

   ```http
   GET http://localhost:8001/api/shared/patients/19850312-2398 HTTP/1.1
   Host: localhost:8001
   User-Agent: police-system/0.1.0
   Accept: application/json
   ```

3. Send request and analyze response
4. Test unauthorized endpoint access:

   ```http
   GET http://localhost:8001/patients HTTP/1.1
   Host: localhost:8001
   User-Agent: external-system
   Accept: application/json
   ```

5. Repeat for Hospital querying Police:

   ```http
   GET http://localhost:8000/api/shared/suspects/19850312-2398 HTTP/1.1
   Host: localhost:8000
   User-Agent: hospital-system/0.1.0
   Accept: application/json
   ```

**Expected Findings**:

- **CRITICAL**: No authentication required for sensitive APIs
- **HIGH**: Shared endpoints accessible without API keys
- **MEDIUM**: No rate limiting (potential for abuse)

**Current State**:

- Both `/api/shared/*` endpoints are completely open
- CORS allows any origin (development mode)
- No logging of which system made requests

#### Test 2.2: Cross-Origin Request Forgery (CSRF) Testing

**Purpose**: Verify CORS policies prevent unauthorized cross-origin requests

**Steps**:

1. Use ZAP's Manual Request Editor
2. Send request with malicious origin:

   ```http
   GET http://localhost:8000/api/shared/suspects HTTP/1.1
   Host: localhost:8000
   Origin: http://attacker.com
   User-Agent: Mozilla/5.0
   Accept: application/json
   ```

3. Check response headers for:
   - `Access-Control-Allow-Origin`
   - `Access-Control-Allow-Methods`

**Expected Finding**:

- ⚠️ **CRITICAL (Dev Mode)**: `Access-Control-Allow-Origin: *` allows any origin
- ✅ **Production Requirement**: Should restrict to specific origins

**Test Script (using curl via ZAP)**:

```bash
# Test 1: Legitimate backend request
curl -X GET http://localhost:8001/api/shared/patients \
  -H "Origin: http://localhost:8000" \
  --proxy http://localhost:8090

# Test 2: Unauthorized origin
curl -X GET http://localhost:8001/api/shared/patients \
  -H "Origin: http://evil.com" \
  --proxy http://localhost:8090

# Compare Access-Control-Allow-Origin headers
```

#### Test 2.3: Data Synchronization Security (Flag Update)

**Purpose**: Test flag synchronization mechanism for data leakage

**Steps**:

1. In ZAP Manual Request Editor, send flag update:

   ```http
   PUT http://localhost:8000/suspects/19850312-2398/flag HTTP/1.1
   Host: localhost:8000
   Content-Type: application/json
   Content-Length: 14

   {"flag": true}
   ```

2. Immediately query hospital system to verify sync:

   ```http
   GET http://localhost:8001/patients/personal/19850312-2398 HTTP/1.1
   Host: localhost:8001
   Accept: application/json
   ```

3. Analyze both responses for:
   - Timing information (response time)
   - Error messages revealing synchronization details
   - Confirmation of data propagation

**Expected Findings**:

- ✅ Flag syncs automatically (database trigger)
- ⚠️ No audit trail of who changed the flag
- ⚠️ No notification to hospital system of changes

---

## Key Vulnerabilities to Test

### Critical Issues (Address Before Production)

| Vulnerability             | Test Method                            | Expected Result (Dev)                      | Production Requirement                 |
| ------------------------- | -------------------------------------- | ------------------------------------------ | -------------------------------------- |
| **No TLS/SSL**            | Passive scan on all HTTP traffic       | All traffic unencrypted                    | Must use HTTPS with valid certificates |
| **No Authentication**     | Access shared APIs without credentials | Succeeds                                   | Implement JWT or API keys              |
| **Open CORS Policy**      | Request from arbitrary origin          | Allowed (`Access-Control-Allow-Origin: *`) | Whitelist specific origins only        |
| **Personal Data in URLs** | Check path parameters                  | Personal IDs visible in logs               | Use POST body or encrypted tokens      |

### High Priority Issues

| Vulnerability               | Test Method             | Finding              | Mitigation                                          |
| --------------------------- | ----------------------- | -------------------- | --------------------------------------------------- |
| **Missing CSRF Tokens**     | Active scan on PUT/POST | No CSRF protection   | Implement CSRF tokens for state-changing operations |
| **No Rate Limiting**        | Rapid repeated requests | No throttling        | Add rate limiting middleware                        |
| **Detailed Error Messages** | Send malformed requests | Stack traces exposed | Use generic error messages in production            |

### Medium Priority Issues

| Vulnerability                | Test Method       | Finding                        | Mitigation                           |
| ---------------------------- | ----------------- | ------------------------------ | ------------------------------------ |
| **Missing Security Headers** | Passive scan      | No HSTS, X-Frame-Options, etc. | Add security headers middleware      |
| **Session Management**       | N/A (no sessions) | Not applicable                 | Implement when adding authentication |

---

## Interpreting Results

### ZAP Alert Risk Levels

- **High (Red)**: Exploitable vulnerabilities, immediate attention required
- **Medium (Orange)**: Potential security weaknesses, should be addressed
- **Low (Yellow)**: Minor issues or informational findings
- **Informational (Blue)**: Not a vulnerability, but worth noting

### False Positives to Ignore (Development Context)

1. **"Content Security Policy (CSP) Header Not Set"**

   - **Why**: No frontend assets served by backend
   - **Action**: Can ignore for API-only services

2. **"X-Content-Type-Options Header Missing"**

   - **Why**: JSON APIs don't need MIME sniffing protection
   - **Action**: Add in production for defense-in-depth

3. **"Private IP Disclosure"**
   - **Why**: localhost/127.0.0.1 expected in development
   - **Action**: Ignore in dev, ensure proper configuration in production

### Critical Findings Requiring Documentation

1. **Unencrypted Transmission of Personal IDs**

   - **Impact**: Swedish personal IDs (sensitive data) transmitted in plain text
   - **Document As**: Known limitation in development environment
   - **Production Plan**: Require HTTPS with TLS 1.2+

2. **Lack of Authentication on Shared APIs**

   - **Impact**: Anyone can query inter-system endpoints
   - **Document As**: Security risk requiring mitigation before deployment
   - **Production Plan**: Implement API key authentication (see Future Enhancements)

3. **Open CORS Policy**
   - **Impact**: Any website can make requests to APIs
   - **Document As**: Development convenience feature
   - **Production Plan**: Whitelist specific origins only

---

## Documentation Template

### Test Report Structure

```markdown
# Security Test Report: Data Transfer Security

**Date**: [YYYY-MM-DD]
**Tester**: [Your Name]
**System Version**: v0.1.0
**Testing Tool**: OWASP ZAP [version]

## Executive Summary

Brief overview of testing scope and major findings.

## Test Environment

- Frontend: http://localhost:3000
- Police API: http://localhost:8000
- Hospital API: http://localhost:8001
- Proxy: ZAP on localhost:8090

## Scenario 1: Frontend-to-Backend Testing

### Test 1.1: Sensitive Data Exposure

- **Risk Level**: HIGH
- **Finding**: Personal IDs transmitted over HTTP without encryption
- **Evidence**: [Screenshot from ZAP History tab]
- **Impact**: Potential interception of sensitive personal data
- **Status**: ACCEPTED (Dev environment limitation)
- **Mitigation Plan**: Implement HTTPS with Let's Encrypt certificates in production

### Test 1.2: API Security Scanning

- **Risk Level**: MEDIUM
- **Finding**: No CSRF protection on state-changing endpoints
- **Evidence**: PUT /suspects/{id}/flag succeeds without CSRF token
- **Impact**: Potential for cross-site request forgery attacks
- **Status**: TO BE ADDRESSED
- **Mitigation Plan**: Add CSRF token middleware before production deployment

## Scenario 2: Backend-to-Backend Testing

### Test 2.1: Shared API Access Control

- **Risk Level**: CRITICAL
- **Finding**: No authentication required for /api/shared/\* endpoints
- **Evidence**: Successful GET request without credentials
- **Impact**: Unauthorized access to sensitive data between systems
- **Status**: ACKNOWLEDGED (Development limitation)
- **Mitigation Plan**: Implement API key authentication before production

### Test 2.2: CORS Policy

- **Risk Level**: HIGH
- **Finding**: Access-Control-Allow-Origin: \* permits any origin
- **Evidence**: [ZAP response headers screenshot]
- **Impact**: Any website can make API requests
- **Status**: ACCEPTED (Development mode)
- **Mitigation Plan**: Configure CORS to whitelist only legitimate origins

## Summary of Findings

| Risk Level | Count | Status                             |
| ---------- | ----- | ---------------------------------- |
| Critical   | 1     | Development limitation, documented |
| High       | 3     | To be addressed before production  |
| Medium     | 4     | Planned for future implementation  |
| Low        | 7     | Accepted for thesis project scope  |

## Recommendations for Production Deployment

1. **Immediate (Pre-Production)**:

   - Implement HTTPS/TLS for all communications
   - Add API key authentication for shared endpoints
   - Restrict CORS to specific origins

2. **High Priority**:

   - Add CSRF protection tokens
   - Implement rate limiting
   - Sanitize error messages

3. **Medium Priority**:
   - Add security headers (HSTS, X-Frame-Options)
   - Implement request/response logging for audit
   - Add input validation middleware

## Conclusion

The system's current security posture is appropriate for a thesis development environment but requires significant enhancements before production deployment. The primary concern is the lack of encryption and authentication for sensitive data transfer, which must be addressed through HTTPS implementation and API key-based authentication.

## Appendices

### Appendix A: ZAP Configuration Screenshots

[Include screenshots of ZAP setup]

### Appendix B: Sample HTTP Requests/Responses

[Include examples of tested requests]

### Appendix C: Full ZAP Scan Results

[Export HTML report from ZAP: Report → Generate HTML Report]
```

---

## Exporting ZAP Reports

### Generate HTML Report

1. In ZAP: `Report` → `Generate HTML Report`
2. Select location and filename
3. Include:
   - Alert details
   - HTTP messages
   - Statistics

### Generate XML Report (for programmatic analysis)

```bash
# From ZAP API (if enabled)
curl "http://localhost:8090/JSON/core/view/alerts/?baseurl=http://localhost:8000"
```

---

## Quick Reference: Testing Checklist

### Pre-Test Setup

- [ ] All services running (police, hospital, frontend)
- [ ] ZAP proxy configured (localhost:8090)
- [ ] Browser proxy settings configured
- [ ] ZAP root CA certificate installed and trusted

### Scenario 1: Frontend-to-Backend

- [ ] Passive scan enabled
- [ ] Frontend interactions captured (view suspects, view patients, toggle flags)
- [ ] Active scan completed on `/suspects` endpoint
- [ ] Data leakage check performed
- [ ] Results documented with screenshots

### Scenario 2: Backend-to-Backend

- [ ] Manual requests sent to shared APIs
- [ ] CORS policy tested with multiple origins
- [ ] Flag synchronization tested and verified
- [ ] Unauthorized access attempts tested
- [ ] Results documented with evidence

### Post-Test

- [ ] HTML report generated
- [ ] Critical findings documented
- [ ] Production mitigation plan created
- [ ] Test report saved to `docs/SECURITY_TESTING.md`

---

## Additional Resources

- **OWASP ZAP Documentation**: [zaproxy.org/docs](https://www.zaproxy.org/docs/)
- **OWASP API Security Top 10**: [owasp.org/API-Security](https://owasp.org/www-project-api-security/)
- **Rust Security Best Practices**: [anssi-fr/rust-guide](https://github.com/anssi-fr/rust-guide)

---

**Last Updated**: January 2025  
**Version**: 1.0
