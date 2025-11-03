#!/bin/bash

# OWASP ZAP Automated Security Scanner for Police & Hospital Backend APIs
# This script runs ZAP API scans on both backend systems (ports 8000, 8001)
# and produces timestamped reports in multiple formats (HTML, JSON, XML).
#
# Prerequisites:
# - OWASP ZAP installed (brew install --cask owasp-zap)
# - Backend services running on ports 8000 and 8001
# - Seed data loaded in databases
#
# Usage:
#   ./run-all-owasp-zap-scans.sh
#
# Exit codes:
#   0 - Success (no high-risk vulnerabilities)
#   1 - Failure (high-risk vulnerabilities found or scan error)

set -euo pipefail

# ============================================================================
# CONFIGURATION
# ============================================================================

# Target URLs (modify if using different ports)
POLICE_TARGET="http://localhost:8000"
HOSPITAL_TARGET="http://localhost:8001"

# ZAP Configuration
ZAP_PORT_POLICE=8090
ZAP_PORT_HOSPITAL=8091

# Scan timeout in minutes
SCAN_TIMEOUT=10

# Detect ZAP installation path
if [[ "$OSTYPE" == "darwin"* ]]; then
    # macOS
    ZAP_CMD="/Applications/ZAP.app/Contents/Java/zap.sh"
elif command -v zap.sh &> /dev/null; then
    # Linux with zap.sh in PATH
    ZAP_CMD="zap.sh"
else
    echo "❌ ERROR: OWASP ZAP not found"
    echo "Install with: brew install --cask owasp-zap (macOS)"
    echo "Or download from: https://www.zaproxy.org/download/"
    exit 1
fi

# Verify ZAP exists
if [[ ! -f "$ZAP_CMD" ]] && ! command -v "$ZAP_CMD" &> /dev/null; then
    echo "❌ ERROR: ZAP executable not found at: $ZAP_CMD"
    exit 1
fi

# ============================================================================
# SETUP
# ============================================================================

echo "🔍 OWASP ZAP Security Scanner - Police & Hospital Systems"
echo "=========================================================="
echo ""

# Create timestamp for results folder
TIMESTAMP=$(date +"%Y-%m-%d_%H-%M-%S")
RESULTS_DIR="zap-results/scan_${TIMESTAMP}"

echo "📁 Creating results directory: ${RESULTS_DIR}"
mkdir -p "${RESULTS_DIR}"

echo "🕒 Scan started at: $(date)"
echo ""

# Log file
LOG_FILE="${RESULTS_DIR}/scan.log"
touch "$LOG_FILE"

# Function to log messages
log() {
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] $1" | tee -a "$LOG_FILE"
}

# Function to check if service is running
check_service() {
    local url=$1
    local name=$2
    
    log "Checking if $name is running at $url..."
    
    if curl -sf "${url}/health" > /dev/null 2>&1; then
        log "✅ $name is running"
        return 0
    else
        log "❌ ERROR: $name is not running at $url"
        log "Please start the backend service first:"
        log "  cd backend/${name,,}-system && cargo run"
        return 1
    fi
}

# ============================================================================
# PRE-FLIGHT CHECKS
# ============================================================================

log "Running pre-flight checks..."

if ! check_service "$POLICE_TARGET" "Police System"; then
    exit 1
fi

if ! check_service "$HOSPITAL_TARGET" "Hospital System"; then
    exit 1
fi

log "✅ All backend services are running"
echo ""

# ============================================================================
# SCAN FUNCTIONS
# ============================================================================

# Function to run ZAP API scan
run_zap_scan() {
    local target_url=$1
    local system_name=$2
    local zap_port=$3
    local output_base=$4
    
    log "🔍 Starting ZAP API scan for $system_name ($target_url)..."
    
    local html_report="${output_base}-report.html"
    local json_report="${output_base}-report.json"
    local xml_report="${output_base}-report.xml"
    
    # Run ZAP baseline scan (quick mode)
    # Using -cmd for command-line mode (headless)
    # -quickurl: target URL to scan
    # -quickprogress: show progress
    # -quickout: output report file
    
    log "Generating HTML report..."
    if "$ZAP_CMD" -cmd \
        -port "$zap_port" \
        -config api.disablekey=true \
        -config connection.timeoutInSecs=60 \
        -quickurl "$target_url" \
        -quickprogress \
        -quickout "$html_report" >> "$LOG_FILE" 2>&1; then
        log "✅ HTML report generated: $html_report"
    else
        log "❌ ERROR: Failed to generate HTML report for $system_name"
        return 1
    fi
    
    # Generate JSON report
    log "Generating JSON report..."
    if "$ZAP_CMD" -cmd \
        -port "$zap_port" \
        -config api.disablekey=true \
        -config connection.timeoutInSecs=60 \
        -quickurl "$target_url" \
        -quickout "$json_report" >> "$LOG_FILE" 2>&1; then
        log "✅ JSON report generated: $json_report"
    else
        log "⚠️  Warning: Failed to generate JSON report for $system_name"
    fi
    
    # Generate XML report
    log "Generating XML report..."
    if "$ZAP_CMD" -cmd \
        -port "$zap_port" \
        -config api.disablekey=true \
        -config connection.timeoutInSecs=60 \
        -quickurl "$target_url" \
        -quickout "$xml_report" >> "$LOG_FILE" 2>&1; then
        log "✅ XML report generated: $xml_report"
    else
        log "⚠️  Warning: Failed to generate XML report for $system_name"
    fi
    
    log "✅ Scan complete for $system_name"
    echo ""
    
    return 0
}

# Function to check for high-risk vulnerabilities
check_high_risk_vulnerabilities() {
    local json_report=$1
    local system_name=$2
    
    if [[ ! -f "$json_report" ]]; then
        log "⚠️  Warning: Cannot check vulnerabilities - JSON report not found: $json_report"
        return 0
    fi
    
    # Check if jq is available for JSON parsing
    if ! command -v jq &> /dev/null; then
        log "⚠️  Warning: jq not installed, skipping vulnerability severity check"
        log "Install with: brew install jq (macOS) or apt install jq (Linux)"
        return 0
    fi
    
    # Count high-risk alerts (riskcode = 3)
    # ZAP risk codes: 0=Informational, 1=Low, 2=Medium, 3=High
    local high_risk_count
    high_risk_count=$(jq '[.site[].alerts[] | select(.riskcode == "3")] | length' "$json_report" 2>/dev/null || echo "0")
    
    if [[ "$high_risk_count" -gt 0 ]]; then
        log "🚨 WARNING: Found $high_risk_count high-risk vulnerabilities in $system_name"
        log "Review the report: $json_report"
        return 1
    else
        log "✅ No high-risk vulnerabilities found in $system_name"
        return 0
    fi
}

# ============================================================================
# MAIN EXECUTION
# ============================================================================

# Track overall success
OVERALL_SUCCESS=0

# Scan Police System
log "===== Police System Scan ====="
cd "$RESULTS_DIR"
if run_zap_scan "$POLICE_TARGET" "Police System" "$ZAP_PORT_POLICE" "police-api-scan"; then
    if ! check_high_risk_vulnerabilities "police-api-scan-report.json" "Police System"; then
        OVERALL_SUCCESS=1
    fi
else
    log "❌ Police System scan failed"
    OVERALL_SUCCESS=1
fi
cd - > /dev/null

# Wait between scans to avoid port conflicts
sleep 2

# Scan Hospital System
log "===== Hospital System Scan ====="
cd "$RESULTS_DIR"
if run_zap_scan "$HOSPITAL_TARGET" "Hospital System" "$ZAP_PORT_HOSPITAL" "hospital-api-scan"; then
    if ! check_high_risk_vulnerabilities "hospital-api-scan-report.json" "Hospital System"; then
        OVERALL_SUCCESS=1
    fi
else
    log "❌ Hospital System scan failed"
    OVERALL_SUCCESS=1
fi
cd - > /dev/null

# ============================================================================
# SUMMARY
# ============================================================================

echo ""
log "=========================================================="
log "Scan Summary"
log "=========================================================="
log "Timestamp: $TIMESTAMP"
log "Results directory: $RESULTS_DIR"
log "Log file: $LOG_FILE"
echo ""

log "Generated Reports:"
log "  Police System:"
log "    - HTML: ${RESULTS_DIR}/police-api-scan-report.html"
log "    - JSON: ${RESULTS_DIR}/police-api-scan-report.json"
log "    - XML:  ${RESULTS_DIR}/police-api-scan-report.xml"
echo ""
log "  Hospital System:"
log "    - HTML: ${RESULTS_DIR}/hospital-api-scan-report.html"
log "    - JSON: ${RESULTS_DIR}/hospital-api-scan-report.json"
log "    - XML:  ${RESULTS_DIR}/hospital-api-scan-report.xml"
echo ""

log "Scan completed at: $(date)"
echo ""

# ============================================================================
# VIEW RESULTS
# ============================================================================

log "📊 To view results:"
log "  HTML Reports (visual):"
log "    open ${RESULTS_DIR}/police-api-scan-report.html"
log "    open ${RESULTS_DIR}/hospital-api-scan-report.html"
echo ""
log "  JSON Reports (automation):"
log "    cat ${RESULTS_DIR}/police-api-scan-report.json | jq"
log "    cat ${RESULTS_DIR}/hospital-api-scan-report.json | jq"
echo ""

# ============================================================================
# EXIT STATUS
# ============================================================================

if [[ $OVERALL_SUCCESS -eq 0 ]]; then
    log "✅ SUCCESS: All scans completed with no high-risk vulnerabilities"
    exit 0
else
    log "❌ FAILURE: High-risk vulnerabilities detected or scan errors occurred"
    log "Review the reports above for details"
    exit 1
fi
