# Frontend Build Prompts

A sequential series of AI prompts to build the complete frontend for the Police-Hospital system integration thesis project.

## Overview

These prompts are designed to be used in order with an AI coding assistant. Each prompt builds upon the previous one and references the project architecture and API documentation.

**Important Context:**

- Backend APIs are running on ports 8000 (Police) and 8001 (Hospital)
- Complete API documentation is in `docs/API.md`
- Architecture details are in `ARCHITECTURE.md`
- The frontend should be a single unified application, not separate UIs

---

## Prompt 1: Project Setup & Configuration

```
I'm building a React + TypeScript frontend for a thesis project that demonstrates cross-database
synchronization between a police system and hospital system.

Requirements:
- Create a new Vite project in the `frontend/` directory
- Use React 18+ with TypeScript
- Configure Vite to run on port 5173
- Set up Tailwind CSS for styling
- Create a basic project structure with these folders:
  - src/components/ (with subdirs: police/, hospital/, shared/, layout/)
  - src/services/ (for API calls)
  - src/hooks/ (for custom React hooks)
  - src/context/ (for state management)
  - src/utils/ (for helper functions)
  - src/types/ (for TypeScript interfaces)

Create the initial setup including:
1. package.json with all necessary dependencies (react, react-dom, typescript, axios, tailwind)
2. vite.config.ts configured properly
3. tsconfig.json with strict type checking
4. tailwind.config.js
5. .env.example with placeholders for VITE_POLICE_API_URL and VITE_HOSPITAL_API_URL
6. Basic folder structure
7. A simple index.html template

Make it production-ready with proper configuration.
```

---

## Prompt 2: TypeScript Type Definitions

````
Based on the API documentation in docs/API.md, create comprehensive TypeScript type definitions
in `src/types/index.ts`.

The API returns these data structures:

**Suspect (from Police API):**
```json
{
  "id": 1,
  "full_name": "Erik Andersson",
  "personal_id": "19850312-2398",
  "flag": false
}
````

**Patient (from Hospital API):**

```json
{
  "id": 1,
  "full_name": "Erik Andersson",
  "personal_id": "19850312-2398",
  "flag": false
}
```

Create interfaces for:

1. Suspect (with all fields, making appropriate ones optional based on API responses)
2. Patient (with all fields, making appropriate ones optional)
3. CreateSuspect (for POST requests - no id field)
4. CreatePatient (for POST requests - no id field)
5. UpdateSuspect (for PUT requests - personal_id required, others optional)
6. UpdatePatient (for PUT requests - personal_id required, others optional)
7. FlagUpdate (for flag update requests)
8. ApiError (for error responses)
9. HealthResponse (for health check endpoints)

Also create a type for LoadingState and add any utility types needed.

Make sure all types are properly exported and well-documented with JSDoc comments.

```

---

## Prompt 3: API Service Layer - Base Configuration

```

Create the base API configuration and utilities in `src/services/api.ts`.

Requirements:

1. Import axios
2. Read API URLs from environment variables (VITE_POLICE_API_URL, VITE_HOSPITAL_API_URL)
3. Create two axios instances:
   - policeApi (for http://localhost:8000)
   - hospitalApi (for http://localhost:8001)
4. Configure both with:
   - Default headers (Content-Type: application/json)
   - Timeout of 10 seconds
   - Request interceptor to log all outgoing requests (in development)
   - Response interceptor to handle errors globally
5. Create a generic error handler that:
   - Logs errors to console
   - Formats error messages from API responses
   - Returns a consistent error object
6. Export both axios instances and the error handler

Use TypeScript with proper typing for all functions.

```

---

## Prompt 4: Police System API Service

```

Create `src/services/policeApi.ts` with all API functions for the police system.

Based on docs/API.md, implement these functions:

**Suspect Management:**

- `getAllSuspects()` - GET /suspects - Returns Promise<Suspect[]>
- `getSuspectById(id: number)` - GET /suspects/{id} - Returns Promise<Suspect>
- `getSuspectByPersonalId(personalId: string)` - GET /suspects/personal/{personalId} - Returns Promise<Suspect>
- `createSuspect(data: CreateSuspect)` - POST /suspects - Returns Promise<Suspect>
- `updateSuspect(id: number, data: UpdateSuspect)` - PUT /suspects/{id} - Returns Promise<Suspect>
- `deleteSuspect(id: number)` - DELETE /suspects/{id} - Returns Promise<void>

**Flag Management:**

- `updateSuspectFlag(personalId: string, flag: boolean)` - PUT /suspects/{personalId}/flag - Returns Promise<Suspect>

**Cross-System Queries:**

- `queryPatientRecord(personalId: string)` - GET /api/shared/patients/{personalId} - Returns Promise<Patient | null>
- `getAllSharedPatients()` - GET /api/shared/patients - Returns Promise<Patient[]>
- `getFlaggedPatients()` - GET /api/shared/patients/flagged - Returns Promise<Patient[]>

**Health Check:**

- `checkHealth()` - GET /health - Returns Promise<HealthResponse>

Use the policeApi instance from api.ts. Add proper error handling for all functions.
Handle 404 responses gracefully by returning null instead of throwing.
Add JSDoc comments for each function.

```

---

## Prompt 5: Hospital System API Service

```

Create `src/services/hospitalApi.ts` with all API functions for the hospital system.

Based on docs/API.md, implement these functions:

**Patient Management:**

- `getAllPatients()` - GET /patients - Returns Promise<Patient[]>
- `getPatientById(id: number)` - GET /patients/{id} - Returns Promise<Patient>
- `getPatientByPersonalId(personalId: string)` - GET /patients/personal/{personalId} - Returns Promise<Patient>
- `getFlaggedPatients()` - GET /patients/flagged - Returns Promise<Patient[]>
- `createPatient(data: CreatePatient)` - POST /patients - Returns Promise<Patient>
- `updatePatient(id: number, data: UpdatePatient)` - PUT /patients/{id} - Returns Promise<Patient>
- `deletePatient(id: number)` - DELETE /patients/{id} - Returns Promise<void>

**Cross-System Queries:**

- `querySuspectRecord(personalId: string)` - GET /api/shared/suspects/{personalId} - Returns Promise<Suspect | null>
- `getAllSharedSuspects()` - GET /api/shared/suspects - Returns Promise<Suspect[]>

**Health Check:**

- `checkHealth()` - GET /health - Returns Promise<HealthResponse>

Use the hospitalApi instance from api.ts. Add proper error handling for all functions.
Handle 404 responses gracefully by returning null instead of throwing.
Add JSDoc comments for each function.

```

---

## Prompt 6: Validation Utilities

```

Create `src/utils/validation.ts` with validation functions for Swedish personal IDs.

Requirements:

1. Create a function `validateSwedishPersonalId(personalId: string): boolean`

   - Format should be: YYYYMMDD-XXXX
   - Total length: 13 characters
   - First 8 characters must be digits (YYYYMMDD)
   - Character 9 must be a hyphen (-)
   - Last 4 characters must be digits (XXXX)
   - Return true if valid, false otherwise

2. Create a function `formatPersonalId(personalId: string): string`

   - Takes a personal ID (with or without hyphen)
   - Returns it in the standard format YYYYMMDD-XXXX
   - If already formatted, return as-is
   - If only 12 digits, insert hyphen after position 8

3. Create a function `isAdult(personalId: string): boolean`

   - Extracts birth date from personal ID
   - Returns true if person is 18 or older

4. Add comprehensive error handling and input validation
5. Add unit test examples in comments for each function
6. Export all functions

Use TypeScript with proper typing.

```

---

## Prompt 7: Formatting Utilities

```

Create `src/utils/formatting.ts` with data formatting helper functions.

Requirements:

1. `formatDate(date: string | Date): string`

   - Formats dates to Swedish format (YYYY-MM-DD)
   - Handles both string and Date inputs

2. `formatDateTime(date: string | Date): string`

   - Formats to Swedish datetime format (YYYY-MM-DD HH:mm)

3. `formatPersonName(fullName: string | null | undefined): string`

   - Handles null/undefined gracefully
   - Returns "Unnamed" for empty values
   - Properly capitalizes names

4. `getInitials(fullName: string): string`

   - Extracts initials from full name (e.g., "Erik Andersson" -> "EA")

5. `getFlagStatusText(flag: boolean | null | undefined): string`

   - Returns "Flagged" for true
   - Returns "Not Flagged" for false
   - Returns "Unknown" for null/undefined

6. `getFlagStatusColor(flag: boolean | null | undefined): string`
   - Returns Tailwind color classes
   - "text-red-600" for flagged
   - "text-green-600" for not flagged
   - "text-gray-400" for unknown

Export all functions with TypeScript types and JSDoc comments.

```

---

## Prompt 8: Application Context Provider

```

Create `src/context/AppContext.tsx` for global state management using React Context API.

Requirements:

1. Create an AppContext that manages:

   - Police data (suspects: Suspect[], loading: boolean, error: string | null)
   - Hospital data (patients: Patient[], flaggedPatients: Patient[], loading: boolean, error: string | null)
   - Sync status (lastSyncTime: Date | null, syncStatus: 'idle' | 'syncing' | 'success' | 'error')

2. Create an AppProvider component that:

   - Provides initial state
   - Includes methods to update each part of the state
   - Handles loading and error states properly

3. Include these methods in the context:

   - `refreshPoliceData()` - Fetches all suspects
   - `refreshHospitalData()` - Fetches all patients and flagged patients
   - `refreshAll()` - Refreshes both systems
   - `updateSyncStatus(status)` - Updates synchronization status
   - `setPoliceLoading(loading: boolean)` - Updates police loading state
   - `setHospitalLoading(loading: boolean)` - Updates hospital loading state
   - `setPoliceError(error: string | null)` - Updates police error state
   - `setHospitalError(error: string | null)` - Updates hospital error state

4. Create a custom hook `useAppContext()` that:

   - Returns the context
   - Throws an error if used outside the provider

5. Use TypeScript with proper typing for all state and methods
6. Add JSDoc comments explaining the context structure

The provider should wrap the entire app and make this data available to all components.

```

---

## Prompt 9: Custom Hook - usePoliceData

```

Create `src/hooks/usePoliceData.ts` - a custom hook for managing police system data.

Requirements:

1. Import necessary services from policeApi.ts
2. Import useAppContext to access global state
3. Return an object with:

   **State:**

   - suspects: Suspect[]
   - loading: boolean
   - error: string | null

   **Methods:**

   - `fetchSuspects()` - Loads all suspects
   - `fetchSuspectById(id: number)` - Gets single suspect
   - `fetchSuspectByPersonalId(personalId: string)` - Gets suspect by personal ID
   - `createSuspect(data: CreateSuspect)` - Creates new suspect
   - `updateSuspect(id: number, data: UpdateSuspect)` - Updates suspect
   - `deleteSuspect(id: number)` - Deletes suspect
   - `updateFlag(personalId: string, flag: boolean)` - Updates flag and refreshes data
   - `queryPatient(personalId: string)` - Queries hospital for patient record

4. Each method should:

   - Set loading state to true before the API call
   - Handle errors gracefully and update error state
   - Set loading to false after completion
   - Update the global context state after successful operations

5. Include a `useEffect` that automatically fetches suspects on mount

6. Add proper TypeScript typing and error handling
7. Add JSDoc comments for each method

This hook should be the primary way components interact with police data.

```

---

## Prompt 10: Custom Hook - useHospitalData

```

Create `src/hooks/useHospitalData.ts` - a custom hook for managing hospital system data.

Requirements:

1. Import necessary services from hospitalApi.ts
2. Import useAppContext to access global state
3. Return an object with:

   **State:**

   - patients: Patient[]
   - flaggedPatients: Patient[]
   - loading: boolean
   - error: string | null

   **Methods:**

   - `fetchPatients()` - Loads all patients
   - `fetchFlaggedPatients()` - Loads only flagged patients
   - `fetchPatientById(id: number)` - Gets single patient
   - `fetchPatientByPersonalId(personalId: string)` - Gets patient by personal ID
   - `createPatient(data: CreatePatient)` - Creates new patient
   - `updatePatient(id: number, data: UpdatePatient)` - Updates patient
   - `deletePatient(id: number)` - Deletes patient
   - `querySuspect(personalId: string)` - Queries police for suspect record

4. Each method should:

   - Set loading state to true before the API call
   - Handle errors gracefully and update error state
   - Set loading to false after completion
   - Update the global context state after successful operations

5. Include a `useEffect` that automatically fetches patients and flagged patients on mount

6. Add proper TypeScript typing and error handling
7. Add JSDoc comments for each method

This hook should be the primary way components interact with hospital data.

```

---

## Prompt 11: Custom Hook - useFlagSync

```

Create `src/hooks/useFlagSync.ts` - a hook for monitoring flag synchronization between systems.

Requirements:

1. Use polling to check for flag synchronization status
2. Compare flags between police suspects and hospital patients with matching personal_ids
3. Return an object with:

   **State:**

   - `syncedRecords`: Array of { personalId: string, isSynced: boolean, policeFlag: boolean, hospitalFlag: boolean }
   - `outOfSync`: number (count of records where flags don't match)
   - `lastChecked`: Date | null
   - `isChecking`: boolean

   **Methods:**

   - `checkSync()` - Manually trigger a synchronization check
   - `startPolling(intervalMs: number)` - Start automatic polling
   - `stopPolling()` - Stop automatic polling

4. The hook should:

   - Fetch both suspects and patients
   - Match records by personal_id
   - Compare flag values
   - Identify any mismatches (which shouldn't happen due to database triggers, but good to verify)
   - Default to checking every 5 seconds

5. Use TypeScript with proper interfaces for the return object
6. Add cleanup in useEffect to stop polling when component unmounts
7. Add JSDoc comments explaining the synchronization logic

This hook helps visualize that the automatic database synchronization is working.

```

---

## Prompt 12: Layout Component - Header

```

Create `src/components/layout/Header.tsx` - the main application header.

Requirements:

1. Display the application title "Police-Hospital System Integration"
2. Show system status indicators:
   - Police System (green dot if healthy, red if error, gray if loading)
   - Hospital System (green dot if healthy, red if error, gray if loading)
3. Display last sync time from AppContext
4. Show a sync status indicator (idle/syncing/success/error)
5. Include a manual "Refresh All" button that calls refreshAll() from context
6. Make it responsive (stack on mobile, inline on desktop)
7. Use Tailwind CSS for styling with a professional look:

   - Dark blue background (#1a237e) for police theme
   - Clean white text
   - Status dots with proper colors
   - Smooth transitions

8. Use TypeScript with proper typing
9. Import and use the useAppContext hook
10. Add a useEffect to check health of both systems every 30 seconds

The header should be fixed at the top and always visible.

```

---

## Prompt 13: Layout Component - SystemPanel

```

Create `src/components/layout/SystemPanel.tsx` - a reusable container for each system's view.

Requirements:

1. Accept these props:

   - title: string (e.g., "Police System" or "Hospital System")
   - icon: string (emoji like "🚔" or "🏥")
   - color: "police" | "hospital" (determines color scheme)
   - children: ReactNode (the content to display)
   - loading: boolean (shows loading spinner)
   - error: string | null (displays error message if present)

2. Render a card-style panel with:

   - Title bar with icon and title
   - Color-coded left border (blue for police, red for hospital)
   - Content area that displays children
   - Loading overlay when loading is true
   - Error message display when error is present
   - Refresh button that can be passed as a callback prop

3. Use Tailwind CSS with:

   - Rounded corners
   - Shadow for depth
   - Smooth transitions
   - Responsive sizing
   - Clean typography

4. Use TypeScript with proper prop interfaces
5. Make it fully reusable for both police and hospital panels

This component provides consistent styling across both system views.

```

---

## Prompt 14: Shared Component - PersonCard

```

Create `src/components/shared/PersonCard.tsx` - a reusable card for displaying person information.

Requirements:

1. Accept these props:

   - person: Suspect | Patient (using a union type or generic)
   - type: "suspect" | "patient"
   - onViewDetails?: (id: number) => void (optional callback)
   - onEdit?: (id: number) => void (optional callback)
   - onDelete?: (id: number) => void (optional callback)
   - onToggleFlag?: (personalId: string, currentFlag: boolean) => void (optional callback)
   - showActions?: boolean (default true)

2. Display:

   - Full name (large, bold)
   - Personal ID (Swedish format with validation)
   - Flag status (badge - red if flagged, green if not)
   - ID number (small, gray)
   - Action buttons (if showActions is true)

3. Style with Tailwind:

   - Card layout with hover effect
   - Color-coded left border based on type (blue for suspect, red for patient)
   - Flag badge with appropriate colors
   - Action buttons with icons (use emojis: 👁️ 📝 🗑️ 🚩)
   - Responsive layout (stack on mobile)

4. Handle all callbacks properly with null checks
5. Use the formatting utilities for name and personal ID
6. Use TypeScript with proper typing
7. Add accessibility attributes (aria-labels)

This is the primary component for displaying person records throughout the app.

```

---

## Prompt 15: Shared Component - SyncIndicator

```

Create `src/components/shared/SyncIndicator.tsx` - a visual indicator for flag synchronization status.

Requirements:

1. Use the useFlagSync hook to get synchronization data
2. Accept these props:

   - autoRefresh?: boolean (default true - whether to poll automatically)
   - refreshInterval?: number (default 5000ms)

3. Display:

   - Total synced records count
   - Out of sync records count (should be 0 if working correctly)
   - Last checked timestamp
   - Visual sync status (animated when checking)
   - List of any out-of-sync records (if any exist - for debugging)

4. Visual states:

   - All synced: Green checkmark ✅ "All flags synchronized"
   - Checking: Blue spinner 🔄 "Checking synchronization..."
   - Out of sync: Red warning ⚠️ "X records out of sync"
   - Error: Red X ❌ "Sync check failed"

5. Include a manual "Check Now" button
6. Show last checked time in relative format ("2 seconds ago", "1 minute ago")
7. Use Tailwind for styling with smooth animations
8. Add a details section that can be expanded to show sync details for each record

9. Use TypeScript with proper typing
10. Add polling cleanup on unmount

This component helps verify that the automatic database synchronization is working as expected.

```

---

## Prompt 16: Police Component - SuspectList

```

Create `src/components/police/SuspectList.tsx` - displays all suspects in a list/grid.

Requirements:

1. Use the usePoliceData hook to get suspects
2. Display suspects using the PersonCard component
3. Include:

   - Search/filter by name or personal ID
   - Sort options (by name, by personal ID, by flag status)
   - Filter by flag status (All / Flagged / Not Flagged)
   - Grid layout (responsive: 1 column mobile, 2-3 columns desktop)

4. Show:

   - Total count of suspects
   - Count of flagged suspects
   - Loading state
   - Error state
   - Empty state (when no suspects)

5. Wire up PersonCard callbacks:

   - onViewDetails: Show detailed view (can be a modal or separate component)
   - onEdit: Open edit form
   - onDelete: Confirm and delete suspect
   - onToggleFlag: Update flag and show success message

6. Add a "Create New Suspect" button at the top
7. Use Tailwind for styling
8. Include proper TypeScript typing
9. Add smooth transitions for list updates
10. Show a toast/notification when operations succeed or fail

This is the main view for the police system.

```

---

## Prompt 17: Police Component - SuspectForm

```

Create `src/components/police/SuspectForm.tsx` - form for creating/editing suspects.

Requirements:

1. Accept these props:

   - mode: "create" | "edit"
   - suspect?: Suspect (for edit mode)
   - onSubmit: (data: CreateSuspect | UpdateSuspect) => Promise<void>
   - onCancel: () => void

2. Form fields:

   - Full Name (text input, required)
   - Personal ID (text input with validation, required)
   - Flag status (checkbox, default false)

3. Validation:

   - Full name: Required, at least 2 characters
   - Personal ID: Required, must match Swedish format (use validateSwedishPersonalId)
   - Show validation errors inline

4. Features:

   - Real-time validation as user types
   - Disable submit button while submitting
   - Show loading spinner during submission
   - Auto-format personal ID on blur
   - Clear form after successful creation
   - Populate form in edit mode

5. Style with Tailwind:

   - Clean form layout
   - Clear labels
   - Input focus states
   - Error message styling
   - Primary button (blue for police theme)
   - Cancel button (gray)

6. Use React Hook Form or controlled components
7. Use TypeScript with proper typing
8. Add accessibility attributes
9. Handle success/error from onSubmit callback

This form handles both creating new suspects and editing existing ones.

```

---

## Prompt 18: Police Component - FlagControl

```

Create `src/components/police/FlagControl.tsx` - dedicated interface for flagging/unflagging suspects.

Requirements:

1. Use the usePoliceData hook
2. Display a search interface to find suspects by personal ID
3. When a suspect is found:

   - Show their details (name, personal ID, current flag status)
   - Display a prominent toggle button to change flag status
   - Show the current flag status clearly (large badge)
   - Include a confirmation dialog when flagging/unflagging

4. After flag update:

   - Show success message with animation
   - Display a note: "Flag automatically synchronized to hospital system"
   - Show before/after flag status
   - Provide a link to check the hospital record

5. Include a section showing:

   - Recent flag changes (last 5 operations)
   - Total flagged suspects count
   - Quick actions to flag/unflag multiple suspects

6. Style with Tailwind:

   - Large, clear toggle switch or button
   - Color-coded flag status (red = flagged, green = not flagged)
   - Visual feedback for sync confirmation
   - Professional card layout

7. Use TypeScript with proper typing
8. Add sound/visual confirmation when flag is updated (optional)
9. Show loading state during API call

This component makes the core flag synchronization feature prominent and easy to use.

```

---

## Prompt 19: Hospital Component - PatientList

```

Create `src/components/hospital/PatientList.tsx` - displays all patients in a list/grid.

Requirements:

1. Use the useHospitalData hook to get patients
2. Display patients using the PersonCard component
3. Include:

   - Search/filter by name or personal ID
   - Sort options (by name, by personal ID, by flag status)
   - Filter by flag status (All / Flagged / Not Flagged)
   - Grid layout (responsive: 1 column mobile, 2-3 columns desktop)

4. Show:

   - Total count of patients
   - Count of flagged patients (with note: "Flagged by police system")
   - Loading state
   - Error state
   - Empty state (when no patients)

5. Wire up PersonCard callbacks:

   - onViewDetails: Show detailed view
   - onEdit: Open edit form
   - onDelete: Confirm and delete patient
   - onToggleFlag: Disabled (with tooltip explaining flags are managed by police)

6. Add a "Register New Patient" button at the top
7. Highlight flagged patients visually (red border or badge)
8. Use Tailwind for styling
9. Include proper TypeScript typing
10. Show a notification banner if any flagged patients exist

This is the main view for the hospital system.

```

---

## Prompt 20: Hospital Component - PatientForm

```

Create `src/components/hospital/PatientForm.tsx` - form for creating/editing patients.

Requirements:

1. Accept these props:

   - mode: "create" | "edit"
   - patient?: Patient (for edit mode)
   - onSubmit: (data: CreatePatient | UpdatePatient) => Promise<void>
   - onCancel: () => void

2. Form fields:

   - Full Name (text input, required)
   - Personal ID (text input with validation, required)
   - Flag status (display only, not editable - show note: "Managed by police system")

3. Validation:

   - Full name: Required, at least 2 characters
   - Personal ID: Required, must match Swedish format
   - Show validation errors inline

4. Features:

   - Real-time validation as user types
   - Disable submit button while submitting
   - Show loading spinner during submission
   - Auto-format personal ID on blur
   - Clear form after successful creation
   - Populate form in edit mode
   - Check if patient exists in police system (show warning if they're a flagged suspect)

5. Style with Tailwind:

   - Clean form layout
   - Clear labels
   - Input focus states
   - Error message styling
   - Primary button (red for hospital theme)
   - Cancel button (gray)
   - Warning banner if flagged in police system

6. Use React Hook Form or controlled components
7. Use TypeScript with proper typing
8. Add accessibility attributes

This form handles both registering new patients and editing existing ones.

```

---

## Prompt 21: Hospital Component - FlaggedPatients

```

Create `src/components/hospital/FlaggedPatients.tsx` - dedicated view for flagged patients.

Requirements:

1. Use the useHospitalData hook to get flagged patients
2. Display a prominent list of all flagged patients
3. For each flagged patient show:

   - Full name and personal ID
   - Flag status badge (always red/flagged)
   - When they were flagged (if available)
   - Option to view their full details
   - Option to check their police record

4. Include:

   - Total count of flagged patients
   - Filter/search by name or personal ID
   - Sort by name or personal ID
   - Export to CSV button (optional)

5. Add an information banner at the top explaining:

   - "These patients are automatically flagged by the police system"
   - "Flags cannot be changed from the hospital interface"
   - "Flag changes sync instantly from police database"

6. Provide action buttons:

   - "Check Police Record" - queries the police system for details
   - "View Patient Details" - shows full patient information
   - "Refresh" - manually refresh the flagged list

7. Style with Tailwind:

   - Alert/warning aesthetic (red theme)
   - Clear badges and status indicators
   - Professional table or card layout
   - Responsive design

8. Use TypeScript with proper typing
9. Show empty state if no flagged patients
10. Add real-time refresh (poll every 10 seconds)

This component helps hospital staff monitor flagged individuals.

```

---

## Prompt 22: Shared Component - CrossSystemQuery

```

Create `src/components/shared/CrossSystemQuery.tsx` - interface for querying one system from another.

Requirements:

1. Accept these props:

   - system: "police" | "hospital" (which system to query from)
   - oppositeSystem: "hospital" | "police" (which system to query)

2. UI Elements:

   - Input field for Swedish personal ID
   - "Search" button
   - Results display area
   - Loading state
   - Error handling

3. When police queries hospital:

   - Use querySuspectRecord from usePoliceData
   - Show "Check if suspect has hospital records"
   - Display patient details if found
   - Show "No hospital record found" if not found

4. When hospital queries police:

   - Use queryPatient from useHospitalData
   - Show "Check if patient has police record"
   - Display suspect details if found (including flag status)
   - Show "No police record found" if not found
   - If flagged suspect found, show warning banner

5. Display results using PersonCard component
6. Include:

   - Personal ID validation before search
   - Search history (last 5 queries)
   - "Search Another" button after results
   - Clear results button

7. Style with Tailwind:

   - Search bar with icon
   - Results card with appropriate colors
   - Warning styling if flagged suspect found
   - Loading skeleton

8. Use TypeScript with proper typing
9. Add helpful tooltips explaining cross-system queries

This component demonstrates the inter-system communication capability.

```

---

## Prompt 23: Main App Component

```

Create `src/App.tsx` - the main application component that brings everything together.

Requirements:

1. Import all necessary components and providers
2. Wrap the entire app with AppProvider from AppContext
3. Create a layout with:

   - Header component at the top (fixed)
   - Two-panel layout for Police and Hospital systems (side-by-side on desktop, stacked on mobile)
   - Footer with project info

4. Left Panel (Police System):

   - SystemPanel component with "police" color
   - Tabs or sections for:
     - SuspectList (default view)
     - FlagControl
     - CrossSystemQuery (querying hospital)
   - Display police data loading/error states

5. Right Panel (Hospital System):

   - SystemPanel component with "hospital" color
   - Tabs or sections for:
     - PatientList (default view)
     - FlaggedPatients
     - CrossSystemQuery (querying police)
   - Display hospital data loading/error states

6. Include:

   - SyncIndicator component (floating or in a dedicated section)
   - Global error boundary
   - Loading states for initial data fetch
   - Responsive layout (panels stack on mobile)

7. Add a simple tab/section navigation within each panel
8. Use Tailwind for layout (grid or flexbox)
9. Use TypeScript with proper typing
10. Include comments explaining the structure

The App should be clean, professional, and demonstrate all key features of the system.

```

---

## Prompt 24: Global Styles & Theme

```

Create `src/index.css` with global styles and Tailwind configuration.

Requirements:

1. Import Tailwind base, components, and utilities
2. Define CSS custom properties (variables) for:

   - Police theme colors (blues: primary, secondary, light)
   - Hospital theme colors (reds: primary, secondary, light)
   - Success color (green)
   - Error color (red)
   - Warning color (yellow)
   - Neutral colors (grays)

3. Add global styles for:

   - Body (font, background, margins)
   - Headings (h1-h6 with appropriate sizes)
   - Links (color, hover states)
   - Buttons (base styles)
   - Forms (inputs, labels, error messages)
   - Cards (consistent padding, shadows, borders)
   - Transitions (smooth animations)

4. Define utility classes:

   - .police-theme (blue color scheme)
   - .hospital-theme (red color scheme)
   - .flag-badge (styled badge for flag status)
   - .status-dot (colored dot for status indicators)
   - .loading-spinner (animated spinner)
   - .error-message (styled error display)
   - .success-message (styled success display)

5. Add responsive breakpoints that match common screen sizes
6. Include print styles (optional)
7. Add smooth scroll behavior
8. Style scrollbars (webkit)

Make the design professional, clean, and accessible.

```

---

## Prompt 25: Environment Configuration & Main Entry

```

Create the final configuration files:

1. **src/main.tsx** - Application entry point

   - Import React and ReactDOM
   - Import App component
   - Import index.css
   - Use ReactDOM.createRoot
   - Wrap App in React.StrictMode
   - Mount to div#root
   - Add error boundary for top-level errors

2. **.env.example** - Template for environment variables

   ```
   VITE_POLICE_API_URL=http://localhost:8000
   VITE_HOSPITAL_API_URL=http://localhost:8001
   VITE_POLL_INTERVAL=5000
   VITE_DEBUG_MODE=false
   ```

3. **.env** - Actual environment file (copy from .env.example)

   - Same content as .env.example
   - This file should be in .gitignore

4. Update **README.md** in frontend directory with:

   - Project description
   - Installation instructions (`npm install`)
   - Development command (`npm run dev`)
   - Build command (`npm run build`)
   - Environment variables explanation
   - Port information (frontend: 5173, police API: 8000, hospital API: 8001)
   - Links to backend documentation

5. Create **.env.local** file for local overrides (optional)

6. Add error boundary component in case of crashes

Make sure all environment variables are properly prefixed with VITE\_ and are accessible via import.meta.env.

```

---

## Prompt 26: Testing & Polish

```

Final polish and testing utilities:

1. Create `src/utils/testHelpers.ts` with:

   - Mock data generators (generateMockSuspect, generateMockPatient)
   - API response mocks for testing
   - Helper functions for testing components

2. Add a development mode feature flag check
3. Add console logging only in development mode
4. Create a `src/config/constants.ts` file with:

   - API endpoints
   - Polling intervals
   - Validation rules
   - UI constants (max items per page, etc.)

5. Add PropTypes or improve TypeScript strictness
6. Add loading skeletons for better UX during data fetching
7. Add toast notifications for user feedback (success/error messages)
8. Implement optimistic updates where appropriate

9. Create a simple error boundary component for graceful error handling

10. Add keyboard shortcuts for power users:

    - Ctrl/Cmd + K: Focus search
    - Ctrl/Cmd + N: Create new record
    - Ctrl/Cmd + R: Refresh data
    - Escape: Close modals/forms

11. Add accessibility improvements:

    - ARIA labels
    - Keyboard navigation
    - Focus management
    - Screen reader support

12. Performance optimizations:
    - React.memo for expensive components
    - useMemo for expensive calculations
    - useCallback for stable function references
    - Lazy loading for routes (if implementing routing)

Document all features in code comments.

````

---

## Usage Instructions

### How to Use These Prompts

1. **Sequential Execution**: Use these prompts in order, as each builds upon previous work
2. **Context**: Always provide the AI with access to ARCHITECTURE.md and docs/API.md
3. **Iteration**: Review each generated component and refine as needed
4. **Testing**: Test each component individually before moving to the next
5. **Integration**: After all components are built, test the entire application

### Starting the Development Process

```bash
# 1. Ensure backend is running
cd backend/police-system && cargo run  # Terminal 1
cd backend/hospital-system && cargo run  # Terminal 2

# 2. Start with Prompt 1 to set up the project
# 3. Work through prompts 2-26 sequentially
# 4. After frontend is complete, start the dev server
cd frontend && npm run dev  # Terminal 3

# 5. Access the application at http://localhost:5173
````

### Verification Checklist

After completing all prompts, verify:

- [ ] Frontend runs without errors
- [ ] Can view suspects and patients
- [ ] Can create new records
- [ ] Can update existing records
- [ ] Can delete records
- [ ] Flag updates in police system show in hospital system
- [ ] Cross-system queries work (hospital queries police, police queries hospital)
- [ ] SyncIndicator shows all records are synchronized
- [ ] Forms validate Swedish personal IDs correctly
- [ ] Responsive design works on mobile and desktop
- [ ] Error states display correctly
- [ ] Loading states display correctly
- [ ] All TypeScript types are properly defined

### Additional Resources

- **Backend API Documentation**: `docs/API.md`
- **Architecture Overview**: `ARCHITECTURE.md`
- **Database Setup**: `docs/psql-guide.md`
- **Backend Testing**: `docs/TESTING.md`

### Common Issues & Solutions

**Issue**: Environment variables not loading

- Solution: Ensure variables are prefixed with `VITE_`
- Restart dev server after changing .env

**Issue**: CORS errors

- Solution: Backend CORS is configured for localhost:5173, verify backend is running

**Issue**: API calls failing

- Solution: Check backend services are running on ports 8000 and 8001

**Issue**: TypeScript errors

- Solution: Ensure all types are properly imported and exported

---

## Notes

- These prompts assume you're using an AI that can read your project files
- Some prompts may need adjustment based on specific requirements
- Feel free to modify components after generation
- Add additional features as needed for your thesis
- Consider adding routing (React Router) if you want separate pages
- Consider adding authentication in the future
- The focus is on demonstrating cross-system synchronization, not production deployment

---

**Total Prompts**: 26
**Estimated Time**: 4-8 hours for complete implementation
**Difficulty**: Intermediate to Advanced
**Technologies**: React, TypeScript, Vite, Tailwind CSS, Axios

Good luck with your thesis project! 🎓
