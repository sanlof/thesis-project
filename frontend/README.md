# Thesis Project Frontend

React + TypeScript + Vite frontend for the Police-Hospital Data Sharing System demo.

## Quick Start

### Prerequisites

- Node.js >= 18.0.0
- npm >= 9.0.0
- Backend services running on ports 8000 (police) and 8001 (hospital)

### Installation

1. **Install dependencies:**
   ```bash
   npm install
   ```

2. **Configure environment variables:**
   ```bash
   cp .env.example .env
   ```
   
   Edit `.env` if your backend services use different ports.

3. **Start development server:**
   ```bash
   npm run dev
   ```
   
   The application will be available at `http://localhost:5173`

## Available Scripts

- `npm run dev` - Start development server with hot reload
- `npm run build` - Build for production
- `npm run preview` - Preview production build locally
- `npm run lint` - Run ESLint for code quality checks
- `npm run type-check` - Run TypeScript type checking without emitting files

## Project Structure

```
frontend/
├── public/               # Static assets
│   └── vite.svg
├── src/
│   ├── components/       # React components
│   │   ├── police/      # Police system components
│   │   ├── hospital/    # Hospital system components
│   │   ├── shared/      # Shared components
│   │   └── layout/      # Layout components
│   ├── services/        # API communication layer
│   ├── hooks/           # Custom React hooks
│   ├── context/         # React Context for state management
│   ├── utils/           # Utility functions
│   ├── App.tsx          # Root component
│   ├── main.tsx         # Application entry point
│   └── index.css        # Global styles with Tailwind directives
├── index.html           # HTML template
├── package.json         # Dependencies and scripts
├── tsconfig.json        # TypeScript configuration
├── vite.config.ts       # Vite configuration
├── tailwind.config.js   # Tailwind CSS configuration
├── postcss.config.js    # PostCSS configuration
└── .eslintrc.cjs        # ESLint configuration
```

## Technology Stack

- **React 18.2** - UI library
- **TypeScript 5.3** - Type safety
- **Vite 5.0** - Build tool and dev server
- **Axios 1.6** - HTTP client
- **Tailwind CSS 3.3** - Utility-first CSS framework

## Development

### Proxy Configuration

Vite is configured to proxy API requests to avoid CORS issues:

- `/api/police/*` → `http://localhost:8000`
- `/api/hospital/*` → `http://localhost:8001`

### Color Scheme

The project uses custom Tailwind colors:

- **Police System**: Blue tones (`police-*`)
- **Hospital System**: Red tones (`hospital-*`)
- **Status Colors**: Green (success), Red (error), Amber (warning), Blue (info)

### Type Safety

The project uses strict TypeScript configuration for maximum type safety:

- All code is type-checked
- Unused variables and parameters are flagged
- Implicit any is not allowed
- Strict null checks enabled

## Building for Production

```bash
npm run build
```

The build output will be in the `dist/` directory.

### Preview Production Build

```bash
npm run preview
```

## Environment Variables

See `.env.example` for available configuration options:

- `VITE_POLICE_API_URL` - Police backend API URL
- `VITE_HOSPITAL_API_URL` - Hospital backend API URL
- `VITE_POLL_INTERVAL` - Data refresh interval (milliseconds)
- `VITE_DEBUG_MODE` - Enable debug logging
- `VITE_API_TIMEOUT` - API request timeout

## Features

- **Side-by-side system views** - Police and hospital data displayed simultaneously
- **Flag synchronization** - Real-time visualization of flag updates
- **Cross-system queries** - Query one system from another
- **CRUD operations** - Create, read, update, delete for both systems
- **Responsive design** - Works on desktop and mobile devices
- **Type-safe** - Full TypeScript support

## Troubleshooting

### Port 5173 already in use

```bash
# Kill process using port 5173
lsof -ti:5173 | xargs kill -9

# Or change port in vite.config.ts
```

### Backend connection issues

1. Verify backend services are running:
   ```bash
   curl http://localhost:8000/health
   curl http://localhost:8001/health
   ```

2. Check `.env` file has correct API URLs

3. Look for CORS errors in browser console

### Type errors

```bash
# Run type checking
npm run type-check

# Check specific file
npx tsc --noEmit src/path/to/file.tsx
```

## Next Steps

After setup, you'll need to implement:

1. Component files in `src/components/`
2. API service layer in `src/services/`
3. Custom hooks in `src/hooks/`
4. Type definitions in `src/services/types.ts`
5. Utility functions in `src/utils/`

See `ARCHITECTURE.md` in the project root for detailed implementation guidance.

## License

This is a thesis project for educational purposes.
