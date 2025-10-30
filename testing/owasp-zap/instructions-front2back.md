# Testing Frontend-to-Backend with OWASP ZAP

**Yes!** You can use ZAP as a **proxy** to intercept and test the traffic between your React frontend and the backends.

## How It Works:

```
React Frontend (Port 3000)
    ↓
OWASP ZAP Proxy (Port 8080) ← Intercepts & analyzes traffic
    ↓
Backend APIs (Ports 8000/8001)
```

ZAP sits in the middle and captures all requests/responses.

---

## Setup: Frontend-to-Backend Testing

### Step 1: Configure Browser to Use ZAP Proxy

**Option A: Firefox (Recommended)**
1. Open Firefox
2. **Preferences → Network Settings → Manual proxy**
3. HTTP Proxy: `localhost`, Port: `8080`
4. Check "Use this proxy server for all protocols"

**Option B: Chrome**
```bash
# Launch Chrome with proxy
/Applications/Google\ Chrome.app/Contents/MacOS/Google\ Chrome \
  --proxy-server="localhost:8080" \
  --user-data-dir="/tmp/chrome-zap"
```

### Step 2: Start OWASP ZAP

```bash
# Launch ZAP in GUI mode
open -a "OWASP ZAP"
```

1. Choose **Manual Explore**
2. Leave proxy on default: `localhost:8080`
3. Click **Start**

### Step 3: Install ZAP Certificate (First Time Only)

**Required for HTTPS sites (not needed for your localhost HTTP)**

1. In ZAP: **Tools → Options → Dynamic SSL Certificates**
2. Click **Save** to export certificate
3. Import to browser trusted certificates

### Step 4: Use Your Frontend

```bash
# Start all services
cd backend/police-system && cargo run    # Terminal 1
cd backend/hospital-system && cargo run  # Terminal 2
cd frontend && npm run dev               # Terminal 3
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
GET http://localhost:3000/               ← Frontend HTML
GET http://localhost:3000/src/main.tsx   ← Frontend JS
GET http://localhost:8000/suspects       ← API call (via Vite proxy)
GET http://localhost:8001/patients       ← API call (via Vite proxy)
PUT http://localhost:8000/suspects/.../flag  ← Flag toggle
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

{"flag": true}
```

### 3. In ZAP, Right-Click Request → Resend

Try malicious payloads:
```json
{"flag": "true"}           ← String instead of boolean
{"flag": true, "id": 999}  ← Extra fields
{"flag": null}             ← Null value
```

### 4. See How Frontend/Backend Handle It

- Does frontend crash?
- Does backend reject invalid data?
- Are errors handled gracefully?

---

## Advanced: Automated Frontend Testing---

## What Gets Tested in Frontend-to-Backend Mode:

### ✅ API Calls from React
- Fetch requests to `/api/police/*`
- Fetch requests to `/api/hospital/*`
- Request/response headers
- JSON payloads

### ✅ Vite Proxy Behavior
- How Vite rewrites URLs
- Proxy header handling
- CORS through proxy

### ✅ Frontend Security
- CSP (Content Security Policy)
- Cookie handling
- Local storage usage
- XSS in rendered data

### ✅ Integration Issues
- How frontend handles backend errors
- Invalid response handling
- Network failure behavior

---

## Differences: Direct Backend vs Frontend-to-Backend

| Aspect | Direct Backend Testing | Frontend-to-Backend Testing |
|--------|----------------------|---------------------------|
| **Setup** | ZAP → Backend | Browser → ZAP → Backend |
| **Tests** | API endpoints only | Real user interactions |
| **Finds** | Backend vulnerabilities | Integration issues |
| **Coverage** | All endpoints | Only used endpoints |
| **Speed** | Fast (automated) | Slow (manual interaction) |

---

## Best Practice: Test Both Ways

### 1. Direct Backend Testing (Automated)
```bash
./testing/zap-test.sh
```
**Finds:** Backend API vulnerabilities

### 2. Frontend-to-Backend Testing (Manual)
```bash
# Start ZAP proxy
# Configure browser
# Use frontend normally
```
**Finds:** Integration issues, real-world usage problems

---

## Quick Answer:

**Yes, use ZAP as a proxy!** This lets you:
- See all frontend→backend traffic
- Test real user interactions
- Find integration bugs
- Verify how frontend handles errors

**But also test backends directly** for complete API coverage.
