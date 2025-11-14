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

# Nmap Security Testing - Quick Guide (macOS)

## ⚠️ EDUCATIONAL RESEARCH USE ONLY

**This guide describes penetration testing tools used for academic research on AI-generated code.**

### Legal Warning

**STOP**: Before using these tools, understand that:

- ❌ **Unauthorized network scanning is illegal** in most jurisdictions
- ❌ Running these scans on systems you don't own can result in criminal charges
- ✅ These tools were used exclusively on **localhost** for thesis research
- ✅ All testing was performed on **fictitious data** in a controlled environment

### Intended Use

This documentation is for:

- Academic review of our research methodology
- Understanding how AI-generated code was tested
- Educational purposes in authorized environments

**If you run these scans, you are responsible for ensuring you have proper authorization.**

---

Quick guide for running nmap security tests on the Police & Hospital backend systems.

## Prerequisites

### Install Nmap

```bash
# Install via Homebrew
brew install nmap

# Verify installation
nmap --version
```

### Start Backend Services

**Before testing**, start both backend services:

```bash
# Terminal 1: Police System
cd backend/police-system
cargo run

# Terminal 2: Hospital System
cd backend/hospital-system
cargo run

# Verify both are running
curl http://localhost:8000/health
curl http://localhost:8001/health
```

---

## Option 1: Automated Script (Recommended)

The project includes a pre-configured testing script that runs all 12 security tests automatically.

### Using the Script

```bash
# Navigate to testing directory
cd testing/nmap/

# Make executable (first time only)
chmod +x run-all-scans.sh

# Run all scans
./run-all-scans.sh
```

**Time:** ~12-15 minutes

The script (`run-all-scans.sh`) will:

- Create timestamped results directory
- Run all 12 security scans sequentially
- Display progress for each test
- Save results to `nmap-results/scan_YYYY-MM-DD_HH-MM-SS/`

### View Results

```bash
# List all scans
ls nmap-results/

# View specific result
cat nmap-results/scan_2025-10-30_14-35-22/07-vulnerability-scan.txt

# Open all results in VS Code
code nmap-results/
```

---

## Option 2: Manual Commands

Run these commands individually if you prefer:

```bash
# Create results folder
mkdir -p nmap-results
cd nmap-results

# Basic tests
nmap -p 8000,8001 localhost
nmap -p 8000,8001 -sV localhost
nmap -p 5432,8000,8001 localhost

# Security tests
nmap -p 8000,8001 --script http-methods localhost
nmap -p 8000,8001 --script http-headers localhost
nmap -p 8000,8001 --script http-cors localhost
nmap -p 8000,8001 --script vuln localhost

# Comprehensive scan
nmap -A -p 8000,8001 localhost
```

---

## Understanding Results

### Good Signs ✅

```
PORT     STATE SERVICE
8000/tcp open  http-alt
8001/tcp open  vcom-tunnel
```

- Both services running
- No vulnerabilities listed
- Proper HTTP error codes (400, 404, 408)

### Expected Findings ⚠️ (Development Mode)

- HTTP only (no HTTPS)
- CORS allows all origins (`Access-Control-Allow-Origin: *`)
- No authentication required
- Server header exposes Actix-web

**These are acceptable for development but need fixing for production.**

---

## Troubleshooting

### "Permission denied"

```bash
chmod +x run-all-scans.sh
```

### "No such file or directory"

```bash
# Check current location
pwd

# Navigate to correct folder
cd testing
```

### "Failed to resolve host"

```bash
# Backends not running - start them first
cd backend/police-system && cargo run  # Terminal 1
cd backend/hospital-system && cargo run  # Terminal 2
```

### Script seems stuck

- Tests 7 and 11 take the longest (~5 minutes each)
- Total time: ~12-15 minutes
- Check progress: `ls -lh nmap-results/scan_*/` in another terminal

---

## Quick Reference

```bash
# Install
brew install nmap

# Start services
cd backend/police-system && cargo run
cd backend/hospital-system && cargo run

# Run script
cd testing
./run-all-scans.sh

# View results
ls nmap-results/
```

---

## What Gets Tested

| Test | What It Checks                    | Time   |
| ---- | --------------------------------- | ------ |
| 1-3  | Port status, services, PostgreSQL | 1 min  |
| 4-6  | HTTP methods, headers, CORS       | 2 min  |
| 7    | Known vulnerabilities (CVE)       | 5 min  |
| 8-10 | SQL injection, XSS, DoS           | 2 min  |
| 11   | Comprehensive security scan       | 5 min  |
| 12   | Essential HTTP scripts            | 30 sec |

**Total: ~12-15 minutes**

---

_Last Updated: January 2025_
