#!/bin/bash

echo "🔍 Complete Nmap Security Scan - Police & Hospital Systems"
echo "============================================================"
echo ""

# Create timestamp folder name
# Format: YYYY-MM-DD_HH-MM-SS (e.g., 2025-10-30_14-35-22)
TIMESTAMP=$(date +"%Y-%m-%d_%H-%M-%S")
RESULTS_DIR="nmap-results/scan_${TIMESTAMP}"

echo "Creating results folder: ${RESULTS_DIR}"
echo "Starting scan at: $(date)"
echo ""

# Create timestamped results directory
mkdir -p "${RESULTS_DIR}"
cd "${RESULTS_DIR}"

echo "📌 Test 1/12: Basic Port Check..."
nmap -p 8000,8001 localhost > 01-port-check.txt
echo "✅ Complete"
echo ""

echo "📌 Test 2/12: Service Version Detection..."
nmap -p 8000,8001 -sV localhost > 02-version-detection.txt
echo "✅ Complete"
echo ""

echo "📌 Test 3/12: All Services (Including PostgreSQL)..."
nmap -p 5432,8000,8001 localhost > 03-all-services.txt
echo "✅ Complete"
echo ""

echo "📌 Test 4/12: HTTP Methods..."
nmap -p 8000,8001 --script http-methods localhost > 04-http-methods.txt
echo "✅ Complete"
echo ""

echo "📌 Test 5/12: HTTP Headers..."
nmap -p 8000,8001 --script http-headers localhost > 05-http-headers.txt
echo "✅ Complete"
echo ""

echo "📌 Test 6/12: CORS Configuration..."
nmap -p 8000,8001 --script http-cors localhost > 06-cors-check.txt
echo "✅ Complete"
echo ""

echo "📌 Test 7/12: Vulnerability Scan..."
nmap -p 8000,8001 --script vuln localhost > 07-vulnerability-scan.txt
echo "✅ Complete"
echo ""

echo "📌 Test 8/12: SQL Injection Test..."
nmap -p 8000,8001 --script http-sql-injection localhost > 08-sql-injection.txt
echo "✅ Complete"
echo ""

echo "📌 Test 9/12: XSS Test..."
nmap -p 8000,8001 --script http-xss* localhost > 09-xss-test.txt
echo "✅ Complete"
echo ""

echo "📌 Test 10/12: Slowloris Attack Test..."
nmap -p 8000,8001 --script http-slowloris-check localhost > 10-slowloris-test.txt
echo "✅ Complete"
echo ""

echo "📌 Test 11/12: Comprehensive Security Scan..."
nmap -A -p 8000,8001 localhost > 11-comprehensive-scan.txt
echo "✅ Complete"
echo ""

echo "📌 Test 12/12: Essential HTTP Scripts..."
nmap -p 8000,8001 --script http-methods,http-headers,http-cors,http-title,http-server-header localhost > 12-essential-http-scripts.txt
echo "✅ Complete"
echo ""

echo ""
echo "✅ All scans complete!"
echo ""
echo "Results saved in: ${RESULTS_DIR}/"
echo "Finished at: $(date)"
echo ""
echo "📄 Generated files:"
ls -lh *.txt