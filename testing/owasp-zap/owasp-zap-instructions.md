# OWASP ZAP Security Testing - Quick Guide (macOS/Linux)

Quick guide for running OWASP ZAP security scans on the Police & Hospital backend REST APIs using command-line interface (headless mode).

## Overview

OWASP ZAP (Zed Attack Proxy) is an open-source web application security scanner. This guide focuses on automated API security testing for the thesis project's backend data transfer endpoints.

**Scope:** Backend API endpoints only (ports 8000, 8001)  
**Mode:** Headless/CLI only (no GUI required)  
**Target:** REST API security for data transfer between systems

---

## Prerequisites

### 1. Install OWASP ZAP

**macOS (via Homebrew):**

```bash
brew install --cask owasp-zap
```

**Linux (Ubuntu/Debian):**

```bash
# Add OWASP repository
sudo add-apt-repository ppa:zaproxy/release
sudo apt update
sudo apt install zaproxy
```

**Alternative: Direct Download**

Download from [https://www.zaproxy.org/download/](https://www.zaproxy.org/download/)

### 2. Verify Installation

```bash
# macOS
/Applications/OWASP\ ZAP.app/Contents/Java/zap.sh -version

# Linux
zap.sh -version

# Create alias for convenience (add to ~/.zshrc or ~/.bashrc)
alias zap='/Applications/OWASP\ ZAP.app/Contents/Java/zap.sh'
```

Expected output: `OWASP ZAP 2.x.x`

### 3. Start Backend Services

**Before testing**, ensure both backend services are running:

```bash
# Terminal 1: Police System (port 8000)
cd backend/police-system
cargo run

# Terminal 2: Hospital System (port 8001)
cd backend/hospital-system
cargo run

# Verify services are running
curl http://localhost:8000/health
curl http://localhost:8001/health
```

---

## Understanding ZAP Scan Types

### 1. Baseline Scan (Quick - Recommended First)

- **Time:** 1-2 minutes per target
- **Coverage:** Common vulnerabilities, passive scanning
- **Use Case:** Quick security check, CI/CD integration
- **False Positives:** Low

### 2. API Scan (Medium - Recommended)

- **Time:** 3-5 minutes per target
- **Coverage:** API-specific vulnerabilities, OpenAPI/REST testing
- **Use Case:** REST API security assessment
- **False Positives:** Low-Medium

### 3. Full Scan (Comprehensive - Thorough)

- **Time:** 10-20 minutes per target
- **Coverage:** Active attacks, all vulnerability classes
- **Use Case:** Complete security audit
- **False Positives:** Medium-High

**For this project:** API Scan is recommended as it balances thoroughness with speed and focuses on REST API security.

---

## Option 1: Automated Script (Recommended)

The project includes a pre-configured script that runs ZAP scans automatically.

### Using the Script

```bash
# Navigate to testing directory
cd testing/owasp-zap

# Make executable (first time only)
chmod +x run-all-owasp-zap-scans.sh

# Run all scans
./run-all-owasp-zap-scans.sh
```

**Time:** ~8-15 minutes total

The script will:

- Create timestamped results directory
- Run API scans on both Police (8000) and Hospital (8001) systems
- Generate HTML, JSON, and XML reports
- Check for high-severity vulnerabilities
- Exit with error code if critical issues found

### View Results

```bash
# List all scan results
ls zap-results/

# View HTML report in browser
open zap-results/scan_2025-01-30_14-35-22/police-api-scan-report.html
open zap-results/scan_2025-01-30_14-35-22/hospital-api-scan-report.html

# View JSON results (for automation)
cat zap-results/scan_2025-01-30_14-35-22/police-api-scan-report.json | jq
```

---

## Option 2: Manual Commands

Run these commands individually for more control:

### Setup

```bash
# Create results directory
mkdir -p testing/owasp-zap/zap-results
cd testing/owasp-zap/zap-results

# Define ZAP command (adjust path as needed)
# macOS:
ZAP_CMD="/Applications/OWASP\ ZAP.app/Contents/Java/zap.sh"
# Linux:
ZAP_CMD="zap.sh"
```

### Basic Baseline Scan

Quick security check with passive scanning only:

```bash
# Police System (port 8000)
$ZAP_CMD -cmd \
  -quickurl http://localhost:8000 \
  -quickout police-baseline-report.html

# Hospital System (port 8001)
$ZAP_CMD -cmd \
  -quickurl http://localhost:8001 \
  -quickout hospital-baseline-report.html
```

### API Scan (Recommended)

Focused REST API security testing:

```bash
# Police System API Scan
$ZAP_CMD -cmd \
  -port 8090 \
  -quickurl http://localhost:8000 \
  -quickprogress \
  -config api.key=your-api-key \
  -config connection.timeoutInSecs=60 \
  -autorun /path/to/api-scan-config.yaml

# Hospital System API Scan
$ZAP_CMD -cmd \
  -port 8091 \
  -quickurl http://localhost:8001 \
  -quickprogress \
  -config api.key=your-api-key \
  -config connection.timeoutInSecs=60 \
  -autorun /path/to/api-scan-config.yaml
```

### Full Scan with Active Attacks

Comprehensive security assessment:

```bash
# Police System Full Scan
$ZAP_CMD -cmd \
  -port 8090 \
  -quickurl http://localhost:8000 \
  -quickprogress \
  -config spider.maxDuration=5 \
  -config ajaxSpider.maxDuration=5 \
  -quickout police-full-scan-report.html

# Hospital System Full Scan
$ZAP_CMD -cmd \
  -port 8091 \
  -quickurl http://localhost:8001 \
  -quickprogress \
  -config spider.maxDuration=5 \
  -config ajaxSpider.maxDuration=5 \
  -quickout hospital-full-scan-report.html
```

### Generate Multiple Report Formats

```bash
# HTML Report (visual)
$ZAP_CMD -cmd -quickurl http://localhost:8000 -quickout report.html

# JSON Report (for automation)
$ZAP_CMD -cmd -quickurl http://localhost:8000 -quickout report.json

# XML Report (for integration)
$ZAP_CMD -cmd -quickurl http://localhost:8000 -quickout report.xml
```

---

## Interpreting Results

### Risk Levels

ZAP categorizes findings by risk level:

| Risk Level    | Color  | Meaning                             | Action Required       |
| ------------- | ------ | ----------------------------------- | --------------------- |
| High          | Red    | Critical security vulnerabilities   | Fix immediately       |
| Medium        | Orange | Significant security issues         | Fix before production |
| Low           | Yellow | Minor security concerns             | Consider fixing       |
| Informational | Blue   | Best practice violations or notices | Optional improvements |

### Common Findings for Development Environments

#### Expected (Acceptable for Development) ⚠️

1. **Missing Anti-CSRF Tokens**

   - **Why:** No authentication implemented yet
   - **Action:** Implement when adding auth

2. **HTTP Only (No HTTPS)**

   - **Why:** Development mode, localhost testing
   - **Action:** Use TLS/HTTPS in production

3. **CORS Allows All Origins**

   - **Why:** Development mode (`allow_any_origin()`)
   - **Action:** Restrict CORS in production

4. **Server Header Disclosure**

   - **Why:** Actix-web default headers
   - **Action:** Suppress in production

5. **Content Security Policy Not Defined**
   - **Why:** API endpoints, not serving HTML
   - **Action:** Add CSP headers for production

#### Red Flags (Fix Immediately) 🚨

1. **SQL Injection Vulnerabilities**

   - **Why:** Direct SQL without prepared statements
   - **Action:** Already mitigated with SQLx, verify

2. **Path Traversal**

   - **Why:** Improper input validation
   - **Action:** Add path validation

3. **XML External Entity (XXE)**

   - **Why:** Unsafe XML parsing
   - **Action:** Not applicable (JSON only)

4. **Command Injection**

   - **Why:** Unsafe system calls
   - **Action:** Avoid system calls, validate input

5. **Authentication Bypass**
   - **Why:** No authentication implemented
   - **Action:** Expected for thesis, note limitation

### Reading HTML Reports

1. **Summary Dashboard**

   - Total alerts by risk level
   - Total URLs tested
   - Total requests made

2. **Alert Details**

   - Description of vulnerability
   - URL(s) affected
   - Parameter(s) involved
   - Evidence (request/response)
   - Solution recommendations
   - CWE/WASC references

3. **Site Tree**
   - All discovered endpoints
   - Request/response history

### Reading JSON Reports (Automation)

```bash
# Count alerts by risk level
cat report.json | jq '.site[].alerts | group_by(.riskdesc) | map({risk: .[0].riskdesc, count: length})'

# List all high-risk alerts
cat report.json | jq '.site[].alerts[] | select(.riskcode == "3") | {name: .name, url: .url}'

# Extract all URLs tested
cat report.json | jq '.site[].alerts[].instances[].uri' | sort -u
```

---

## Scanning Specific Endpoints

### Test Individual API Endpoints

```bash
# Test specific endpoint patterns
$ZAP_CMD -cmd \
  -quickurl http://localhost:8000/suspects \
  -quickout suspects-endpoint-scan.html

$ZAP_CMD -cmd \
  -quickurl http://localhost:8001/patients/flagged \
  -quickout flagged-patients-scan.html
```

### Test Inter-System APIs

```bash
# Test shared API endpoints
$ZAP_CMD -cmd \
  -quickurl http://localhost:8000/api/shared/suspects \
  -quickout police-shared-api-scan.html

$ZAP_CMD -cmd \
  -quickurl http://localhost:8001/api/shared/patients \
  -quickout hospital-shared-api-scan.html
```

---

## Advanced Configuration

### Custom Scan Policy

Create `api-scan-policy.yaml`:

```yaml
env:
  contexts:
    - name: "Police API"
      urls:
        - "http://localhost:8000"
      includePaths:
        - "http://localhost:8000/suspects.*"
        - "http://localhost:8000/api/shared.*"
      excludePaths: []

  parameters:
    failOnError: true
    failOnWarning: false
    progressToStdout: true

jobs:
  - type: spider
    parameters:
      maxDuration: 5

  - type: passiveScan-wait
    parameters:
      maxDuration: 10

  - type: activeScan
    parameters:
      maxDuration: 10
      policy: "API-Minimal"
```

### Run with Custom Policy

```bash
$ZAP_CMD -cmd -autorun api-scan-policy.yaml
```

---

## Troubleshooting

### "Command not found: zap.sh"

**Solution:** Add ZAP to PATH or use full path

```bash
# Add to ~/.zshrc or ~/.bashrc
export PATH="/Applications/OWASP ZAP.app/Contents/Java:$PATH"
alias zap='zap.sh'
```

### "Connection refused" or "Failed to connect"

**Solution:** Ensure backends are running

```bash
# Check if services are running
curl http://localhost:8000/health
curl http://localhost:8001/health

# If not running, start them
cd backend/police-system && cargo run  # Terminal 1
cd backend/hospital-system && cargo run  # Terminal 2
```

### "Permission denied"

**Solution:** Make script executable

```bash
chmod +x run-all-owasp-zap-scans.sh
```

### Scan takes too long

**Solution:** Use baseline scan or reduce scan duration

```bash
# Quick baseline scan (1-2 minutes)
$ZAP_CMD -cmd -quickurl http://localhost:8000 -quickout report.html

# Limit scan duration
$ZAP_CMD -cmd \
  -quickurl http://localhost:8000 \
  -config spider.maxDuration=2 \
  -config ajaxSpider.maxDuration=2 \
  -quickout report.html
```

### "Address already in use"

**Solution:** ZAP proxy port conflict

```bash
# Use different proxy port
$ZAP_CMD -cmd -port 8092 -quickurl http://localhost:8000 -quickout report.html
```

---

## Integration with CI/CD

### Exit Codes

The automated script returns:

- `0` - Success (no high-risk issues)
- `1` - Failure (high-risk issues found)

### Example GitHub Actions

```yaml
- name: Run OWASP ZAP Scans
  run: |
    cd testing/owasp-zap
    ./run-all-owasp-zap-scans.sh

- name: Upload ZAP Reports
  uses: actions/upload-artifact@v3
  with:
    name: zap-reports
    path: testing/owasp-zap/zap-results/
```

---

## Best Practices

### Before Scanning

1. ✅ Start backend services
2. ✅ Verify health endpoints respond
3. ✅ Load seed data in database
4. ✅ Close other applications to reduce noise

### During Scanning

1. ⏸️ Don't modify backend code
2. ⏸️ Don't restart services
3. ⏸️ Monitor backend logs for errors

### After Scanning

1. 📊 Review HTML reports for visual analysis
2. 🤖 Parse JSON reports for automation
3. 📝 Document findings in thesis
4. 🔧 Fix high-risk issues before production

---

## Quick Reference

```bash
# Install
brew install --cask owasp-zap

# Verify
zap.sh -version

# Start services
cd backend/police-system && cargo run
cd backend/hospital-system && cargo run

# Run automated script
cd testing/owasp-zap
./run-all-owasp-zap-scans.sh

# View results
open zap-results/scan_*/police-api-scan-report.html
```

---

## What Gets Tested

| Test | Target              | Focus                      | Time           |
| ---- | ------------------- | -------------------------- | -------------- |
| 1    | Police API (8000)   | REST endpoint security     | 4-6 min        |
| 2    | Hospital API (8001) | REST endpoint security     | 4-6 min        |
| 3    | Shared APIs         | Cross-system communication | Included above |
| 4    | Health Endpoints    | Basic connectivity         | Included above |

**Total Time:** ~8-15 minutes

---

## Common Vulnerabilities Checked

- ✅ SQL Injection (already protected by SQLx)
- ✅ Cross-Site Scripting (XSS)
- ✅ Path Traversal
- ✅ Server Information Disclosure
- ✅ CORS Misconfiguration
- ✅ Insecure HTTP Methods
- ✅ Missing Security Headers
- ✅ Unvalidated Redirects
- ✅ Cookie Security Issues
- ✅ Input Validation

---

## Resources

- [OWASP ZAP Official Documentation](https://www.zaproxy.org/docs/)
- [ZAP Command Line Options](https://www.zaproxy.org/docs/desktop/cmdline/)
- [ZAP API Documentation](https://www.zaproxy.org/docs/api/)
- [Automation Framework](https://www.zaproxy.org/docs/automate/automation-framework/)

---
