# Testing Frontend-to-Backend with OWASP ZAP

How to use ZAP as a proxy to intercept and test the traffic between your React frontend and the backends.

How It Works:
React Frontend (Port 3000)
↓
OWASP ZAP Proxy (Port 8080) ← Intercepts & analyzes traffic
↓
Backend APIs (Ports 8000/8001)
ZAP sits in the middle and captures all requests/responses.

Setup: Frontend-to-Backend Testing
Step 1: Configure Browser to Use ZAP Proxy
Option A: Firefox (Recommended)

Open Firefox
Preferences → Network Settings → Manual proxy
HTTP Proxy: localhost, Port: 8080
Check "Use this proxy server for all protocols"

Option B: Chrome
bash# Launch Chrome with proxy
/Applications/Google\ Chrome.app/Contents/MacOS/Google\ Chrome \
 --proxy-server="localhost:8080" \
 --user-data-dir="/tmp/chrome-zap"
Step 2: Start OWASP ZAP
bash# Launch ZAP in GUI mode
open -a "OWASP ZAP"

Choose Manual Explore
Leave proxy on default: localhost:8080
Click Start

Step 3: Install ZAP Certificate (First Time Only)
Required for HTTPS sites (not needed for your localhost HTTP)

In ZAP: Tools → Options → Dynamic SSL Certificates
Click Save to export certificate
Import to browser trusted certificates

Step 4: Use Your Frontend
bash# Start all services
cd backend/police-system && cargo run # Terminal 1
cd backend/hospital-system && cargo run # Terminal 2
cd frontend && npm run dev # Terminal 3

```

Open proxied browser to: `http://localhost:3000`

### Step 5: Interact with Frontend

**Everything you do in the frontend will be captured by ZAP:**

1. Load the page
2. View suspects/patients
3. Toggle flags
4. Any API calls made

**ZAP will show all requests in the "History" tab**

---

## What You'll See in ZAP:

### Captured Requests:
```

GET http://localhost:3000/ ← Frontend HTML
GET http://localhost:3000/src/main.tsx ← Frontend JS
GET http://localhost:8000/suspects ← API call (via Vite proxy)
GET http://localhost:8001/patients ← API call (via Vite proxy)
PUT http://localhost:8000/suspects/.../flag ← Flag toggle

```

### Testing Frontend Behavior:

1. **Spider the frontend** - ZAP discovers all pages/components
2. **See API calls** - Every fetch() from React
3. **Modify requests** - Test how frontend handles bad responses
4. **Inject payloads** - See if frontend sanitizes input

---

## Example: Testing Flag Toggle Feature

### 1. Open Frontend in Proxied Browser

Navigate to `http://localhost:3000`

### 2. Click "Flag" Button on a Suspect

ZAP captures this request:
```

PUT http://localhost:8000/suspects/19850312-2398/flag
Content-Type: application/json

{"flag": true} 3. In ZAP, Right-Click Request → Resend
Try malicious payloads:
json{"flag": "true"} ← String instead of boolean
{"flag": true, "id": 999} ← Extra fields
{"flag": null} ← Null value 4. See How Frontend/Backend Handle It

Does frontend crash?
Does backend reject invalid data?
Are errors handled gracefully?

Advanced: Automated Frontend TestingZAP Frontend-to-Backend Testing ScriptCode #!/bin/bash

echo "🌐 OWASP ZAP - Frontend to Backend Testing"
echo "==========================================="
echo ""

# Configuration

ZAP*PORT=8080
FRONTEND_URL="http://localhost:3000"
TIMESTAMP=$(date +"%Y-%m-%d*%H-%M-%S")
RESULTS_DIR="testing/
What Gets Tested in Frontend-to-Backend Mode:
✅ API Calls from React

Fetch requests to /api/police/_
Fetch requests to /api/hospital/_
Request/response headers
JSON payloads

✅ Vite Proxy Behavior

How Vite rewrites URLs
Proxy header handling
CORS through proxy

✅ Frontend Security

CSP (Content Security Policy)
Cookie handling
Local storage usage
XSS in rendered data

✅ Integration Issues

How frontend handles backend errors
Invalid response handling
Network failure behavior

Differences: Direct Backend vs Frontend-to-Backend
AspectDirect Backend TestingFrontend-to-Backend TestingSetupZAP → BackendBrowser → ZAP → BackendTestsAPI endpoints onlyReal user interactionsFindsBackend vulnerabilitiesIntegration issuesCoverageAll endpointsOnly used endpointsSpeedFast (automated)Slow (manual interaction)

Best Practice: Test Both Ways

1. Direct Backend Testing (Automated)
   bash./testing/zap-test.sh
   Finds: Backend API vulnerabilities
2. Frontend-to-Backend Testing (Manual)
   bash# Start ZAP proxy

# Configure browser

# Use frontend normally

Finds: Integration issues, real-world usage problems
