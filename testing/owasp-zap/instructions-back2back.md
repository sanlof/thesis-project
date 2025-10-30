# Police & Hospital Systems Security Testing Guide

## Prerequisites

### 1. Install OWASP ZAP

**macOS (recommended):**

```bash
brew install --cask owasp-zap
```

Or download manually: [https://www.zaproxy.org/download/](https://www.zaproxy.org/download/)

---

### 2. Start Your Backend Services

**Terminal 1 – Police System**

```bash
cd backend/police-system
cargo run
```

**Terminal 2 – Hospital System**

```bash
cd backend/hospital-system
cargo run
```

**Verify both are running:**

```bash
curl http://localhost:8000/health
curl http://localhost:8001/health
```

---

## Quick Start: Automated Scan

### Step 1. Launch OWASP ZAP

**Option 1:** Open from Applications

```bash
open -a "OWASP ZAP"
```

**Option 2:** Run from terminal

```bash
/Applications/OWASP\ ZAP.app/Contents/MacOS/OWASP-ZAP
```

---

### Step 2. Run Automated Scan

1. Click **“Automated Scan”** on the welcome screen.
2. Enter URL: `http://localhost:8000`
3. Click **“Attack”**.
4. Wait for the scan to complete (~5–10 minutes).
5. Repeat for `http://localhost:8001`.

---

### Step 3. View Results

- **Alerts tab:** Shows vulnerabilities
- **History tab:** Lists all requests
- **Spider tab:** Displays discovered endpoints

---

## Advanced: Manual Testing

### Step 1. Configure ZAP

**Local Proxy Settings**

- Address: `localhost`
- Port: `8080` (default)

**API Settings**

- Enable API access
- Note your API key

---

### Step 2. Explore APIs

Add endpoints manually in **Manual Explore → URL**.

#### Police System (Port 8000)

```
http://localhost:8000/health
http://localhost:8000/suspects
http://localhost:8000/suspects/1
http://localhost:8000/suspects/personal/19850312-2398
http://localhost:8000/suspects/19850312-2398/flag
http://localhost:8000/api/shared/suspects
http://localhost:8000/api/shared/suspects/19850312-2398
```

#### Hospital System (Port 8001)

```
http://localhost:8001/health
http://localhost:8001/patients
http://localhost:8001/patients/1
http://localhost:8001/patients/personal/19850312-2398
http://localhost:8001/patients/flagged
http://localhost:8001/api/shared/patients
http://localhost:8001/api/shared/patients/flagged
http://localhost:8001/api/shared/patients/19850312-2398
```

---

### Step 3. Spider Scan

1. Right-click `http://localhost:8000`
2. Select **Attack → Spider**
3. Start and wait for completion
4. Repeat for `http://localhost:8001`

---

### Step 4. Active Scan

1. Right-click `http://localhost:8000`
2. Select **Attack → Active Scan**
3. Start and wait (~10–15 minutes)
4. Repeat for `http://localhost:8001`

---

### Step 5. Review Alerts

| Severity      | Meaning                           |
| ------------- | --------------------------------- |
| 🔴 **High**   | Critical issues – fix immediately |
| 🟠 **Medium** | Important vulnerabilities         |
| 🟡 **Low**    | Minor issues                      |
| 🔵 **Info**   | Best practice recommendations     |

---

## Testing Specific Vulnerabilities

### SQL Injection

Send manually via Request Editor:

```
GET http://localhost:8000/suspects/personal/19850312-2398'OR'1'='1
```

✅ Expected: Returns `404` (SQLx prevents injection)

---

### XSS Test

**POST Request**

```json
{
  "full_name": "<script>alert('XSS')</script>",
  "personal_id": "19990101-1234",
  "flag": false
}
```

✅ Expected: Input sanitized, no alert triggered

---

### CORS Test

Add header:

```
Origin: http://evil.com
```

✅ Expected (development): `Access-Control-Allow-Origin: *`  
✅ Expected (production): Restricted origins only

---

### Authentication Bypass

Try accessing endpoints without credentials.

- Development: allowed (no auth)
- Production: should return `401 Unauthorized`

---

## Automated Script Testing

### ZAP Test Script

```bash
#!/bin/bash
echo "🔒 OWASP ZAP Security Testing - Police & Hospital Systems"
echo "=========================================================="

ZAP_PATH="/Applications/OWASP ZAP.app/Contents/Java"
ZAP_PORT=8080
POLICE_URL="http://localhost:8000"
HOSPITAL_URL="http://localhost:8001"
# (Add automation logic here)
```

**Run the script:**

```bash
chmod +x testing/zap-test.sh
./testing/zap-test.sh
```

⏱️ Approx. 20–30 minutes per system

---

## Expected Findings (Development Mode)

| Finding                  | Severity  | Expected               | Fix                         |
| ------------------------ | --------- | ---------------------- | --------------------------- |
| Missing CSRF Tokens      | 🟠 Medium | No CSRF protection     | Add for production          |
| CORS Misconfiguration    | 🟠 Medium | `*` allowed            | Restrict origins            |
| Missing Security Headers | 🟡 Low    | Lacking common headers | Add `X-Frame-Options`, etc. |
| HTTP Only                | 🔴 High   | No HTTPS               | Use TLS/SSL                 |
| Server Header Disclosure | 🟡 Low    | Shows `actix-web`      | Remove header               |

✅ **Good Findings:**

- SQL injection prevented by SQLx
- No XSS vulnerabilities
- Proper error handling (no stack traces)

---

## Viewing Reports

```bash
open testing/zap-results/scan*/police-report.html
open testing/zap-results/scan*/hospital-report.html
```

---

## False Positives

- Manually verify questionable findings
- Document acceptable risks (for development mode)

---

## Testing Cross-System Features

### Flag Synchronization

```bash
PUT http://localhost:8000/suspects/19850312-2398/flag
{"flag": true}
```

Verify:

```bash
GET http://localhost:8001/patients/personal/19850312-2398
```

✅ Expected: `flag: true`

---

### Cross-System API Tests

```bash
GET http://localhost:8001/api/shared/patients/19850312-2398
GET http://localhost:8000/api/shared/suspects/19850312-2398
```

---

## Comparing Tools: ZAP vs Nmap

| Tool     | Purpose                       | Time    | Output            |
| -------- | ----------------------------- | ------- | ----------------- |
| **Nmap** | Network & port scanning       | ~15 min | Text reports      |
| **ZAP**  | Web app vulnerability testing | ~30 min | HTML/JSON reports |

✅ **Recommendation:**  
Run Nmap for infrastructure scanning and ZAP for app-layer security.

---

## Troubleshooting

**ZAP won’t start**

```bash
lsof -i :8080
# Change port
ZAP_PORT=8090 ./testing/zap-test.sh
```

**Connection refused**

```bash
curl http://localhost:8000/health
curl http://localhost:8001/health
```

Restart services if needed.

---

## Production Security Checklist

Before deployment, ensure:

- ✅ HTTPS/TLS enabled
- ✅ JWT authentication
- ✅ Restricted CORS
- ✅ CSRF protection
- ✅ Security headers
- ✅ Rate limiting
- ✅ Input validation
- ✅ Audit logging
- ✅ Regular scans

---

## Additional Resources

- [ZAP Documentation](https://www.zaproxy.org/docs/)
- [OWASP Top 10](https://owasp.org/www-project-top-ten/)
- Your API Docs: `docs/API.md`
- Security Notes: `ARCHITECTURE.md#security-considerations`

---

**Happy testing! 🔒**
