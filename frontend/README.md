# Police & Hospital System - Frontend

A minimal React + TypeScript frontend for displaying data from the police and hospital backend APIs.

## Tech Stack

- **React 18** - UI library
- **TypeScript** - Type safety
- **Vite** - Build tool and dev server
- **No CSS frameworks** - Plain CSS/inline styles only
- **No routing** - Single page application
- **No forms/authentication** - Read-only data display

## Project Structure

```
frontend/
├── index.html           # HTML entry point
├── package.json         # Dependencies and scripts
├── tsconfig.json        # TypeScript configuration
├── vite.config.ts       # Vite configuration with API proxies
└── src/
    ├── main.tsx         # React entry point
    └── App.tsx          # Root component
```

## Prerequisites

- Node.js 18+ and npm
- Backend services running:
  - Police system on `http://localhost:8000`
  - Hospital system on `http://localhost:8001`

## Installation

```bash
cd frontend
npm install
```

## Development

Start the development server:

```bash
npm run dev
```

The frontend will be available at **<http://localhost:3000>**

## API Proxy Configuration

The Vite dev server is configured to proxy API requests to avoid CORS issues:

| Frontend Path     | Backend Target            | Purpose             |
| ----------------- | ------------------------- | ------------------- |
| `/api/police/*`   | `http://localhost:8000/*` | Police system API   |
| `/api/hospital/*` | `http://localhost:8001/*` | Hospital system API |

### Example API Calls

From your React components, make requests to the proxied paths:

```typescript
// Fetch all suspects from police system
fetch("/api/police/suspects")
  .then((res) => res.json())
  .then((data) => console.log(data));

// Fetch all patients from hospital system
fetch("/api/hospital/patients")
  .then((res) => res.json())
  .then((data) => console.log(data));

// Check if a patient has police records
fetch("/api/police/api/shared/suspects/19850312-2398")
  .then((res) => res.json())
  .then((data) => console.log(data));

// Check if a suspect has hospital records
fetch("/api/hospital/api/shared/patients/19850312-2398")
  .then((res) => res.json())
  .then((data) => console.log(data));
```

## Build for Production

```bash
npm run build
```

This creates an optimized production build in the `dist/` directory.

Preview the production build:

```bash
npm run preview
```

## TypeScript Configuration

The project uses strict TypeScript settings:

- `strict: true` - All strict type checking enabled
- `noUnusedLocals: true` - Error on unused local variables
- `noUnusedParameters: true` - Error on unused function parameters
- `noFallthroughCasesInSwitch: true` - Error on switch fallthrough
- `noUncheckedIndexedAccess: true` - Stricter array/object access

## Next Steps

1. Create type definitions for API responses (e.g., `src/types.ts`)
2. Build components to display suspects and patients
3. Implement data fetching with `useEffect` and `useState`
4. Add error handling and loading states
5. Create components for flagged patients/suspects
6. Implement inter-system data cross-referencing

## Troubleshooting

### Port 3000 already in use

Change the port in `vite.config.ts`:

```typescript
server: {
  port: 3001, // or any available port
  // ...
}
```

### API proxy not working

1. Ensure backend services are running on ports 8000 and 8001
2. Check that you're using the proxied paths (`/api/police/*`, `/api/hospital/*`)
3. Restart the Vite dev server after config changes

### Backend CORS errors

The proxy should handle CORS, but if you see CORS errors:

1. Verify the proxy configuration in `vite.config.ts`
2. Check backend CORS settings allow `localhost:3000`
3. Use the proxied paths instead of direct backend URLs

## API Reference

See the main project documentation:

- `docs/API.md` - Complete API documentation
- `docs/TESTING.md` - Backend testing guide

## Development Workflow

1. Start PostgreSQL: `brew services start postgresql@15`
2. Start police backend: `cd backend/police-system && cargo run`
3. Start hospital backend: `cd backend/hospital-system && cargo run`
4. Start frontend: `cd frontend && npm run dev`
5. Open browser to `http://localhost:3000`

---

**Note:** This is a minimal setup focused on displaying data. No state management, routing, or complex features are included by design.
