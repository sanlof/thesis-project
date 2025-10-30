Testing with OWASP ZAP
OWASP ZAP (Zed Attack Proxy) is an excellent security testing tool for your thesis project. Here's a comprehensive guide to test your Police & Hospital systems.
Prerequisites
Install OWASP ZAP
macOS (recommended):
bashbrew install --cask owasp-zap
Or download from: https://www.zaproxy.org/download/
Start Your Backend Services
bash# Terminal 1: Police System
cd backend/police-system
cargo run

# Terminal 2: Hospital System

cd backend/hospital-system
cargo run

# Verify both are running

curl http://localhost:8000/health
curl http://localhost:8001/health

Quick Start: Automated Scan

1. Launch OWASP ZAP
   bash# Open ZAP
   open -a "OWASP ZAP"

# Or from terminal

/Applications/OWASP\ ZAP.app/Contents/MacOS/OWASP-ZAP

```

### 2. Run Automated Scan

1. **Click "Automated Scan"** in the welcome screen
2. **Enter URL:** `http://localhost:8000`
3. **Click "Attack"**
4. Wait for scan to complete (~5-10 minutes)
5. Repeat for: `http://localhost:8001`

### 3. View Results

- **Alerts tab** shows vulnerabilities
- **History tab** shows all requests
- **Spider tab** shows discovered endpoints

---

## Advanced: Manual Testing

### Step 1: Configure ZAP

1. **Tools → Options → Local Proxies**
   - Address: `localhost`
   - Port: `8080` (default)

2. **Tools → Options → API**
   - Enable API
   - Note the API key

### Step 2: Explore APIs

**Add your endpoints manually:**

1. **Manual Explore → URL:** `http://localhost:8000`
2. **Add these endpoints to Sites tree:**

**Police System (Port 8000):**
```

http://localhost:8000/health
http://localhost:8000/suspects
http://localhost:8000/suspects/1
http://localhost:8000/suspects/personal/19850312-2398
http://localhost:8000/suspects/19850312-2398/flag
http://localhost:8000/api/shared/suspects
http://localhost:8000/api/shared/suspects/19850312-2398

```

**Hospital System (Port 8001):**
```

http://localhost:8001/health
http://localhost:8001/patients
http://localhost:8001/patients/1
http://localhost:8001/patients/personal/19850312-2398
http://localhost:8001/patients/flagged
http://localhost:8001/api/shared/patients
http://localhost:8001/api/shared/patients/flagged
http://localhost:8001/api/shared/patients/19850312-2398
Step 3: Spider Scan

Right-click on http://localhost:8000
Attack → Spider
Start Scan
Wait for completion
Repeat for http://localhost:8001

Step 4: Active Scan

Right-click on http://localhost:8000
Attack → Active Scan
Start Scan
Wait for completion (~10-15 minutes)
Repeat for http://localhost:8001

Step 5: Review Alerts
Click "Alerts" tab to see findings:

🔴 High - Critical security issues
🟠 Medium - Important vulnerabilities
🟡 Low - Minor issues
🔵 Informational - Best practices

Testing Specific Vulnerabilities
SQL Injection Test

Manual Request Editor
URL: http://localhost:8000/suspects/personal/19850312-2398'OR'1'='1
Send
Check response - should return 404 (SQLx prevents injection)

XSS Test

POST to: http://localhost:8000/suspects
Body:

json{
"full_name": "<script>alert('XSS')</script>",
"personal_id": "19990101-1234",
"flag": false
}

Send
Check if script is sanitized

CORS Test

Right-click endpoint
Open/Resend with Request Editor
Add header: Origin: http://evil.com
Send
Check response headers - should see Access-Control-Allow-Origin: \* (development mode)

Authentication Bypass

Try accessing endpoints without credentials
Expected: All endpoints accessible (no auth in development)
For production: Should return 401 Unauthorized

Automated Script Testing
Create a ZAP automation script:ZAP Automated Security Test ScriptCode #!/bin/bash

echo "🔒 OWASP ZAP Security Testing - Police & Hospital Systems"
echo "=========================================================="
echo ""

# Configuration

ZAP_PATH="/Applications/OWASP ZAP.app/Contents/Java"
ZAP_PORT=8080
POLICE_URL="htUsing the Script
bash# Save the script
chmod +x testing/zap-test.sh

# Run it

./testing/zap-test.sh
Time: ~20-30 minutes per system

Expected Findings (Development Mode)
⚠️ Common Vulnerabilities You'll Find:

Missing Anti-CSRF Tokens (Medium)

Expected: No CSRF protection in development
Fix: Add CSRF tokens for production

CORS Misconfiguration (Medium)

Finding: Access-Control-Allow-Origin: \*
Fix: Restrict to specific origins

Missing Security Headers (Low)

Missing: X-Frame-Options, X-Content-Type-Options
Fix: Add security headers middleware

HTTP Only (No HTTPS) (High)

Finding: Using HTTP instead of HTTPS
Fix: Deploy with TLS/SSL certificates

Server Header Disclosure (Low)

Finding: Exposes "actix-web" in headers
Fix: Remove or obfuscate server header

✅ Good Findings:

SQL Injection Prevention

SQLx prepared statements prevent injection

No XSS Vulnerabilities

Rust's type system + JSON serialization prevents XSS

Proper Error Handling

No stack traces or sensitive info in errors

Interpreting Results
View HTML Reports
bash# Open in browser
open testing/zap-results/scan*\*/police-report.html
open testing/zap-results/scan*\*/hospital-report.html

```

### Alert Severity

- 🔴 **High** - Fix immediately before production
- 🟠 **Medium** - Important for security
- 🟡 **Low** - Good to fix but not critical
- 🔵 **Informational** - Security best practices

### False Positives

Some findings may be false positives:
- Check manually if uncertain
- Document why certain findings are acceptable in development

---

## Testing Specific Features

### Test Flag Synchronization

1. **In ZAP, send PUT request:**
```

PUT http://localhost:8000/suspects/19850312-2398/flag
Content-Type: application/json

{"flag": true}

```

2. **Verify sync worked:**
```

GET http://localhost:8001/patients/personal/19850312-2398

```

3. **Check response** - flag should be `true`

### Test Cross-System API

1. **Test police querying hospital:**
```

GET http://localhost:8001/api/shared/patients/19850312-2398

```

2. **Test hospital querying police:**
```

GET http://localhost:8000/api/shared/suspects/19850312-2398

Comparing ZAP vs Nmap
ToolBest ForTimeOutputNmapNetwork/port scanning~15 minText filesOWASP ZAPWeb app vulnerabilities~30 minHTML/JSON reports
Recommendation: Use both for comprehensive testing:

Run Nmap for infrastructure scan
Run ZAP for application security

Troubleshooting
ZAP won't start
bash# Check if port 8080 is available
lsof -i :8080

# Use different port

ZAP_PORT=8090 ./testing/zap-test.sh
"Connection refused"
bash# Verify backends are running
curl http://localhost:8000/health
curl http://localhost:8001/health

# Restart if needed

cd backend/police-system && cargo run
cd backend/hospital-system && cargo run
Script hangs

ZAP daemon may take time to start
Active scans can take 15-20 minutes
Check ZAP logs: ~/Library/Application Support/ZAP/zap.log

Production Security Checklist
Before deploying, fix these ZAP findings:

Enable HTTPS/TLS
Add authentication (JWT)
Restrict CORS origins
Add CSRF protection
Add security headers
Rate limiting
Input validation
Remove server headers
Add audit logging
Regular security scans

Additional Resources

ZAP Documentation: https://www.zaproxy.org/docs/
OWASP Top 10: https://owasp.org/www-project-top-ten/
Your API Docs: docs/API.md
Security Notes: ARCHITECTURE.md#security-considerations

Happy testing! 🔒
