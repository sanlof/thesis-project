#!/bin/bash

echo "🔒 OWASP ZAP Security Testing - Police & Hospital Systems"
echo "=========================================================="
echo ""

# Configuration
ZAP_PATH="/Applications/OWASP ZAP.app/Contents/Java"
ZAP_PORT=8080
POLICE_URL="http://localhost:8000"
HOSPITAL_URL="http://localhost:8001"
TIMESTAMP=$(date +"%Y-%m-%d_%H-%M-%S")
RESULTS_DIR="testing/zap-results/scan_${TIMESTAMP}"

# Create results directory
mkdir -p "${RESULTS_DIR}"
echo "📁 Results will be saved to: ${RESULTS_DIR}"
echo ""

# Check if ZAP is installed
if [ ! -d "$ZAP_PATH" ]; then
    echo "❌ OWASP ZAP not found at: ${ZAP_PATH}"
    echo "Install with: brew install --cask owasp-zap"
    exit 1
fi

# Check if backends are running
echo "🔍 Checking if backend services are running..."
if ! curl -s "${POLICE_URL}/health" > /dev/null; then
    echo "❌ Police system not responding at ${POLICE_URL}"
    echo "Start with: cd backend/police-system && cargo run"
    exit 1
fi

if ! curl -s "${HOSPITAL_URL}/health" > /dev/null; then
    echo "❌ Hospital system not responding at ${HOSPITAL_URL}"
    echo "Start with: cd backend/hospital-system && cargo run"
    exit 1
fi

echo "✅ Both backends are running"
echo ""

# Start ZAP daemon
echo "🚀 Starting OWASP ZAP in daemon mode..."
"${ZAP_PATH}/zap.sh" -daemon -port ${ZAP_PORT} -config api.disablekey=true &
ZAP_PID=$!

# Wait for ZAP to start
echo "⏳ Waiting for ZAP to initialize (30 seconds)..."
sleep 30

# Function to run spider
run_spider() {
    local url=$1
    local name=$2
    echo "🕷️  Running Spider scan on ${name}..."
    curl -s "http://localhost:${ZAP_PORT}/JSON/spider/action/scan/?url=${url}" > /dev/null
    sleep 5
    
    # Wait for spider to complete
    while [ "$(curl -s "http://localhost:${ZAP_PORT}/JSON/spider/view/status/" | grep -o '"status":"[0-9]*"' | grep -o '[0-9]*')" != "100" ]; do
        echo "   Spider progress: $(curl -s "http://localhost:${ZAP_PORT}/JSON/spider/view/status/" | grep -o '[0-9]*')%"
        sleep 5
    done
    echo "✅ Spider complete for ${name}"
}

# Function to run active scan
run_active_scan() {
    local url=$1
    local name=$2
    echo "🎯 Running Active scan on ${name}..."
    curl -s "http://localhost:${ZAP_PORT}/JSON/ascan/action/scan/?url=${url}" > /dev/null
    sleep 5
    
    # Wait for active scan to complete
    while [ "$(curl -s "http://localhost:${ZAP_PORT}/JSON/ascan/view/status/" | grep -o '"status":"[0-9]*"' | grep -o '[0-9]*')" != "100" ]; do
        echo "   Active scan progress: $(curl -s "http://localhost:${ZAP_PORT}/JSON/ascan/view/status/" | grep -o '[0-9]*')%"
        sleep 10
    done
    echo "✅ Active scan complete for ${name}"
}

# Function to generate report
generate_report() {
    local name=$1
    local filename=$2
    echo "📊 Generating report for ${name}..."
    curl -s "http://localhost:${ZAP_PORT}/OTHER/core/other/htmlreport/" -o "${RESULTS_DIR}/${filename}.html"
    curl -s "http://localhost:${ZAP_PORT}/OTHER/core/other/xmlreport/" -o "${RESULTS_DIR}/${filename}.xml"
    curl -s "http://localhost:${ZAP_PORT}/OTHER/core/other/jsonreport/" -o "${RESULTS_DIR}/${filename}.json"
    echo "✅ Reports generated"
}

# Test Police System
echo ""
echo "🚔 Testing Police System (${POLICE_URL})"
echo "----------------------------------------"
run_spider "${POLICE_URL}" "Police System"
run_active_scan "${POLICE_URL}" "Police System"
generate_report "Police System" "police-report"

# Test Hospital System
echo ""
echo "🏥 Testing Hospital System (${HOSPITAL_URL})"
echo "-------------------------------------------"
run_spider "${HOSPITAL_URL}" "Hospital System"
run_active_scan "${HOSPITAL_URL}" "Hospital System"
generate_report "Hospital System" "hospital-report"

# Generate combined summary
echo ""
echo "📋 Generating combined summary..."
curl -s "http://localhost:${ZAP_PORT}/JSON/core/view/alerts/" | \
    jq '.' > "${RESULTS_DIR}/all-alerts.json"

# Count vulnerabilities
HIGH=$(curl -s "http://localhost:${ZAP_PORT}/JSON/core/view/alertsSummary/" | \
    jq '[.alertsSummary[] | select(.risk=="High")] | length')
MEDIUM=$(curl -s "http://localhost:${ZAP_PORT}/JSON/core/view/alertsSummary/" | \
    jq '[.alertsSummary[] | select(.risk=="Medium")] | length')
LOW=$(curl -s "http://localhost:${ZAP_PORT}/JSON/core/view/alertsSummary/" | \
    jq '[.alertsSummary[] | select(.risk=="Low")] | length')
INFO=$(curl -s "http://localhost:${ZAP_PORT}/JSON/core/view/alertsSummary/" | \
    jq '[.alertsSummary[] | select(.risk=="Informational")] | length')

# Shutdown ZAP
echo ""
echo "🛑 Shutting down OWASP ZAP..."
curl -s "http://localhost:${ZAP_PORT}/JSON/core/action/shutdown/" > /dev/null
wait $ZAP_PID 2>/dev/null

# Summary
echo ""
echo "✅ All scans complete!"
echo ""
echo "📊 Vulnerability Summary:"
echo "   🔴 High:          ${HIGH}"
echo "   🟠 Medium:        ${MEDIUM}"
echo "   🟡 Low:           ${LOW}"
echo "   🔵 Informational: ${INFO}"
echo ""
echo "📄 Reports saved in: ${RESULTS_DIR}/"
echo "   - police-report.html"
echo "   - hospital-report.html"
echo "   - all-alerts.json"
echo ""
echo "🌐 Open reports with:"
echo "   open ${RESULTS_DIR}/police-report.html"
echo "   open ${RESULTS_DIR}/hospital-report.html"
echo ""
echo "Finished at: $(date)"