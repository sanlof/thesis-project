# Frontend Setup Package - Summary

Complete configuration files for React + TypeScript + Vite frontend setup.

## 📦 Package Contents

This package contains all necessary configuration files to set up your frontend:

### Core Configuration Files

1. **package.json**
   - All dependencies (React, TypeScript, Vite, Axios, Tailwind)
   - npm scripts for development, build, and testing
   - Engine requirements (Node >= 18, npm >= 9)

2. **vite.config.ts**
   - Vite configuration with React plugin
   - Development server on port 5173
   - Proxy configuration for backend APIs:
     * `/api/police/*` → `http://localhost:8000`
     * `/api/hospital/*` → `http://localhost:8001`
   - Build optimization settings

3. **tsconfig.json**
   - Strict TypeScript configuration
   - Modern ES2020 target
   - Path mapping for `@/*` imports
   - All strict mode checks enabled

4. **tsconfig.node.json**
   - TypeScript config for Vite configuration files
   - Separate from main app config

### Styling Configuration

5. **tailwind.config.js**
   - Custom color palette:
     * Police system (blue tones: `police-50` to `police-900`)
     * Hospital system (red tones: `hospital-50` to `hospital-900`)
     * Status colors (success, error, warning, info)
     * Flag status colors
   - Custom animations and shadows
   - Extended spacing and border radius

6. **postcss.config.js**
   - PostCSS configuration for Tailwind
   - Autoprefixer for browser compatibility

7. **index.css** (to be created)
   - Global styles with Tailwind directives
   - Custom CSS if needed

### Quality Assurance

8. **.eslintrc.cjs**
   - ESLint configuration for React + TypeScript
   - React Hooks rules
   - TypeScript-specific rules
   - Code quality checks

### Environment & Documentation

9. **.env.example**
   - Environment variable template
   - Backend API URLs
   - Polling interval
   - Debug mode
   - Feature flags

10. **README.md**
    - Quick start guide
    - Available scripts
    - Project structure
    - Technology stack
    - Troubleshooting tips

11. **INSTALLATION.md**
    - Comprehensive setup guide
    - Prerequisites
    - Step-by-step installation
    - Configuration details
    - Verification steps
    - Troubleshooting solutions

### Utilities

12. **setup.sh**
    - Automated setup script
    - Installs dependencies
    - Creates directories
    - Sets up environment
    - Runs initial checks

13. **index.html**
    - HTML template
    - Meta tags for SEO
    - Vite entry point

14. **.gitignore**
    - Ignores node_modules, dist, and other generated files
    - Protects environment variables

15. **.vscode-extensions.json**
    - Recommended VS Code extensions:
      * ESLint
      * Prettier
      * Tailwind CSS IntelliSense
      * TypeScript

---

## 🚀 Quick Start

### Option 1: Automated Setup (Recommended)

```bash
# 1. Create frontend directory
mkdir -p frontend
cd frontend

# 2. Copy all files from this package to frontend/

# 3. Run setup script
chmod +x setup.sh
./setup.sh

# 4. Start development
npm run dev
```

### Option 2: Manual Setup

```bash
# 1. Create frontend directory and copy files
mkdir -p frontend
cd frontend
# Copy all files from package here

# 2. Install dependencies
npm install

# 3. Setup environment
cp .env.example .env

# 4. Create directories
mkdir -p src/{components/{police,hospital,shared,layout},services,hooks,context,utils}
mkdir -p public

# 5. Start development
npm run dev
```

---

## 📁 Directory Structure After Setup

```
frontend/
├── node_modules/          # Dependencies (created by npm install)
├── public/                # Static assets
├── src/                   # Source code
│   ├── components/
│   │   ├── police/       # Police system components
│   │   ├── hospital/     # Hospital system components
│   │   ├── shared/       # Shared components
│   │   └── layout/       # Layout components
│   ├── services/         # API layer
│   ├── hooks/            # Custom React hooks
│   ├── context/          # React Context
│   ├── utils/            # Utilities
│   ├── App.tsx          # Root component (to create)
│   ├── main.tsx         # Entry point (to create)
│   └── index.css        # Global styles (to create)
├── .env                  # Environment variables (created from .env.example)
├── .eslintrc.cjs        # ESLint config
├── .gitignore           # Git ignore
├── index.html           # HTML template
├── package.json         # Dependencies
├── postcss.config.js    # PostCSS config
├── README.md            # Documentation
├── setup.sh             # Setup script
├── tailwind.config.js   # Tailwind config
├── tsconfig.json        # TypeScript config (app)
├── tsconfig.node.json   # TypeScript config (Vite)
└── vite.config.ts       # Vite config
```

---

## 🎨 Color System

The Tailwind configuration includes custom colors for your application:

### Police System Colors (Blue)
- `bg-police-500` - Primary blue (#3b82f6)
- `text-police-700` - Dark blue (#1d4ed8)
- Use for police-related UI elements

### Hospital System Colors (Red)
- `bg-hospital-500` - Primary red (#ef4444)
- `text-hospital-700` - Dark red (#b91c1c)
- Use for hospital-related UI elements

### Status Colors
- `text-success` - Green (#10b981)
- `text-error` - Red (#ef4444)
- `text-warning` - Amber (#f59e0b)
- `text-info` - Blue (#3b82f6)

### Flag Status
- `text-flagged` - Red (#ef4444) for flagged individuals
- `text-unflagged` - Gray (#6b7280) for unflagged

**Example usage:**
```tsx
<button className="bg-police-500 hover:bg-police-700 text-white">
  Police Action
</button>

<button className="bg-hospital-500 hover:bg-hospital-700 text-white">
  Hospital Action
</button>
```

---

## 🔌 API Integration

### Using the Proxy

The Vite proxy is configured to forward requests:

```typescript
// ✅ Correct - Uses proxy (no CORS issues)
axios.get('/api/police/suspects')
axios.get('/api/hospital/patients')

// ❌ Incorrect - Direct request (CORS issues)
axios.get('http://localhost:8000/suspects')
```

### Environment Variables

Access in your code:
```typescript
const policeApiUrl = import.meta.env.VITE_POLICE_API_URL
const hospitalApiUrl = import.meta.env.VITE_HOSPITAL_API_URL
const pollInterval = import.meta.env.VITE_POLL_INTERVAL
```

---

## 📝 Available Scripts

After installation, you can run:

| Command | Description |
|---------|-------------|
| `npm run dev` | Start development server (port 5173) |
| `npm run build` | Build for production |
| `npm run preview` | Preview production build |
| `npm run lint` | Run ESLint |
| `npm run type-check` | TypeScript type checking |

---

## ✅ Verification Checklist

After setup, verify:

- [ ] `npm install` completed without errors
- [ ] `.env` file created and configured
- [ ] `npm run dev` starts server on port 5173
- [ ] Browser opens to `http://localhost:5173`
- [ ] No console errors in browser developer tools
- [ ] Backend health checks work:
  - `curl http://localhost:5173/api/police/health`
  - `curl http://localhost:5173/api/hospital/health`
- [ ] Tailwind CSS is working (test with utility classes)
- [ ] TypeScript compilation works (`npm run type-check`)

---

## 🔧 Next Steps

With the configuration complete, you need to create:

### 1. Main Application Files

Create these files in `src/`:

**src/main.tsx**
```typescript
import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './App'
import './index.css'

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
)
```

**src/index.css**
```css
@tailwind base;
@tailwind components;
@tailwind utilities;

/* Custom styles here */
```

**src/App.tsx**
```typescript
function App() {
  return (
    <div className="min-h-screen bg-gray-50">
      <h1 className="text-3xl font-bold text-center py-8">
        Police-Hospital Data Sharing System
      </h1>
      {/* Your components here */}
    </div>
  )
}

export default App
```

### 2. Type Definitions

Create `src/services/types.ts` with:
- Suspect interface
- Patient interface
- API response types

### 3. API Services

Create `src/services/policeApi.ts` and `hospitalApi.ts` with Axios instances

### 4. Components

Implement components according to `ARCHITECTURE.md`:
- Police components
- Hospital components
- Shared components
- Layout components

### 5. Custom Hooks

Create hooks for:
- Data fetching
- Flag synchronization
- Cross-system queries

---

## 🐛 Troubleshooting

### Issue: npm install fails

**Solution:**
```bash
# Clear npm cache
npm cache clean --force
rm -rf node_modules package-lock.json
npm install
```

### Issue: Port 5173 in use

**Solution:**
```bash
# Kill process
lsof -ti:5173 | xargs kill -9

# Or change port in vite.config.ts
```

### Issue: Backend not accessible

**Solution:**
```bash
# Verify backends are running
curl http://localhost:8000/health
curl http://localhost:8001/health

# Start them if needed
cd backend/police-system && cargo run
cd backend/hospital-system && cargo run
```

### Issue: Tailwind not working

**Solution:**
```bash
# Ensure index.css has Tailwind directives
# Restart dev server
```

---

## 📚 Documentation

- **README.md** - Quick reference and daily usage
- **INSTALLATION.md** - Detailed setup guide
- **ARCHITECTURE.md** (project root) - System design and implementation guide
- **API.md** (docs/) - Backend API documentation

---

## 🎯 Key Features of This Setup

✅ **Production-ready** - Strict TypeScript, ESLint rules, optimized build

✅ **Developer-friendly** - Hot reload, type checking, helpful error messages

✅ **Well-documented** - Comprehensive guides and inline comments

✅ **Customized** - Police/hospital color scheme, proper proxy setup

✅ **Modern stack** - Latest versions of React, TypeScript, Vite, Tailwind

✅ **Type-safe** - Full TypeScript support with strict mode

✅ **Fast development** - Vite's instant HMR and optimized dev server

✅ **Maintainable** - Clear project structure and separation of concerns

---

## 📞 Support

If you encounter issues:

1. Check INSTALLATION.md troubleshooting section
2. Verify all prerequisites are met
3. Ensure backend services are running
4. Check browser console for errors
5. Review terminal output for build errors

---

**Ready to build?** Run `./setup.sh` and start developing! 🚀
