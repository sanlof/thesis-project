# ⚠️ STOP - READ THIS FIRST ⚠️

## YOU MAY BE BREAKING THE LAW

Before running ANY commands in this guide, understand:

1. **These are penetration testing tools**
2. **Using them without authorization is a crime in most countries**
3. **"I didn't know" is not a legal defense**

## Required Authorization Checklist

You may ONLY proceed if ALL of these are true:

- [ ] You are the system owner
- [ ] These services are running on YOUR computer
- [ ] You started these services yourself as part of the thesis project
- [ ] You understand you are responsible for any misuse

If you cannot check all boxes, **STOP NOW**.

---

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

# OWASP ZAP Automated Scan – Guide (macOS)

This guide walks you through installing and using **OWASP ZAP’s Automatic Scan** to test local web applications for common security issues.  
It assumes you’re on **macOS**, using **Homebrew**, and running local apps on `localhost`.  
By the end, you’ll have completed a working scan and reviewed relevant alerts for data transfer and SQL injection risks.

---

## Prerequisites

You’ll need:

- macOS with [Homebrew](https://brew.sh) installed
- Node.js and npm (`brew install node`)
- Rust and Cargo (`brew install rust`)
- Firefox browser (for proxy configuration)

Your local applications:

- **Frontend** – `http://localhost:3000`
- **Hospital System Backend** – `http://localhost:8000`
- **Police System Backend** – `http://localhost:8001`

Make sure each app runs locally before starting ZAP.

---

## Quick Installation

### Install OWASP ZAP via Homebrew

```bash
brew install --cask owasp-zap
```

After installation, launch ZAP from **Applications > OWASP ZAP** or by typing:

```bash
open /Applications/ZAP.app
```

---

## Start Local Applications

In separate terminal windows:

**Frontend:**

```bash
cd frontend
npm run
```

**Hospital System Backend:**

```bash
cd backend
cd hospital-system
cargo run
```

**Police System Backend:**

```bash
cd backend
cd police-system
cargo run
```

Verify you can visit each service in your browser:

- `http://localhost:3000`
- `http://localhost:8000`
- `http://localhost:8001`

---

## Starting OWASP ZAP

When ZAP opens:

- Choose **Start a new session** (default is fine).
- You’ll see the main window with the menu bar, side panels, and the “Quick Start” tab.

---

## Configure Firefox to Use ZAP as Proxy

1. In Firefox, go to:
   **Settings → General → Network Settings → Manual proxy configuration**.
2. Set:

   - **HTTP Proxy:** `127.0.0.1`
   - **Port:** `8080`

3. Check “Use this proxy server for all protocols.”
4. Click **OK**.

This routes browser traffic through ZAP so it can intercept requests and responses.

Because we’re using HTTP (not HTTPS), no certificate setup is required.

---

## Automated Scan Overview

The Automated Scan feature in ZAP combines a **crawler (spider)** and an **active scanner**:

- The spider explores your site and identifies URLs and parameters.
- The scanner sends test payloads to find vulnerabilities automatically.

You’ll use it to analyze how your frontend and backends exchange data.

---

## Running the Automated Scan

1. In ZAP’s main window, open the **Quick Start** tab.

2. Click **Automated Scan**.

3. In the “URL to attack” field, enter:

   ```
   http://localhost:3000
   ```

4. Click **Launch Browser** to open Firefox through ZAP’s proxy.

5. In that browser window:

   - Visit `http://localhost:3000`, `http://localhost:8000`, and `http://localhost:8001`
   - Interact briefly so ZAP sees traffic and adds them to its site tree.

6. Back in ZAP, confirm your targets appear under “Sites” (left panel).

   - If not, reload pages through the proxied browser.

7. Under **Automated Scan**, press **Attack** to start.

ZAP will now crawl the app and send payloads automatically.
The progress bar and scan logs appear at the bottom.

---

## Monitoring the Scan

- **Sites tree:** shows discovered endpoints.
- **Active Scan tab:** displays real-time scanning status.
- **Alerts tab:** lists vulnerabilities by severity (High, Medium, Low, Informational).

The scan can take several minutes.
You can continue browsing or pause/resume the scan anytime.

---

## Filtering and Viewing Alerts

Once the scan completes:

1. Open the **Alerts** tab.
2. Use the filter icon to show only **High** and **Medium** severity alerts.
3. Focus on findings related to **data transfer** and **SQL injection**.

---

## Understanding Key Alerts

Below are common issues you may encounter while scanning your local systems.
Each example includes a short explanation and why it matters for your setup.

---

### 1. Cleartext Transport (Sensitive Data over HTTP)

**What it means:**
Sensitive information (credentials, PII, or tokens) was sent over plain HTTP.

**Why it matters:**
Anyone intercepting network traffic could read or alter the data.
In a system with two connected databases (hospital and police), this could expose personal data or trigger unwanted syncs via `postgres_fdw`.

**Example evidence:**

```http
POST /api/patient-data HTTP/1.1
Host: localhost:8000
Content-Type: application/json

{"patient_id":"12345","ssn":"<REDACTED>"}
```

---

### 2. Sensitive Information in URL

**What it means:**
Parameters containing personal or internal data appear in URLs.

**Why it matters:**
URLs may be logged in browser history or proxy caches.

**Example:**

```
GET /records?citizenId=9876&system=hospital HTTP/1.1
```

---

### 3. Insecure Cookies

**What it means:**
Cookies lack `Secure` or `HttpOnly` flags.

**Why it matters:**
Attackers could intercept or manipulate session cookies during transfer.

**Example:**

```
Set-Cookie: session=abc123; Path=/;
```

ZAP may flag: _“Cookie without Secure flag”_ or _“Cookie without HttpOnly flag.”_

---

### 4. Missing Security Headers (Transport Layer)

**What it means:**
HTTP responses don’t include headers like `Strict-Transport-Security` or `X-Content-Type-Options`.

**Why it matters:**
Though optional for HTTP, these headers prevent accidental downgrades and improve resilience if you later enable HTTPS.

---

### 5. SQL Injection

**What it means:**
User-controlled input in a request may modify SQL queries or trigger database errors.

**Why it matters:**
Both backends connect to databases that share data via `postgres_fdw`.
A single SQL injection could compromise both systems or cause data corruption across the link.

**Example evidence:**

```http
GET /api/users?id=1' OR '1'='1
```

ZAP may show an error response like:

```
ERROR: syntax error at or near "'1'"
```

This indicates that input was interpreted as part of a SQL query.

---

### 6. Sensitive Data in Responses

**What it means:**
Endpoints return personal or internal identifiers that could reveal private data.

**Example:**

```http
HTTP/1.1 200 OK
Content-Type: application/json

{"name":"<REDACTED>","db_id":12,"system":"police"}
```

**Why it matters:**
Even in internal systems, unnecessary exposure of identifiers increases risk when combined with other findings.

---

## Exporting Scan Results

To save or share your findings:

1. In ZAP’s menu, go to **Report → Generate Report**.
2. Choose a format:

   - **HTML** (good for browsing)
   - **XML** (good for import into tools)

3. Specify a filename and click **Generate**.

You’ll find the report in your chosen directory with full alert details.

---

## Troubleshooting Common Issues

| Problem                   | Likely Cause                      | Fix                                                    |
| ------------------------- | --------------------------------- | ------------------------------------------------------ |
| No traffic visible in ZAP | Firefox not using proxy           | Recheck Firefox proxy settings: `127.0.0.1:8080`       |
| Scan stops immediately    | Target not reachable              | Confirm app is running and URL uses `http://localhost` |
| Alerts all “Low”          | Small site or missing spider data | Browse the site manually before starting the scan      |
| Can’t start scan          | ZAP session not saved or UI froze | Restart ZAP and try again                              |

---

## Summary

You’ve:

1. Installed OWASP ZAP on macOS.
2. Configured Firefox to proxy through ZAP.
3. Launched and attacked your local applications at `localhost:3000`, `:8000`, and `:8001`.
4. Reviewed alerts focusing on data-transfer issues and SQL injection.
5. Exported your results for review.

---

**Legal Note:**
Only scan systems you own or have explicit permission to test.
