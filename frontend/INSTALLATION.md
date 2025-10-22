# Frontend Installation Guide

Complete guide for setting up the React + TypeScript + Vite frontend for the Police-Hospital Data Sharing System.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Quick Setup](#quick-setup)
- [Manual Setup](#manual-setup)
- [Configuration](#configuration)
- [Verification](#verification)
- [Troubleshooting](#troubleshooting)

---

## Prerequisites

Before installing the frontend, ensure you have:

### Required Software

- **Node.js** >= 18.0.0 ([Download](https://nodejs.org/))
- **npm** >= 9.0.0 (comes with Node.js)

### Backend Services

The frontend requires both backend services to be running:

- **Police System** on `http://localhost:8000`
- **Hospital System** on `http://localhost:8001`

**Verify backends are running:**

```bash
curl http://localhost:8000/health
curl http://localhost:8001/health
```

Expected response: `{"status":"healthy","service":"police-system"}` and similar for hospital.

---

## Quick Setup

Use the provided setup script for automatic installation:

```bash
# Navigate to frontend directory
cd frontend

# Run setup script
chmod +x setup.sh
./setup.sh
```

The script will:
1. Install all npm dependencies
2. Create `.env` file from template
3. Create necessary directories
4. Run type checking
5. Display next steps

**After setup completes:**

```bash
npm run dev
```

Open `http://localhost:5173` in your browser.

---

## Manual Setup

If you prefer to set up manually or the script doesn't work:

### Step 1: Create Frontend Directory

```bash
# From project root
mkdir -p frontend
cd frontend
```

### Step 2: Copy Configuration Files

Copy all files from the `frontend-setup` package to your `frontend` directory:

```bash
# Assuming you're in the frontend directory
cp ../frontend-setup/* .
```

Files to copy:
- `package.json`
- `vite.config.ts`
- `tsconfig.json`
- `tsconfig.node.json`
- `tailwind.config.js`
- `postcss.config.js`
- `.eslintrc.cjs`
- `index.html`
- `.env.example`
- `.gitignore`
- `README.md`

### Step 3: Install Dependencies

```bash
npm install
```

This will install:
- React 18.2 and React DOM
- TypeScript 5.3
- Vite 5.0 with React plugin
- Axios 1.6 for HTTP requests
- Tailwind CSS 3.3 for styling
- ESLint and related plugins
- Type definitions for React

**Installation time:** ~2-5 minutes depending on internet speed

### Step 4: Configure Environment

```bash
# Copy environment template
cp .env.example .env

# Edit if your backend uses different ports
nano .env  # or use your preferred editor
```

Default configuration:
```env
VITE_POLICE_API_URL=http://localhost:8000
VITE_HOSPITAL_API_URL=http://localhost:8001
VITE_POLL_INTERVAL=5000
VITE_DEBUG_MODE=false
```

### Step 5: Create Directory Structure

```bash
mkdir -p src/components/police
mkdir -p src/components/hospital
mkdir -p src/components/shared
mkdir -p src/components/layout
mkdir -p src/services
mkdir -p src/hooks
mkdir -p src/context
mkdir -p src/utils
mkdir -p public
```

### Step 6: Verify Setup

```bash
# Check TypeScript configuration
npm run type-check

# Run linter
npm run lint
```

---

## Configuration

### Environment Variables

The frontend uses Vite's environment variable system. All variables must be prefixed with `VITE_`.

**Available variables:**

| Variable | Default | Description |
|----------|---------|-------------|
| `VITE_POLICE_API_URL` | `http://localhost:8000` | Police backend URL |
| `VITE_HOSPITAL_API_URL` | `http://localhost:8001` | Hospital backend URL |
| `VITE_POLL_INTERVAL` | `5000` | Data refresh interval (ms) |
| `VITE_DEBUG_MODE` | `false` | Enable debug logging |
| `VITE_API_TIMEOUT` | `10000` | API request timeout (ms) |

**To change settings:**

1. Edit `.env` file (never commit this file)
2. Restart dev server for changes to take effect

### Vite Configuration

The `vite.config.ts` includes:

- **Port 5173** - Development server port
- **Proxy Configuration** - Automatically forwards `/api/police/*` and `/api/hospital/*` requests
- **React Fast Refresh** - Hot module replacement for instant updates
- **Build Optimization** - Code splitting and tree shaking

### TypeScript Configuration

The `tsconfig.json` uses strict mode with:

- Strict null checks
- No implicit any
- Unused variable detection
- Modern ES2020 target

### Tailwind Configuration

Custom color palette included:

```javascript
// Police system colors (blue tones)
police-500  // Primary: #3b82f6
police-700  // Dark: #1d4ed8

// Hospital system colors (red tones)
hospital-500  // Primary: #ef4444
hospital-700  // Dark: #b91c1c
```

---

## Verification

### Test Development Server

```bash
npm run dev
```

**Expected output:**

```
  VITE v5.0.8  ready in 432 ms

  ➜  Local:   http://localhost:5173/
  ➜  Network: use --host to expose
  ➜  press h + enter to show help
```

### Test Build

```bash
npm run build
```

**Expected output:**

```
vite v5.0.8 building for production...
✓ 34 modules transformed.
dist/index.html                   0.45 kB │ gzip:  0.30 kB
dist/assets/index-DiwrgTda.css    3.25 kB │ gzip:  1.23 kB
dist/assets/index-BNF8xjKq.js   143.42 kB │ gzip: 46.11 kB
✓ built in 1.23s
```

### Test Backend Connectivity

Once the dev server is running, test API connectivity:

```bash
# Test police API proxy
curl http://localhost:5173/api/police/health

# Test hospital API proxy
curl http://localhost:5173/api/hospital/health
```

Both should return healthy status.

---

## Troubleshooting

### Issue: Port 5173 Already in Use

**Error:** `Port 5173 is in use, trying another one...`

**Solution 1:** Kill existing process

```bash
lsof -ti:5173 | xargs kill -9
```

**Solution 2:** Use different port

Edit `vite.config.ts`:
```typescript
server: {
  port: 5174,  // Change to any available port
}
```

### Issue: Module Not Found

**Error:** `Cannot find module 'react'`

**Solution:** Reinstall dependencies

```bash
rm -rf node_modules package-lock.json
npm install
```

### Issue: TypeScript Errors

**Error:** Multiple TypeScript errors after installation

**Solution:** This is expected if components aren't implemented yet

```bash
# Type checking will pass once you implement components
npm run type-check
```

To suppress these temporarily, you can skip lib check in `tsconfig.json`:
```json
{
  "compilerOptions": {
    "skipLibCheck": true
  }
}
```

### Issue: Tailwind Classes Not Working

**Error:** CSS classes not applying styles

**Solution 1:** Ensure Tailwind directives are in CSS

Create `src/index.css`:
```css
@tailwind base;
@tailwind components;
@tailwind utilities;
```

Import in `src/main.tsx`:
```typescript
import './index.css'
```

**Solution 2:** Restart dev server

```bash
# Stop server (Ctrl+C)
npm run dev
```

### Issue: Backend Connection Refused

**Error:** Network Error / Connection Refused in browser console

**Solution:** Verify backends are running

```bash
# Check police backend
curl http://localhost:8000/health

# Check hospital backend
curl http://localhost:8001/health
```

If not running, start them:
```bash
# Terminal 1
cd backend/police-system
cargo run

# Terminal 2
cd backend/hospital-system
cargo run
```

### Issue: CORS Errors

**Error:** `Access to fetch at 'http://localhost:8000' from origin 'http://localhost:5173' has been blocked by CORS policy`

**Solution:** This shouldn't happen with the proxy, but if it does:

1. Verify proxy configuration in `vite.config.ts`
2. Use proxy URLs in your code: `/api/police/` not `http://localhost:8000/`
3. Restart dev server after config changes

### Issue: Hot Reload Not Working

**Error:** Changes not reflecting immediately

**Solution:**

```bash
# Stop server (Ctrl+C)
# Clear Vite cache
rm -rf node_modules/.vite
# Restart
npm run dev
```

### Issue: Build Fails

**Error:** Build process fails with errors

**Solution:**

```bash
# Check for type errors first
npm run type-check

# Fix all TypeScript errors before building

# Try clean build
rm -rf dist
npm run build
```

---

## Next Steps

After successful installation:

1. **Implement Components**
   - Start with simple components in `src/components/`
   - Follow the structure in `ARCHITECTURE.md`

2. **Create API Services**
   - Implement in `src/services/policeApi.ts` and `hospitalApi.ts`
   - Use the provided type definitions

3. **Add Custom Hooks**
   - Create hooks in `src/hooks/`
   - Follow React hooks best practices

4. **Style with Tailwind**
   - Use custom police/hospital colors
   - Reference `tailwind.config.js` for available utilities

5. **Test Integration**
   - Test with both backends running
   - Verify flag synchronization works
   - Test cross-system queries

---

## Additional Resources

- [React Documentation](https://react.dev/)
- [TypeScript Handbook](https://www.typescriptlang.org/docs/)
- [Vite Guide](https://vitejs.dev/guide/)
- [Tailwind CSS Docs](https://tailwindcss.com/docs)
- [Axios Documentation](https://axios-http.com/docs/intro)

---

## Getting Help

If you encounter issues not covered here:

1. Check the browser console for errors
2. Check the terminal for build/runtime errors
3. Verify backend services are healthy
4. Review `ARCHITECTURE.md` for design details
5. Check Vite's error overlay in the browser

---

_Last Updated: 2025_
