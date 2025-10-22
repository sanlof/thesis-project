# Frontend Development Prompt Sequence

Below is a comprehensive sequence of AI prompts designed to generate the complete frontend code for your thesis project. Each prompt builds upon the previous ones and aligns with your backend APIs and architecture.

---

## **Prompt 1: Project Setup and Configuration**

```
I'm building a React + TypeScript frontend for a thesis project that demonstrates
data sharing between police and hospital systems. The frontend should use Vite
as the build tool.

Based on this architecture:
- React 18+ with TypeScript
- Vite for development and building
- Port 5173 for development server
- Axios for HTTP requests
- Tailwind CSS for styling

Create the complete project setup including:

1. A proper package.json with all necessary dependencies:
   - react, react-dom (^18.2.0)
   - typescript (^5.0.0)
   - vite (^5.0.0)
   - axios (^1.6.0)
   - All necessary @types packages

2. A vite.config.ts configured for React with:
   - Proper port configuration (5173)
   - Proxy configuration for backend APIs:
     * /api/police -> http://localhost:8000
     * /api/hospital -> http://localhost:8001

3. A tsconfig.json with strict TypeScript configuration

4. A tailwind.config.js with a clean, professional color scheme suitable for
   police (blue tones) and hospital (red tones) systems

5. A postcss.config.js for Tailwind

6. An index.html template in the public folder

7. A .env.example file with:
   VITE_POLICE_API_URL=http://localhost:8000
   VITE_HOSPITAL_API_URL=http://localhost:8001
   VITE_POLL_INTERVAL=5000

Provide all configuration files with proper formatting and comments.
```

---

## **Prompt 2: TypeScript Type Definitions**

```
Create comprehensive TypeScript type definitions for the frontend based on these
backend API models:

SUSPECT (from police system):
{
  "id": number,
  "full_name": string | null,
  "personal_id": string | null,  // Swedish format: YYYYMMDD-XXXX
  "flag": boolean | null
}

PATIENT (from hospital system):
{
  "id": number,
  "full_name": string | null,
  "personal_id": string | null,  // Swedish format: YYYYMMDD-XXXX
  "flag": boolean | null
}

Create a file src/types/index.ts with:

1. Interface definitions for:
   - Suspect
   - Patient
   - CreateSuspect (for POST requests - no id field, flag defaults to false)
   - CreatePatient (for POST requests - no id field, flag defaults to false)
   - UpdateSuspect (personal_id required, other fields optional)
   - UpdatePatient (personal_id required, other fields optional)
   - FlagUpdate (just flag: boolean)

2. A type for API error responses:
   - ApiError interface with error: string

3. A type for API loading states:
   - LoadingState type union

4. Utility types for common patterns:
   - QueryResult<T> generic type
   - SyncStatus type for flag synchronization states

Include JSDoc comments for each type explaining its purpose and when it's used.
```

---

## **Prompt 3: API Service Layer - Base Configuration**

```
Create the base API service configuration using Axios. This will be used by both
police and hospital API services.

Create src/services/api.ts with:

1. An Axios instance configured with:
   - Base configuration for headers (Content-Type: application/json)
   - Timeout of 10 seconds
   - Response interceptor that:
     * Returns data directly on success
     * Handles errors gracefully and returns structured error objects
     * Logs errors in development mode
   - Request interceptor for debugging in development

2. Helper functions:
   - handleApiError(error: any): ApiError - standardizes error responses
   - buildUrl(base: string, path: string): string - constructs full URLs

3. Constants:
   - POLICE_API_URL from environment variables
   - HOSPITAL_API_URL from environment variables
   - POLL_INTERVAL from environment variables

4. Export everything needed by other service files

Include error handling for network failures, timeouts, and malformed responses.
Add detailed comments explaining the interceptor logic.
```

---

## **Prompt 4: Police System API Service**

```
Create a complete API service for the police system based on these endpoints:

POLICE API (http://localhost:8000):
- GET /suspects - Get all suspects
- GET /suspects/{id} - Get suspect by database ID
- GET /suspects/personal/{personal_id} - Get suspect by Swedish personal ID
- POST /suspects - Create new suspect
- PUT /suspects/{id} - Update suspect
- DELETE /suspects/{id} - Delete suspect
- PUT /suspects/{personal_id}/flag - Update flag status
- GET /api/shared/suspects - Get all suspects (for hospital to query)
- GET /api/shared/suspects/{personal_id} - Check specific suspect (for hospital)

Create src/services/policeApi.ts with:

1. All CRUD operations with proper TypeScript typing:
   - getAllSuspects(): Promise<Suspect[]>
   - getSuspectById(id: number): Promise<Suspect>
   - getSuspectByPersonalId(personalId: string): Promise<Suspect>
   - createSuspect(data: CreateSuspect): Promise<Suspect>
   - updateSuspect(id: number, data: UpdateSuspect): Promise<Suspect>
   - deleteSuspect(id: number): Promise<void>
   - updateSuspectFlag(personalId: string, flag: boolean): Promise<Suspect>

2. Shared API functions for hospital to use:
   - queryPatient(personalId: string): Promise<Patient | null>
   - getAllPatients(): Promise<Patient[]>

3. Proper error handling with try-catch blocks
4. Console logging for debugging in development
5. JSDoc comments for each function

Use the base API configuration from api.ts and import all necessary types.
```

---

## **Prompt 5: Hospital System API Service**

```
Create a complete API service for the hospital system based on these endpoints:

HOSPITAL API (http://localhost:8001):
- GET /patients - Get all patients
- GET /patients/{id} - Get patient by database ID
- GET /patients/personal/{personal_id} - Get patient by Swedish personal ID
- GET /patients/flagged - Get all flagged patients
- POST /patients - Create new patient
- PUT /patients/{id} - Update patient
- DELETE /patients/{id} - Delete patient
- GET /api/shared/patients - Get all patients (for police to query)
- GET /api/shared/patients/flagged - Get flagged patients (for police)
- GET /api/shared/patients/{personal_id} - Check specific patient (for police)

Create src/services/hospitalApi.ts with:

1. All CRUD operations with proper TypeScript typing:
   - getAllPatients(): Promise<Patient[]>
   - getPatientById(id: number): Promise<Patient>
   - getPatientByPersonalId(personalId: string): Promise<Patient>
   - getFlaggedPatients(): Promise<Patient[]>
   - createPatient(data: CreatePatient): Promise<Patient>
   - updatePatient(id: number, data: UpdatePatient): Promise<Patient>
   - deletePatient(id: number): Promise<void>

2. Shared API functions for police to use:
   - querySuspect(personalId: string): Promise<Suspect | null>
   - getAllSuspects(): Promise<Suspect[]>

3. Proper error handling with try-catch blocks
4. Console logging for debugging in development
5. JSDoc comments for each function

Use the base API configuration from api.ts and import all necessary types.
```

---

## **Prompt 6: Custom Hooks - Police Data Management**

```
Create a custom React hook for managing police system data with automatic
refresh and state management.

Create src/hooks/usePoliceData.ts with:

1. A hook that:
   - Fetches all suspects on mount
   - Provides loading, error, and data states
   - Auto-refreshes every 5 seconds (configurable via VITE_POLL_INTERVAL)
   - Provides CRUD operations:
     * createSuspect(data: CreateSuspect)
     * updateSuspect(id: number, data: UpdateSuspect)
     * deleteSuspect(id: number)
     * updateFlag(personalId: string, flag: boolean)
   - Automatically refreshes data after mutations
   - Handles errors gracefully with user-friendly messages

2. Return type should include:
   - suspects: Suspect[]
   - loading: boolean
   - error: string | null
   - refetch: () => Promise<void>
   - createSuspect: (data: CreateSuspect) => Promise<void>
   - updateSuspect: (id: number, data: UpdateSuspect) => Promise<void>
   - deleteSuspect: (id: number) => Promise<void>
   - updateFlag: (personalId: string, flag: boolean) => Promise<void>

3. Use useEffect for polling with cleanup
4. Use useState for state management
5. Use useCallback for memoizing functions
6. Include TypeScript types for all parameters and return values
7. Add detailed comments explaining the polling logic

The hook should handle race conditions and prevent memory leaks.
```

---

## **Prompt 7: Custom Hooks - Hospital Data Management**

```
Create a custom React hook for managing hospital system data with automatic
refresh and state management, including special handling for flagged patients.

Create src/hooks/useHospitalData.ts with:

1. A hook that:
   - Fetches all patients on mount
   - Fetches flagged patients separately
   - Provides loading, error, and data states for both
   - Auto-refreshes every 5 seconds (configurable via VITE_POLL_INTERVAL)
   - Provides CRUD operations:
     * createPatient(data: CreatePatient)
     * updatePatient(id: number, data: UpdatePatient)
     * deletePatient(id: number)
   - Automatically refreshes data after mutations
   - Handles errors gracefully with user-friendly messages

2. Return type should include:
   - patients: Patient[]
   - flaggedPatients: Patient[]
   - loading: boolean
   - flaggedLoading: boolean
   - error: string | null
   - refetch: () => Promise<void>
   - refetchFlagged: () => Promise<void>
   - createPatient: (data: CreatePatient) => Promise<void>
   - updatePatient: (id: number, data: UpdatePatient) => Promise<void>
   - deletePatient: (id: number) => Promise<void>

3. Use useEffect for polling with cleanup
4. Use useState for state management
5. Use useCallback for memoizing functions
6. Include TypeScript types for all parameters and return values
7. Add comments explaining why flagged patients are tracked separately

The hook should detect flag synchronization by comparing flagged count changes.
```

---

## **Prompt 8: Custom Hook - Cross-System Queries**

```
Create a custom React hook for handling cross-system data queries (police
checking hospital records and vice versa).

Create src/hooks/useCrossSystemQuery.ts with:

1. A hook that provides:
   - policeQueryPatient(personalId: string): Promise<Patient | null>
   - hospitalQuerySuspect(personalId: string): Promise<Suspect | null>
   - Both functions should handle 404 responses as valid "not found" results
   - Loading states for each query type
   - Error handling for network/server errors

2. Return type should include:
   - queryPatientFromPolice: (personalId: string) => Promise<Patient | null>
   - querySuspectFromHospital: (personalId: string) => Promise<Suspect | null>
   - policeQueryLoading: boolean
   - hospitalQueryLoading: boolean
   - policeQueryError: string | null
   - hospitalQueryError: string | null
   - clearErrors: () => void

3. State management for:
   - Query results (cached)
   - Loading states
   - Error states

4. Helper function to clear cached results
5. TypeScript types for all parameters and return values
6. JSDoc comments explaining when to use each query function

This hook enables the main cross-system functionality of the application.
```

---

## **Prompt 9: Utility Functions - Validation and Formatting**

```
Create utility functions for Swedish personal ID validation and data formatting.

Create two files:

1. src/utils/validation.ts with:
   - validateSwedishPersonalId(personalId: string): boolean
     * Validates format: YYYYMMDD-XXXX
     * Checks date part is 8 digits
     * Checks suffix is 4 digits
     * Checks separator is hyphen
     * Returns true if valid, false otherwise

   - formatPersonalIdInput(input: string): string
     * Auto-formats as user types
     * Adds hyphen after 8 digits
     * Limits to 13 characters

   - getValidationError(personalId: string): string | null
     * Returns specific error message if invalid
     * Returns null if valid

2. src/utils/formatting.ts with:
   - formatFullName(fullName: string | null): string
     * Handles null values gracefully
     * Capitalizes properly

   - formatFlagStatus(flag: boolean | null): string
     * Returns "Flagged" or "Not Flagged" or "Unknown"

   - formatDate(dateString: string): string
     * Formats ISO dates to readable format

   - truncateText(text: string, maxLength: number): string
     * Truncates long text with ellipsis

Include comprehensive JSDoc comments and unit test examples in comments.
Export all functions as named exports.
```

---

## **Prompt 10: Layout Components - Header and Footer**

```
Create professional layout components for the application header and footer
using Tailwind CSS.

Create src/components/layout/Header.tsx with:
1. A responsive header component that displays:
   - Application title: "Police & Hospital Data Sharing System"
   - System status indicators (green dots) for both backends
   - Current time/date
   - A toggle to switch between "Police View" and "Hospital View" (for future routing)

2. Styling:
   - Gradient background (blue-to-indigo)
   - White text with proper contrast
   - Responsive padding and layout
   - Shadow for depth
   - Icons for status indicators

3. Props:
   - policeSystemOnline: boolean
   - hospitalSystemOnline: boolean
   - currentView?: 'police' | 'hospital' | 'both'

Create src/components/layout/Footer.tsx with:
1. A simple footer component that displays:
   - "Thesis Project 2025"
   - Brief disclaimer about demonstration purpose
   - Links to documentation (placeholder)

2. Styling:
   - Light gray background
   - Centered text
   - Proper spacing
   - Border-top

Both components should use TypeScript with proper prop typing and include
JSDoc comments.
```

---

## **Prompt 11: Layout Component - System Panel Container**

```
Create a reusable container component for displaying each system's interface
in a consistent, professional layout.

Create src/components/layout/SystemPanel.tsx with:

1. A flexible panel component that:
   - Takes a title prop (e.g., "Police System" or "Hospital System")
   - Accepts children components to render inside
   - Has a colored border based on system type (blue for police, red for hospital)
   - Displays a system icon (🚔 for police, 🏥 for hospital)
   - Shows loading state with spinner
   - Shows error state with error message
   - Has a refresh button in the header

2. Props interface:
   - title: string
   - systemType: 'police' | 'hospital'
   - loading?: boolean
   - error?: string | null
   - onRefresh?: () => void
   - children: React.ReactNode

3. Styling with Tailwind:
   - Card-like appearance with shadow
   - Responsive padding
   - Smooth transitions
   - Color-coded borders (blue-600 for police, red-600 for hospital)
   - Header section with title and controls
   - Main content area
   - Footer with action buttons

4. Include:
   - TypeScript prop types
   - JSDoc comments
   - Proper accessibility attributes
   - Loading spinner component inline
   - Error display component inline

This component will wrap all police and hospital views for consistency.
```

---

## **Prompt 12: Police Components - Suspect List**

```
Create a comprehensive component for displaying the list of suspects in the
police system.

Create src/components/police/SuspectList.tsx with:

1. A data table component that displays:
   - All suspects from the police database
   - Columns: ID, Full Name, Personal ID, Flag Status, Actions
   - Flag status with visual indicator (red badge if flagged)
   - Action buttons: View, Edit, Delete, Flag/Unflag
   - Empty state when no suspects exist
   - Sorting capability (by name, ID, flag status)
   - Search/filter by name or personal ID

2. Props:
   - suspects: Suspect[]
   - loading: boolean
   - onEdit: (suspect: Suspect) => void
   - onDelete: (id: number) => void
   - onToggleFlag: (personalId: string, currentFlag: boolean) => void

3. Features:
   - Responsive table that collapses to cards on mobile
   - Visual feedback for actions (loading spinners on buttons)
   - Confirmation dialog before deletion
   - Flag toggle with immediate visual feedback
   - Alternating row colors for readability
   - Hover effects on rows

4. Styling:
   - Tailwind classes for clean table design
   - Blue accent colors matching police theme
   - Icons for actions (edit, delete, flag)
   - Badge component for flag status

5. Include:
   - TypeScript types
   - JSDoc comments
   - Accessibility features (ARIA labels)
   - Empty state illustration/message

Use React hooks for local state (search, sort) and Tailwind for all styling.
```

---

## **Prompt 13: Police Components - Suspect Form**

```
Create a form component for creating and editing suspects in the police system.

Create src/components/police/SuspectForm.tsx with:

1. A form component that:
   - Works for both creating new suspects and editing existing ones
   - Has fields for: Full Name, Personal ID
   - Validates Swedish personal ID format in real-time
   - Shows validation errors inline
   - Has a checkbox for initial flag status (only for creation)
   - Disables flag checkbox when editing (flags should be changed via dedicated control)
   - Formats personal ID input automatically (adds hyphen)

2. Props:
   - mode: 'create' | 'edit'
   - initialData?: Suspect (for edit mode)
   - onSubmit: (data: CreateSuspect | UpdateSuspect) => Promise<void>
   - onCancel: () => void

3. Features:
   - Real-time validation using the validateSwedishPersonalId utility
   - Disabled state while submitting
   - Clear error messages for each field
   - Auto-focus on first field
   - Keyboard navigation support (Tab, Enter)
   - Cancel button with confirmation if form is dirty

4. Styling:
   - Clean form layout with proper spacing
   - Blue primary buttons matching police theme
   - Red validation error messages
   - Input focus states with blue ring
   - Responsive layout (stacked on mobile)

5. Validation rules:
   - Full name: required, min 2 characters
   - Personal ID: required, must match Swedish format
   - Show specific error for each validation failure

6. Include:
   - TypeScript types
   - JSDoc comments
   - Form state management with useState
   - Error handling for API failures
   - Success callback on successful submission

Use controlled inputs and Tailwind CSS for styling.
```

---

## **Prompt 14: Police Components - Flag Control Panel**

```
Create a dedicated component for flagging/unflagging suspects with clear visual
feedback about synchronization to the hospital system.

Create src/components/police/FlagControl.tsx with:

1. A control panel component that:
   - Displays current flag status prominently
   - Shows a toggle switch or button to change flag
   - Explains that changes will auto-sync to hospital
   - Shows sync status (syncing, synced, error)
   - Displays when the flag was last updated
   - Has confirmation for flagging actions

2. Props:
   - suspect: Suspect
   - onToggleFlag: (personalId: string, newFlag: boolean) => Promise<void>
   - syncStatus?: 'idle' | 'syncing' | 'synced' | 'error'

3. Features:
   - Large, clear toggle switch for flag status
   - Visual indicator during sync (spinning icon)
   - Success message when synced (green checkmark)
   - Error message if sync fails (red X)
   - Information tooltip explaining what flagging means
   - Confirmation modal for flagging: "This will immediately notify the hospital system. Continue?"

4. Visual states:
   - Not flagged: Gray/neutral state
   - Flagged: Red/warning state
   - Syncing: Blue/loading state with spinner
   - Synced: Green/success state with checkmark

5. Styling:
   - Card layout with blue border
   - Large, prominent toggle switch
   - Status badges with colors
   - Icons for each state
   - Responsive padding and spacing

6. Include:
   - TypeScript types
   - JSDoc comments
   - Accessibility (ARIA labels, keyboard support)
   - Animation for state transitions
   - Clear messaging about cross-system impact

This component emphasizes the core functionality of the system: police flagging
that automatically syncs to hospital.
```

---

## **Prompt 15: Hospital Components - Patient List**

```
Create a comprehensive component for displaying the list of patients in the
hospital system, with special emphasis on flagged status.

Create src/components/hospital/PatientList.tsx with:

1. A data table component that displays:
   - All patients from the hospital database
   - Columns: ID, Full Name, Personal ID, Flag Status, Actions
   - Flag status with prominent visual indicator (red badge if flagged)
   - Note: "(Flagged by Police)" for flagged patients
   - Action buttons: View, Edit, Delete
   - Empty state when no patients exist
   - Sorting capability (by name, ID, flag status)
   - Search/filter by name or personal ID
   - Quick filter: "Show only flagged patients"

2. Props:
   - patients: Patient[]
   - loading: boolean
   - onEdit: (patient: Patient) => void
   - onDelete: (id: number) => void

3. Features:
   - Responsive table that collapses to cards on mobile
   - Visual feedback for actions (loading spinners on buttons)
   - Confirmation dialog before deletion
   - Flagged patients have red background highlight
   - Alternating row colors for readability
   - Hover effects on rows
   - Export functionality (CSV) as bonus feature

4. Styling:
   - Tailwind classes for clean table design
   - Red accent colors matching hospital theme
   - Icons for actions (edit, delete, view)
   - Badge component for flag status
   - Special styling for flagged rows (light red background)

5. Include:
   - TypeScript types
   - JSDoc comments
   - Accessibility features (ARIA labels)
   - Empty state illustration/message
   - Loading skeleton while fetching data

Use React hooks for local state (search, sort, filter) and Tailwind for styling.
```

---

## **Prompt 16: Hospital Components - Patient Form**

```
Create a form component for registering and editing patients in the hospital system.

Create src/components/hospital/PatientForm.tsx with:

1. A form component that:
   - Works for both creating new patients and editing existing ones
   - Has fields for: Full Name, Personal ID
   - Validates Swedish personal ID format in real-time
   - Shows validation errors inline
   - Does NOT allow editing flag status (read-only, managed by police)
   - Shows current flag status as read-only badge
   - Formats personal ID input automatically (adds hyphen)

2. Props:
   - mode: 'create' | 'edit'
   - initialData?: Patient (for edit mode)
   - onSubmit: (data: CreatePatient | UpdatePatient) => Promise<void>
   - onCancel: () => void

3. Features:
   - Real-time validation using the validateSwedishPersonalId utility
   - Disabled state while submitting
   - Clear error messages for each field
   - Auto-focus on first field
   - Keyboard navigation support (Tab, Enter)
   - Cancel button with confirmation if form is dirty
   - Warning message: "Note: Flag status is managed by police system"

4. Styling:
   - Clean form layout with proper spacing
   - Red primary buttons matching hospital theme
   - Red validation error messages
   - Input focus states with red ring
   - Responsive layout (stacked on mobile)
   - Read-only flag badge (not editable)

5. Validation rules:
   - Full name: required, min 2 characters
   - Personal ID: required, must match Swedish format
   - Show specific error for each validation failure

6. Include:
   - TypeScript types
   - JSDoc comments
   - Form state management with useState
   - Error handling for API failures
   - Success callback on successful submission

Use controlled inputs and Tailwind CSS for styling.
```

---

## **Prompt 17: Hospital Components - Flagged Patients View**

```
Create a dedicated component for displaying and monitoring patients that have
been flagged by the police system.

Create src/components/hospital/FlaggedPatients.tsx with:

1. A specialized view component that:
   - Shows only flagged patients
   - Displays count of flagged patients prominently
   - Shows when each patient was flagged (if available)
   - Explains that flags come from police system
   - Auto-refreshes more frequently (every 3 seconds)
   - Shows real-time sync indicator

2. Props:
   - flaggedPatients: Patient[]
   - loading: boolean
   - lastSync?: Date
   - onRefresh: () => void

3. Features:
   - List/grid view of flagged patients
   - Each card shows:
     * Patient name and personal ID
     * Large "FLAGGED" badge
     * Disclaimer: "Flagged by police system"
     * Button to view full patient details
   - Empty state: "No flagged patients at this time"
   - Sync indicator showing last update time
   - Manual refresh button

4. Visual design:
   - Alert/warning aesthetic (red/orange theme)
   - Card-based layout
   - Large, clear typography
   - Warning icons
   - Pulsing animation for new flags (optional)

5. Styling:
   - Red border on cards
   - Light red background
   - Warning badges
   - Icons (alert triangle)
   - Responsive grid (1 col mobile, 2 cols tablet, 3 cols desktop)

6. Include:
   - TypeScript types
   - JSDoc comments
   - Auto-refresh logic with useEffect
   - Accessibility features
   - Loading states
   - Count badge in header

This component highlights the automatic synchronization feature - the core
demonstration of the thesis project.
```

---

## **Prompt 18: Shared Components - Cross-System Query Interface**

```
Create an interactive component that allows users to query data across systems
(police checking hospital, hospital checking police).

Create src/components/shared/CrossSystemQuery.tsx with:

1. A query interface component that:
   - Has a selector for query direction: "Police → Hospital" or "Hospital → Police"
   - Has an input field for Swedish personal ID
   - Shows validation for personal ID format
   - Has a "Check" button to execute query
   - Displays query results in a card
   - Shows loading state during query
   - Handles "not found" gracefully (not an error, just no record)

2. Props:
   - systemContext: 'police' | 'hospital'
   - onQuery: (personalId: string, direction: 'police-to-hospital' | 'hospital-to-police') => Promise<any>

3. Features:
   - Toggle for query direction
   - Input with real-time validation
   - Submit button (disabled if invalid ID)
   - Results display:
     * If found: Show name, personal ID, flag status
     * If not found: "No record found in [system]"
     * Visual distinction between found/not found
   - Query history (last 3 queries) shown below
   - Clear button to reset form

4. Results card shows:
   - System icon (🚔 or 🏥)
   - "Record found" or "No record" heading
   - Person details if found
   - Timestamp of query

5. Styling:
   - Clean, modern form design
   - Color-coded based on query direction
   - Results card with appropriate system colors
   - Icons for systems
   - Transitions for results appearing
   - Responsive layout

6. Include:
   - TypeScript types
   - JSDoc comments
   - Form state management
   - Query history state
   - Error handling
   - Accessibility features

This component demonstrates the inter-system communication capability.
```

---

## **Prompt 19: Shared Components - Person Card and Sync Indicator**

```
Create two small, reusable shared components for consistent display across the app.

Create src/components/shared/PersonCard.tsx with:

1. A card component for displaying person information consistently:
   - Shows: Full Name, Personal ID, Flag Status
   - Works for both Suspect and Patient types
   - Has variants for different contexts (list view, detail view)
   - Includes system indicator (🚔 or 🏥)
   - Shows flag with appropriate styling

2. Props:
   - person: Suspect | Patient
   - systemType: 'police' | 'hospital'
   - variant?: 'compact' | 'full'
   - onClick?: () => void

3. Styling:
   - Card layout with hover effects
   - Color-coded border based on system
   - Flag badge (red if flagged)
   - Clean typography
   - Responsive padding

Create src/components/shared/SyncIndicator.tsx with:

1. A status indicator showing synchronization state:
   - Shows current sync status with icon
   - Shows last sync time
   - Animates during sync
   - Shows error state if sync fails

2. Props:
   - syncStatus: 'idle' | 'syncing' | 'synced' | 'error'
   - lastSyncTime?: Date
   - errorMessage?: string

3. Visual states:
   - Idle: Gray dot
   - Syncing: Blue spinning dot
   - Synced: Green dot with checkmark
   - Error: Red dot with X

4. Features:
   - Tooltip with more details
   - Relative time display ("2 seconds ago")
   - Pulsing animation when syncing

5. Styling:
   - Inline display
   - Status dot with animation
   - Small text for timestamp
   - Color-coded states

Include TypeScript types, JSDoc comments, and proper styling for both components.
```

---

## **Prompt 20: Main App Component and Routing**

```
Create the main App component that brings everything together with routing
and layout structure.

Create src/App.tsx with:

1. A main application component that:
   - Uses React Router for navigation (install react-router-dom if needed)
   - Has three main views:
     * Home/Dashboard (both systems side by side)
     * Police System View (full width)
     * Hospital System View (full width)
   - Includes the Header and Footer layout components
   - Provides application-level state management
   - Handles backend health checks
   - Shows connection status for both systems

2. Routes:
   - / - Dashboard view (both systems)
   - /police - Police system full view
   - /hospital - Hospital system full view

3. Features:
   - Health check for both backends on mount (every 30 seconds)
   - Pass connection status to Header
   - Error boundaries for crash protection
   - Loading state on initial load
   - Responsive layout (2-column on desktop, stacked on mobile)

4. State management:
   - Use the custom hooks (usePoliceData, useHospitalData)
   - Manage view state (which system is active)
   - Track backend health status

5. Layout structure:
```

   <div className="app">
     <Header policeOnline={} hospitalOnline={} />
     <main>
       <Routes>
         <Route path="/" element={<Dashboard />} />
         <Route path="/police" element={<PoliceView />} />
         <Route path="/hospital" element={<HospitalView />} />
       </Routes>
     </main>
     <Footer />
   </div>
   ```

6. Create three view components in the same file or separate files:

   - Dashboard: Shows both systems side by side
   - PoliceView: Full police interface
   - HospitalView: Full hospital interface

7. Include:
   - TypeScript types
   - JSDoc comments
   - Error boundaries
   - Loading states
   - Responsive design

Use React Router DOM and all previously created components.

```

---

## **Prompt 21: Dashboard View Component**

```

Create a comprehensive dashboard component that displays both police and
hospital systems side by side for comparison and monitoring.

Create src/views/Dashboard.tsx with:

1. A dashboard layout component that:

   - Shows both systems in a 2-column grid (1 column on mobile)
   - Displays key metrics for each system at the top:
     - Total suspects / patients
     - Number of flagged suspects / patients
     - Last sync time
   - Shows a cross-system query interface in the middle
   - Has tabs or sections for:
     - Overview (metrics and stats)
     - Recent Activity (recent creates, updates, flags)
     - Cross-System Queries
     - Flagged Records (synchronized view)

2. Use previously created components:

   - SystemPanel for each system container
   - SuspectList (summary view, top 5)
   - PatientList (summary view, top 5)
   - FlaggedPatients (combined view)
   - CrossSystemQuery
   - SyncIndicator

3. Features:

   - Real-time updates (using polling from hooks)
   - Visual comparison of flag counts
   - Highlight when new flag syncs occur
   - Quick actions (create suspect/patient, run queries)
   - Navigation links to full system views

4. Metrics cards show:

   - Icon (🚔 or 🏥)
   - Label ("Total Suspects" / "Total Patients")
   - Count (large number)
   - Flagged count (smaller, below)
   - Change indicator (if applicable)

5. Layout:

   - Top: Metrics row (4 cards)
   - Middle: Two columns (Police | Hospital)
   - Bottom: Cross-system query interface
   - Responsive: Stack on mobile

6. Styling:

   - Clean, modern dashboard aesthetic
   - Card-based layout
   - Color-coded sections (blue for police, red for hospital)
   - Proper spacing and alignment
   - Shadows and depth

7. Include:
   - TypeScript types
   - JSDoc comments
   - Use hooks for data fetching
   - Loading states for each section
   - Error handling

This is the main demo interface showcasing the thesis project's core features.

```

---

## **Prompt 22: Police View Component**

```

Create a full-featured police system interface that includes all police
functionality in one comprehensive view.

Create src/views/PoliceView.tsx with:

1. A complete police interface component that:

   - Shows all suspects in a full table
   - Has a "Create New Suspect" button that opens a modal/form
   - Includes the FlagControl component prominently
   - Has a section for querying hospital records
   - Shows recent flag changes
   - Displays sync status with hospital

2. Layout sections:

   - Header: Title "Police System" with create button
   - Stats bar: Total suspects, flagged count, pending syncs
   - Main content: Suspect table with all features
   - Sidebar: Quick actions and hospital queries
   - Modal: SuspectForm for create/edit operations

3. Features:

   - Create suspect (opens modal with SuspectForm)
   - Edit suspect (opens modal with prefilled form)
   - Delete suspect (with confirmation)
   - Flag/unflag suspects (with FlagControl)
   - Query hospital records (CrossSystemQuery component)
   - Filter suspects by flag status
   - Search suspects by name or personal ID
   - Pagination if more than 20 suspects

4. Modal management:

   - Create modal state
   - Edit modal state (with selected suspect)
   - Close modals with ESC key
   - Backdrop click to close (with confirmation if dirty)

5. Use components:

   - SuspectList (full version)
   - SuspectForm (in modal)
   - FlagControl (in modal or side panel)
   - CrossSystemQuery (in sidebar)
   - SystemPanel wrapper

6. Data flow:

   - Use usePoliceData hook
   - Use useCrossSystemQuery hook
   - Pass callback functions to child components
   - Handle success/error notifications

7. Styling:

   - Full-width layout with sidebar
   - Blue theme throughout
   - Responsive: Stack sidebar below on mobile
   - Modal overlay with fade animation
   - Professional police system aesthetic

8. Include:
   - TypeScript types
   - JSDoc comments
   - Error handling
   - Loading states
   - Success notifications (toast or inline)
   - Accessibility features

This view demonstrates all police-specific functionality.

```

---

## **Prompt 23: Hospital View Component**

```

Create a full-featured hospital system interface that includes all hospital
functionality in one comprehensive view, with emphasis on flagged patients.

Create src/views/HospitalView.tsx with:

1. A complete hospital interface component that:

   - Shows all patients in a full table
   - Has a "Register New Patient" button that opens a modal/form
   - Has a dedicated "Flagged Patients" section (prominent)
   - Includes section for querying police records
   - Shows flag sync indicator
   - Displays notice that flags are managed by police

2. Layout sections:

   - Header: Title "Hospital System" with register button
   - Alert banner: "Flagged Patients" count (if any) - prominent, red
   - Stats bar: Total patients, flagged count, last sync time
   - Main content: Patient table with all features
   - Sidebar: Flagged patients list and police queries
   - Modal: PatientForm for create/edit operations

3. Features:

   - Register patient (opens modal with PatientForm)
   - Edit patient (opens modal with prefilled form, flag read-only)
   - Delete patient (with confirmation)
   - View flagged patients (separate prominent section)
   - Query police records (CrossSystemQuery component)
   - Filter patients by flag status
   - Search patients by name or personal ID
   - Pagination if more than 20 patients

4. Flagged patients section:

   - Large heading: "⚠️ Flagged Patients"
   - Count badge
   - List of flagged patients with details
   - Note: "Flags are automatically synchronized from police system"
   - Updates in real-time

5. Modal management:

   - Create modal state
   - Edit modal state (with selected patient)
   - Close modals with ESC key
   - Backdrop click to close (with confirmation if dirty)

6. Use components:

   - PatientList (full version)
   - PatientForm (in modal)
   - FlaggedPatients (in sidebar or main section)
   - CrossSystemQuery (in sidebar)
   - SystemPanel wrapper
   - SyncIndicator

7. Data flow:

   - Use useHospitalData hook
   - Use useCrossSystemQuery hook
   - Pass callback functions to child components
   - Handle success/error notifications

8. Styling:

   - Full-width layout with sidebar
   - Red theme throughout
   - Responsive: Stack sidebar below on mobile
   - Modal overlay with fade animation
   - Professional hospital system aesthetic
   - Flagged section has warning styling

9. Include:
   - TypeScript types
   - JSDoc comments
   - Error handling
   - Loading states
   - Success notifications (toast or inline)
   - Accessibility features

This view demonstrates all hospital-specific functionality with emphasis on
receiving and displaying flagged patients from the police system.

```

---

## **Prompt 24: Global Styles and Tailwind Configuration**

```

Create the global styles and finalize Tailwind configuration for a professional,
cohesive appearance across the application.

Create src/index.css with:

1. Tailwind directives:

   ```css
   @tailwind base;
   @tailwind components;
   @tailwind utilities;
   ```

2. Custom CSS for:
   - Base styles (body, html)
   - Custom component classes:
     - .btn-primary (police blue)
     - .btn-secondary (hospital red)
     - .btn-danger (delete actions)
     - .card (standard card appearance)
     - .badge (status badges)
     - .table-row (table styling)
     - .modal-backdrop (modal overlay)
3. Animation keyframes for:

   - Spinner (loading animation)
   - Pulse (sync indicator)
   - Fade-in (modal appear)
   - Slide-in (notifications)

4. Custom scrollbar styling (optional but professional)

5. Focus states for accessibility:
   - Blue focus ring for police forms
   - Red focus ring for hospital forms
   - Skip to content link

Update tailwind.config.js with:

1. Custom colors:

   - police: { light: '#BBDEFB', DEFAULT: '#1976D2', dark: '#0D47A1' }
   - hospital: { light: '#FFCDD2', DEFAULT: '#D32F2F', dark: '#B71C1C' }
   - success: '#4CAF50'
   - warning: '#FFC107'
   - error: '#F44336'

2. Custom fonts (if desired):

   - Sans: Inter or similar professional font
   - Mono: JetBrains Mono for code/IDs

3. Custom spacing values if needed

4. Custom breakpoints:

   - sm: 640px
   - md: 768px
   - lg: 1024px
   - xl: 1280px
   - 2xl: 1536px

5. Extend theme with:
   - Box shadows
   - Border radius options
   - Transition durations

Create src/App.css as well for any app-specific styles not in components.

Include comments explaining custom classes and when to use them.

```

---

## **Prompt 25: Entry Point and Final Integration**

```

Create the application entry point and ensure all pieces are integrated correctly.

Create src/main.tsx with:

1. React 18 render using createRoot:

   - Import React, ReactDOM
   - Import App component
   - Import global styles (index.css)
   - Import BrowserRouter from react-router-dom
   - Render App wrapped in BrowserRouter and React.StrictMode

2. Add development-only logging:
   - Log app version
   - Log environment variables (non-sensitive)
   - Log backend URLs being used

Create src/vite-env.d.ts with:

- TypeScript declarations for Vite environment variables
- /// <reference types="vite/client" />
- Interface for import.meta.env with custom variables

Create a README.md in frontend directory with:

1. Project overview
2. Installation instructions:

   ```bash
   npm install
   ```

3. Development server:

   ```bash
   npm run dev
   ```

4. Environment setup:

   - Copy .env.example to .env
   - Configure backend URLs

5. Building for production:

   ```bash
   npm run build
   ```

6. Project structure explanation

7. Key features demonstrated:

   - Flag synchronization
   - Cross-system queries
   - CRUD operations
   - Real-time updates

8. Architecture notes:

   - Component organization
   - State management approach
   - API communication layer

9. Development notes:

   - How to add new features
   - How to modify API endpoints
   - How to customize styling

10. Testing instructions (placeholder for now)

Also create a .gitignore for the frontend:

```
node_modules/
dist/
.env
.env.local
*.log
.DS_Store
```

Include all necessary configuration for a production-ready application.

```

---

## **Prompt 26: Error Handling and Loading States**

```

Create centralized error handling and loading state components to ensure
consistent UX throughout the application.

Create src/components/common/ErrorBoundary.tsx with:

1. A React Error Boundary class component that:

   - Catches JavaScript errors anywhere in the child component tree
   - Logs error details to the console
   - Displays a fallback UI with error message
   - Has a "Reload Page" button
   - Has different styles for different error severities
   - Shows contact/help information

2. Features:
   - componentDidCatch lifecycle
   - getDerivedStateFromError static method
   - State for error and errorInfo
   - Production vs development error display
   - Error boundary for each major section (police, hospital)

Create src/components/common/LoadingSpinner.tsx with:

1. A loading spinner component with:

   - Animated spinner (CSS or SVG)
   - Optional loading message
   - Different sizes (small, medium, large)
   - Different colors based on context (police blue, hospital red, neutral)

2. Props:
   - size?: 'sm' | 'md' | 'lg'
   - color?: 'police' | 'hospital' | 'neutral'
   - message?: string

Create src/components/common/EmptyState.tsx with:

1. An empty state component for when there's no data:

   - Icon or illustration
   - Heading ("No suspects found" / "No patients registered")
   - Description text
   - Call-to-action button (e.g., "Create First Suspect")
   - Different variants for different contexts

2. Props:
   - icon?: React.ReactNode
   - heading: string
   - description: string
   - actionLabel?: string
   - onAction?: () => void

Create src/components/common/ErrorMessage.tsx with:

1. An error message component for displaying errors inline:

   - Error icon
   - Error message text
   - Optional retry button
   - Optional dismiss button
   - Different severities (error, warning, info)

2. Props:
   - message: string
   - severity?: 'error' | 'warning' | 'info'
   - onRetry?: () => void
   - onDismiss?: () => void

Create src/components/common/Toast.tsx with:

1. A toast notification system for success/error messages:

   - Appears in top-right corner
   - Auto-dismisses after 3 seconds
   - Different colors for success/error/info
   - Slide-in animation
   - Stack multiple toasts
   - Click to dismiss

2. Also create src/hooks/useToast.ts:
   - Custom hook for showing toasts
   - toast.success(message)
   - toast.error(message)
   - toast.info(message)
   - Manages toast queue

All components should use TypeScript, include JSDoc comments, and use Tailwind
CSS for styling. These components will be used throughout the app for consistent
error handling and loading states.

```

---

## **Prompt 27: Testing Setup and Basic Tests**

```

Set up a basic testing infrastructure for the frontend and create initial tests
for critical functionality.

1. Install testing dependencies:

   ```bash
   npm install --save-dev @testing-library/react @testing-library/jest-dom
   @testing-library/user-event vitest jsdom
   ```

2. Create vitest.config.ts with:

   - Configuration for React testing
   - JSDOM environment
   - Test file patterns
   - Coverage settings

3. Update package.json with test scripts:

   ```json
   "scripts": {
     "test": "vitest",
     "test:ui": "vitest --ui",
     "coverage": "vitest run --coverage"
   }
   ```

4. Create src/tests/utils/validation.test.ts with tests for:

   - validateSwedishPersonalId function
   - Test valid formats
   - Test invalid formats
   - Edge cases

5. Create src/tests/components/PersonCard.test.tsx with:

   - Rendering tests
   - Props handling
   - Flag display
   - Click handlers

6. Create src/tests/hooks/usePoliceData.test.ts with:

   - Data fetching tests
   - CRUD operation tests
   - Error handling tests
   - Loading states

7. Create src/tests/services/policeApi.test.ts with:

   - API call tests (mocked)
   - Error handling
   - Response parsing

8. Create a test utilities file src/tests/utils/testUtils.tsx with:

   - Custom render function with providers
   - Mock data factories
   - Common test helpers

9. Add a README in src/tests/ explaining:
   - How to run tests
   - How to write new tests
   - Testing patterns used
   - Coverage goals

For each test file, include:

- TypeScript types
- Proper imports
- Describe blocks for organization
- Multiple test cases
- Assertions using @testing-library
- Mock data as needed

Focus on testing critical paths:

- Data fetching and display
- Form validation
- User interactions
- Error scenarios
- API integration

This provides a foundation for comprehensive testing.

```

---

## **Prompt 28: Documentation and Developer Guide**

```

Create comprehensive documentation for developers working on the frontend.

Create docs/FRONTEND_GUIDE.md with:

1. Overview section:

   - Project architecture summary
   - Technology stack
   - Key dependencies
   - Design decisions

2. Getting Started section:

   - Prerequisites
   - Installation steps
   - Environment configuration
   - Running development server
   - Running tests

3. Project Structure section:

   - Detailed explanation of folder structure
   - Purpose of each directory
   - Naming conventions
   - File organization patterns

4. Component Guide section:

   - How components are organized
   - Component naming conventions
   - Props patterns
   - Styling approach
   - When to create new components

5. State Management section:

   - How state is managed (Context + Hooks)
   - When to use local vs global state
   - Data flow patterns
   - Custom hooks explanation

6. API Integration section:

   - How API services work
   - Adding new endpoints
   - Error handling patterns
   - Loading state management

7. Styling Guide section:

   - Tailwind usage patterns
   - Custom classes
   - Color scheme
   - Responsive design approach
   - Accessibility considerations

8. Adding New Features section:

   - Step-by-step process
   - Checklist for new features
   - Code review guidelines
   - Testing requirements

9. Common Patterns section:

   - Form handling
   - List/table rendering
   - Modal management
   - Error handling
   - Loading states

10. Troubleshooting section:

    - Common issues and solutions
    - Backend connection problems
    - CORS issues
    - Build errors

11. Performance Considerations:

    - Optimization techniques used
    - When to optimize
    - Profiling tools

12. Future Enhancements:
    - Planned features
    - Known limitations
    - Improvement opportunities

Also create docs/API_INTEGRATION.md with:

1. Complete API reference for frontend developers:

   - All endpoints used
   - Request/response formats
   - Error codes
   - Examples

2. How to add new API endpoints:

   - Update types
   - Update service file
   - Create/update hook
   - Update components

3. Mock data for development:
   - Using mock data during development
   - Mock API setup (optional)

Include code examples, diagrams (as text/ASCII), and clear explanations
throughout both documentation files.

```

---

## **Prompt 29: Performance Optimization and Final Polish**

```

Add performance optimizations and final polish to the frontend application.

1. Create src/hooks/useDebounce.ts:

   - Custom hook for debouncing inputs
   - Use for search/filter inputs
   - Configurable delay

2. Create src/hooks/useLocalStorage.ts:

   - Custom hook for persisting state to localStorage
   - Use for user preferences (theme, view mode, etc.)
   - Type-safe implementation

3. Optimize components for performance:

   - Add React.memo to expensive components:
     - PersonCard
     - SuspectList
     - PatientList
   - Use useCallback for event handlers in:
     - Forms
     - Lists
     - Modal components
   - Use useMemo for computed values:
     - Filtered/sorted lists
     - Derived statistics

4. Create src/components/common/VirtualizedList.tsx (optional):

   - Virtual scrolling for large lists
   - Use if suspect/patient lists grow large
   - Performance benefit for 100+ items

5. Add code splitting:

   - Use React.lazy for route components
   - Suspense boundaries with loading fallbacks
   - Lazy load heavy components (modals, forms)

6. Create src/utils/performance.ts with:

   - Performance monitoring utilities
   - Function to measure render times
   - Function to track API call duration
   - Console logs for development

7. Image optimization:

   - Add loading="lazy" to images
   - Use appropriate image formats
   - Optimize icon sizes

8. Add accessibility improvements:

   - ARIA labels on all interactive elements
   - Keyboard navigation for modals
   - Focus management
   - Screen reader announcements for dynamic content

9. Create src/components/common/SkipToContent.tsx:

   - Skip to main content link for keyboard users
   - Hidden by default, visible on focus

10. Final polish items:

    - Smooth transitions between routes
    - Hover states on all interactive elements
    - Consistent spacing throughout
    - Loading skeletons instead of spinners
    - Optimistic UI updates where appropriate

11. Create src/utils/analytics.ts (placeholder):

    - Functions for tracking user interactions
    - Page view tracking
    - Event tracking
    - Placeholder implementation (console.log for now)

12. Add a "Development mode" indicator:
    - Small badge in bottom-left corner
    - Only visible in development
    - Shows current environment

For each optimization, add comments explaining:

- Why it's needed
- Performance impact
- Trade-offs

Update components to use these optimizations where appropriate.

```

---

## **Prompt 30: Build and Deployment Configuration**

```

Create production build configuration and deployment documentation.

1. Update vite.config.ts for production:

   - Production build optimizations
   - Source map configuration
   - Chunk splitting strategy
   - Asset optimization
   - Environment-specific configuration

2. Create .env.production:

   - Production environment variables
   - Production API URLs (placeholders)
   - Feature flags

3. Create scripts/build.sh:

   - Pre-build checks (linting, type checking)
   - Build command
   - Post-build verification
   - Build size reporting

4. Create scripts/preview.sh:

   - Script to preview production build locally
   - Uses vite preview

5. Create docs/DEPLOYMENT.md with:

   - Build process explanation
   - Environment variable configuration for different environments
   - Deployment options:
     - Static hosting (Netlify, Vercel)
     - Docker container
     - Traditional web server (nginx)
   - CORS configuration for production
   - Security considerations
   - Performance checklist

6. Create Dockerfile (optional):

   - Multi-stage build
   - Production-optimized image
   - nginx configuration for serving React app
   - Health check endpoint

7. Create docker-compose.yml (optional):

   - Frontend service
   - Integration with backend services
   - Network configuration

8. Create .dockerignore:

   - node_modules
   - dist
   - .env files
   - Development files

9. Create nginx.conf (if using nginx):

   - React Router support (redirect all to index.html)
   - Gzip compression
   - Cache headers for static assets
   - Security headers

10. Update package.json scripts:

    ```json
    "scripts": {
      "dev": "vite",
      "build": "tsc && vite build",
      "preview": "vite preview",
      "lint": "eslint src --ext ts,tsx",
      "type-check": "tsc --noEmit",
      "format": "prettier --write src/**/*.{ts,tsx,css}"
    }
    ```

11. Create docs/PRODUCTION_CHECKLIST.md:

    - Pre-deployment checklist
    - Environment verification
    - Security review items
    - Performance review items
    - Testing requirements
    - Rollback procedures

12. Create a CI/CD configuration example:
    - GitHub Actions workflow (.github/workflows/frontend.yml)
    - Automated build and test
    - Deployment to staging/production
    - Include comments for customization

Include all necessary configuration files and detailed deployment instructions
for various platforms. Ensure everything is production-ready.

```

---

## Summary

These 30 prompts provide a complete, systematic approach to building the frontend. Each prompt:

1. **Builds incrementally** on previous work
2. **References the existing backend** APIs and architecture
3. **Includes specific requirements** for TypeScript types, component props, and styling
4. **Emphasizes the core thesis features**: flag synchronization and cross-system queries
5. **Follows the architecture** outlined in ARCHITECTURE.md
6. **Provides professional quality** code with proper error handling, accessibility, and documentation

The sequence takes a developer from project setup through to production deployment, ensuring a complete, cohesive frontend that properly demonstrates the thesis project's capabilities.
```
