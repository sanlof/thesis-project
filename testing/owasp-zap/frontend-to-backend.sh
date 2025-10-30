#!/bin/bash

echo "🌐 OWASP ZAP - Frontend to Backend Testing"
echo "==========================================="
echo ""

# Configuration
ZAP_PORT=8080
FRONTEND_URL="http://localhost:3000"
TIMESTAMP=$(date +"%Y-%m-%d_%H-%M-%S")
RESULTS_DIR="testing/zap-results/frontend-scan_${TIMESTAMP}"

mkdir -p "${RESULTS_DIR}"

# Check if all services are running
echo "🔍 Checking services..."

if ! curl -s http://localhost:8000/health > /dev/null; then
    echo "❌ Police backend not running on port 8000"
    exit 1
fi

if ! curl -s http://localhost:8001/health > /dev/null; then
    echo "❌ Hospital backend not running on port 8001"
    exit 1
fi

if ! curl -s http://localhost:3000 > /dev/null; then
    echo "❌ Frontend not running on port 3000"
    echo "Start with: cd frontend && npm run dev"
    exit 1
fi

echo "✅ All services running"
echo ""

# Start ZAP
echo "🚀 Starting OWASP ZAP as proxy..."
/Applications/OWASP\ ZAP.app/Contents/Java/zap.sh \
    -daemon -port ${ZAP_PORT} \
    -config api.disablekey=true &
ZAP_PID=$!

echo "⏳ Waiting for ZAP to initialize..."
sleep 30

# Configure ZAP to intercept Vite dev server traffic
echo "⚙️  Configuring ZAP for frontend testing..."

# Add frontend URL to scope
curl -s "http://localhost:${ZAP_PORT}/JSON/core/action/includeInContext/?contextName=Default&regex=${FRONTEND_URL}.*" > /dev/null

# Enable HUD (Heads Up Display) for browser
curl -s "http://localhost:${ZAP_PORT}/JSON/hud/action/enable/" > /dev/null

echo ""
echo "📋 Manual Steps Required:"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "1. Configure Firefox/Chrome proxy:"
echo "   HTTP Proxy: localhost"
echo "   Port: ${ZAP_PORT}"
echo ""
echo "2. Open browser and navigate to:"
echo "   ${FRONTEND_URL}"
echo ""
echo "3. Interact with the frontend:"
echo "   - View suspects/patients"
echo "   - Toggle flags"
echo "   - Navigate between pages"
echo ""
echo "4. Watch ZAP capture all requests in the History tab"
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "Press ENTER when you're done testing..."
read

# Run passive scan on collected data
echo ""
echo "🔍 Running passive scan on captured traffic..."
curl -s "http://localhost:${ZAP_PORT}/JSON/pscan/action/enableAllScanners/" > /dev/null

# Wait for passive scan
sleep 10

# Generate reports
echo "📊 Generating reports..."
curl -s "http://localhost:${ZAP_PORT}/OTHER/core/other/htmlreport/" \
    -o "${RESULTS_DIR}/frontend-backend-report.html"

curl -s "http://localhost:${ZAP_PORT}/JSON/core/view/alerts/" | \
    jq '.' > "${RESULTS_DIR}/frontend-alerts.json"

# Get request history
curl -s "http://localhost:${ZAP_PORT}/JSON/core/view/messages/" | \
    jq '.' > "${RESULTS_DIR}/request-history.json"

# Shutdown ZAP
echo "🛑 Shutting down ZAP..."
curl -s "http://localhost:${ZAP_PORT}/JSON/core/action/shutdown/" > /dev/null
wait $ZAP_PID 2>/dev/null

# Summary
echo ""
echo "✅ Frontend testing complete!"
echo ""
echo "📄 Reports saved in: ${RESULTS_DIR}/"
echo "   - frontend-backend-report.html (Open in browser)"
echo "   - frontend-alerts.json (Security findings)"
echo "   - request-history.json (All captured requests)"
echo ""
echo "🌐 View report:"
echo "   open ${RESULTS_DIR}/frontend-backend-report.html"
echo ""