# Frontend Code Generation Prompts

A comprehensive series of AI prompts to generate the complete frontend for the thesis project. These prompts build upon the existing backend infrastructure.

## Phase 1: Shared Type Definitions & API Services

### Prompt 1.1: Police System - Type Definitions

```
Create TypeScript type definitions for the Police System frontend.

File: frontend/police-ui/src/types/suspect.ts

Requirements:
- Define a Suspect interface matching the backend model:
  - id: number
  - full_name: string | null
  - personal_id: string | null
  - flag: boolean | null

- Define a CreateSuspect interface for POST requests (without id):
  - full_name: string
  - personal_id: string
  - flag: boolean

- Define an UpdateSuspect interface for PUT requests:
  - personal_id: string
  - full_name?: string
  - flag?: boolean

- Export all interfaces
- Add JSDoc comments explaining Swedish personal_id format (YYYYMMDD-XXXX)
```

### Prompt 1.2: Hospital System - Type Definitions

```
Create TypeScript type definitions for the Hospital System frontend.

File: frontend/hospital-ui/src/types/patient.ts

Requirements:
- Define a Patient interface matching the backend model:
  - id: number
  - full_name: string | null
  - personal_id: string | null
  - flag: boolean | null

- Define a CreatePatient interface for POST requests (without id):
  - full_name: string
  - personal_id: string
  - flag: boolean

- Define an UpdatePatient interface for PUT requests:
  - personal_id: string
  - full_name?: string
  - flag?: boolean

- Export all interfaces
- Add JSDoc comments explaining Swedish personal_id format (YYYYMMDD-XXXX)

Use the exact same structure as the Police System types for consistency.
```

### Prompt 1.3: Police System - API Configuration

```
Create the base API configuration file for the Police System.

File: frontend/police-ui/src/services/api.ts

Requirements:
- Define BASE_URL constant: 'http://localhost:8000'
- Create a helper function: handleResponse<T>(response: Response): Promise<T>
  - Check if response.ok
  - If not ok, parse error JSON and throw with message
  - If ok, return response.json()
  - Handle empty responses (204 No Content)

- Create a helper function: apiRequest<T>(endpoint: string, options?: RequestInit): Promise<T>
  - Prepend BASE_URL to endpoint
  - Set default headers: 'Content-Type': 'application/json'
  - Merge with provided options
  - Call fetch and use handleResponse
  - Include error handling with console.error

- Export both helpers and BASE_URL
- Use modern async/await syntax
```

### Prompt 1.4: Hospital System - API Configuration

```
Create the base API configuration file for the Hospital System.

File: frontend/hospital-ui/src/services/api.ts

Requirements:
- Define BASE_URL constant: 'http://localhost:8001'
- Create a helper function: handleResponse<T>(response: Response): Promise<T>
  - Check if response.ok
  - If not ok, parse error JSON and throw with message
  - If ok, return response.json()
  - Handle empty responses (204 No Content)

- Create a helper function: apiRequest<T>(endpoint: string, options?: RequestInit): Promise<T>
  - Prepend BASE_URL to endpoint
  - Set default headers: 'Content-Type': 'application/json'
  - Merge with provided options
  - Call fetch and use handleResponse
  - Include error handling with console.error

- Export both helpers and BASE_URL
- Use the exact same structure as the Police System for consistency
```

### Prompt 1.5: Police System - Suspects API Service

```
Create the API service for suspect operations in the Police System.

File: frontend/police-ui/src/services/suspects.ts

Requirements:
Import types from '../types/suspect' and apiRequest from './api'

Implement these async functions:

1. getAllSuspects(): Promise<Suspect[]>
   - GET /suspects

2. getSuspectById(id: number): Promise<Suspect>
   - GET /suspects/{id}

3. getSuspectByPersonalId(personalId: string): Promise<Suspect>
   - GET /suspects/personal/{personalId}

4. createSuspect(suspect: CreateSuspect): Promise<Suspect>
   - POST /suspects
   - Body: JSON.stringify(suspect)

5. updateSuspect(id: number, suspect: UpdateSuspect): Promise<Suspect>
   - PUT /suspects/{id}
   - Body: JSON.stringify(suspect)

6. deleteSuspect(id: number): Promise<void>
   - DELETE /suspects/{id}

7. updateFlag(personalId: string, flag: boolean): Promise<Suspect>
   - PUT /suspects/{personalId}/flag
   - Body: JSON.stringify({ flag })

- Use apiRequest helper for all requests
- Include proper HTTP methods and bodies
- Add error handling with meaningful messages
- Export all functions
```

### Prompt 1.6: Hospital System - Patients API Service

```
Create the API service for patient operations in the Hospital System.

File: frontend/hospital-ui/src/services/patients.ts

Requirements:
Import types from '../types/patient' and apiRequest from './api'

Implement these async functions:

1. getAllPatients(): Promise<Patient[]>
   - GET /patients

2. getPatientById(id: number): Promise<Patient>
   - GET /patients/{id}

3. getPatientByPersonalId(personalId: string): Promise<Patient>
   - GET /patients/personal/{personalId}

4. createPatient(patient: CreatePatient): Promise<Patient>
   - POST /patients
   - Body: JSON.stringify(patient)

5. updatePatient(id: number, patient: UpdatePatient): Promise<Patient>
   - PUT /patients/{id}
   - Body: JSON.stringify(patient)

6. deletePatient(id: number): Promise<void>
   - DELETE /patients/{id}

7. getFlaggedPatients(): Promise<Patient[]>
   - GET /patients/flagged

- Use apiRequest helper for all requests
- Include proper HTTP methods and bodies
- Add error handling with meaningful messages
- Export all functions
- Use the same pattern as the Police System service
```

### Prompt 1.7: Police System - Shared Data API Service

```
Create the API service for inter-system communication in the Police System.

File: frontend/police-ui/src/services/shared.ts

Requirements:
Import Patient type from '../types/patient' and apiRequest from './api'

Note: Define a simple Patient type inline if you don't have access to hospital types:
interface Patient {
  id: number;
  full_name: string | null;
  personal_id: string | null;
  flag: boolean | null;
}

Implement these async functions:

1. getPatientFromHospital(personalId: string): Promise<Patient>
   - GET to hospital system: http://localhost:8001/api/shared/patients/{personalId}
   - This queries the hospital database from the police UI

2. getAllPatientsFromHospital(): Promise<Patient[]>
   - GET to hospital system: http://localhost:8001/api/shared/patients
   - Returns all hospital patients

3. getFlaggedPatientsFromHospital(): Promise<Patient[]>
   - GET to hospital system: http://localhost:8001/api/shared/patients/flagged
   - Returns patients flagged by police

- Use fetch directly with full URLs (not apiRequest since it's cross-system)
- Include proper error handling
- Add comments explaining these are cross-system queries
- Export all functions
```

### Prompt 1.8: Hospital System - Shared Data API Service

```
Create the API service for inter-system communication in the Hospital System.

File: frontend/hospital-ui/src/services/shared.ts

Requirements:
Import Suspect type from '../types/suspect' (define inline if needed)

Define a simple Suspect type inline:
interface Suspect {
  id: number;
  full_name: string | null;
  personal_id: string | null;
  flag: boolean | null;
}

Implement these async functions:

1. getSuspectFromPolice(personalId: string): Promise<Suspect>
   - GET to police system: http://localhost:8000/api/shared/suspects/{personalId}
   - This queries the police database from the hospital UI

2. getAllSuspectsFromPolice(): Promise<Suspect[]>
   - GET to police system: http://localhost:8000/api/shared/suspects
   - Returns all police suspects

- Use fetch directly with full URLs (not apiRequest since it's cross-system)
- Include proper error handling
- Add comments explaining these are cross-system queries
- Export all functions
- Use the same pattern as the Police System shared service
```

---

## Phase 2: Reusable Components

### Prompt 2.1: Police System - Suspect List Component

```
Create a reusable component for displaying a list of suspects.

File: frontend/police-ui/src/components/SuspectList.tsx

Requirements:
- Props:
  - suspects: Suspect[]
  - onEdit: (suspect: Suspect) => void
  - onDelete: (id: number) => void
  - onToggleFlag: (personalId: string, currentFlag: boolean) => void

- Display suspects in a clean table with columns:
  - ID
  - Full Name
  - Personal ID
  - Flag Status (show badge: 🚩 if flagged)
  - Actions (Edit, Delete, Toggle Flag buttons)

- Styling:
  - Use inline styles or className for basic styling
  - Make flagged rows visually distinct (e.g., light red background)
  - Buttons should be clearly labeled
  - Table should be responsive

- Handle empty state: "No suspects found"
- Add hover effects on table rows
- Make it clean and professional looking
- Export as default
```

### Prompt 2.2: Police System - Suspect Form Component

```
Create a form component for creating/editing suspects.

File: frontend/police-ui/src/components/SuspectForm.tsx

Requirements:
- Props:
  - suspect?: Suspect (optional, for edit mode)
  - onSubmit: (data: CreateSuspect | UpdateSuspect) => void
  - onCancel: () => void

- Form fields:
  - Full Name (text input, required)
  - Personal ID (text input, required, pattern: YYYYMMDD-XXXX)
  - Flag (checkbox)

- Features:
  - If suspect prop provided, populate form (edit mode)
  - If no suspect prop, empty form (create mode)
  - Validate Swedish personal ID format
  - Show validation errors inline
  - Disable submit button while submitting

- Styling:
  - Clean, modern form layout
  - Labels above inputs
  - Proper spacing
  - Clear submit/cancel buttons

- Handle form submission with preventDefault
- Clear form after successful submission (create mode)
- Export as default
```

### Prompt 2.3: Hospital System - Patient List Component

```
Create a reusable component for displaying a list of patients.

File: frontend/hospital-ui/src/components/PatientList.tsx

Requirements:
- Props:
  - patients: Patient[]
  - onEdit: (patient: Patient) => void
  - onDelete: (id: number) => void
  - showPoliceCheck?: boolean (optional, default false)

- Display patients in a clean table with columns:
  - ID
  - Full Name
  - Personal ID
  - Flag Status (show badge: 🚩 if flagged, with note "Flagged by Police")
  - Actions (Edit, Delete buttons)
  - Optional: Check Police Records button (if showPoliceCheck is true)

- Styling:
  - Use inline styles or className for basic styling
  - Make flagged rows visually distinct (e.g., light yellow background)
  - Buttons should be clearly labeled
  - Table should be responsive

- Handle empty state: "No patients found"
- Add hover effects on table rows
- Make it clean and professional looking
- Export as default
- Use similar structure to Police SuspectList for consistency
```

### Prompt 2.4: Hospital System - Patient Form Component

```
Create a form component for creating/editing patients.

File: frontend/hospital-ui/src/components/PatientForm.tsx

Requirements:
- Props:
  - patient?: Patient (optional, for edit mode)
  - onSubmit: (data: CreatePatient | UpdatePatient) => void
  - onCancel: () => void

- Form fields:
  - Full Name (text input, required)
  - Personal ID (text input, required, pattern: YYYYMMDD-XXXX)
  - Flag (checkbox, disabled with note: "Managed by Police System")

- Features:
  - If patient prop provided, populate form (edit mode)
  - If no patient prop, empty form (create mode)
  - Validate Swedish personal ID format
  - Show validation errors inline
  - Disable submit button while submitting
  - Flag field should be disabled - explain it's auto-synced from police

- Styling:
  - Clean, modern form layout
  - Labels above inputs
  - Proper spacing
  - Clear submit/cancel buttons
  - Disabled flag field should be visually distinct

- Handle form submission with preventDefault
- Clear form after successful submission (create mode)
- Export as default
- Use similar structure to Police SuspectForm for consistency
```

### Prompt 2.5: Police System - Data Request Component

```
Create a component for requesting data from the hospital system.

File: frontend/police-ui/src/components/DataRequest.tsx

Requirements:
- No props needed (self-contained component)

- Features:
  - Input field for Personal ID
  - "Check Hospital Records" button
  - Display results in a card below input
  - Show loading state while fetching
  - Handle not found (404) gracefully: "No hospital records found"
  - Handle errors: "Error checking hospital records"

- Display hospital patient data if found:
  - Full Name
  - Personal ID
  - Flag Status
  - Message: "This person has hospital records"

- Use getPatientFromHospital from services/shared

- Styling:
  - Clean card layout
  - Input and button in a row
  - Results card appears below
  - Success: green border
  - Not found: blue border with info icon
  - Error: red border with error icon

- Add proper TypeScript types
- Export as default
```

### Prompt 2.6: Hospital System - Data Request Component

```
Create a component for requesting data from the police system.

File: frontend/hospital-ui/src/components/DataRequest.tsx

Requirements:
- No props needed (self-contained component)

- Features:
  - Input field for Personal ID
  - "Check Police Records" button
  - Display results in a card below input
  - Show loading state while fetching
  - Handle not found (404) gracefully: "No police records found"
  - Handle errors: "Error checking police records"

- Display police suspect data if found:
  - Full Name
  - Personal ID
  - Flag Status
  - Warning message if flagged: "⚠️ This person has active police records"

- Use getSuspectFromPolice from services/shared

- Styling:
  - Clean card layout
  - Input and button in a row
  - Results card appears below
  - Found & flagged: red/orange border with warning
  - Found & not flagged: yellow border with info
  - Not found: green border with checkmark
  - Error: red border with error icon

- Add proper TypeScript types
- Export as default
- Use similar structure to Police DataRequest
```

---

## Phase 3: Main Application Components

### Prompt 3.1: Police System - Main App Component

```
Create the main App component for the Police System.

File: frontend/police-ui/src/App.tsx

Requirements:
- State management:
  - suspects: Suspect[]
  - loading: boolean
  - error: string | null
  - editingSuspect: Suspect | null
  - showForm: boolean

- Fetch suspects on component mount using getAllSuspects()
- Implement handlers:
  - handleCreate(data: CreateSuspect)
  - handleUpdate(id: number, data: UpdateSuspect)
  - handleDelete(id: number)
  - handleToggleFlag(personalId: string, currentFlag: boolean)
  - handleEdit(suspect: Suspect) - opens form in edit mode
  - handleCancelEdit() - closes form

- Layout:
  - Header: "🚔 Police System"
  - "Add New Suspect" button (toggles form)
  - DataRequest component (for checking hospital records)
  - SuspectForm (conditional, shown when showForm is true)
  - SuspectList (always shown)
  - Loading state
  - Error state

- After any mutation (create/update/delete/flag), refetch suspects list
- Use existing App.css for styling
- Add toast/alert messages for success/error (simple window.alert for now)
- Import all necessary services and components
- Export as default
```

### Prompt 3.2: Hospital System - Main App Component

```
Create the main App component for the Hospital System.

File: frontend/hospital-ui/src/App.tsx

Requirements:
- State management:
  - patients: Patient[]
  - loading: boolean
  - error: string | null
  - editingPatient: Patient | null
  - showForm: boolean
  - showFlaggedOnly: boolean (toggle for filtering)

- Fetch patients on component mount using getAllPatients()
- Implement handlers:
  - handleCreate(data: CreatePatient)
  - handleUpdate(id: number, data: UpdatePatient)
  - handleDelete(id: number)
  - handleEdit(patient: Patient) - opens form in edit mode
  - handleCancelEdit() - closes form
  - handleToggleFlaggedFilter() - toggles showFlaggedOnly

- Layout:
  - Header: "🏥 Hospital System"
  - "Add New Patient" button (toggles form)
  - "Show Flagged Only" toggle switch
  - DataRequest component (for checking police records)
  - PatientForm (conditional, shown when showForm is true)
  - PatientList (filtered by showFlaggedOnly)
  - Loading state
  - Error state

- Filter displayed patients based on showFlaggedOnly flag
- After any mutation (create/update/delete), refetch patients list
- Use existing App.css for styling
- Add toast/alert messages for success/error (simple window.alert for now)
- Import all necessary services and components
- Export as default
- Use similar structure to Police App for consistency
```

---

## Phase 4: Styling Improvements

### Prompt 4.1: Police System - Enhanced CSS

```
Update the CSS for the Police System with modern, professional styling.

File: frontend/police-ui/src/App.css

Requirements:
- Update existing styles to be more modern
- Add styles for:
  - Table (suspects-table): clean borders, hover effects, alternating row colors
  - Form (suspect-form): card-style with shadow, proper input styling
  - Buttons: primary (blue), secondary (gray), danger (red)
  - Loading spinner
  - Error message box (red background, white text)
  - Success message box (green background, white text)
  - Badge for flag status (small red badge with "Flagged")
  - Data request card (border, shadow, clean layout)

- Color scheme:
  - Primary: #1a237e (dark blue - already in header)
  - Success: #2e7d32 (green)
  - Warning: #f57c00 (orange)
  - Danger: #c62828 (red)
  - Gray: #757575

- Use modern CSS:
  - Flexbox/Grid for layouts
  - Box shadows for depth
  - Smooth transitions
  - Responsive design (min-width breakpoints)

- Keep it clean and not over-designed
- Maintain existing header styling
```

### Prompt 4.2: Hospital System - Enhanced CSS

```
Update the CSS for the Hospital System with modern, professional styling.

File: frontend/hospital-ui/src/App.css

Requirements:
- Update existing styles to be more modern
- Add styles for:
  - Table (patients-table): clean borders, hover effects, alternating row colors
  - Form (patient-form): card-style with shadow, proper input styling
  - Buttons: primary (red), secondary (gray), danger (dark red)
  - Loading spinner
  - Error message box (red background, white text)
  - Success message box (green background, white text)
  - Info message box (blue background, white text)
  - Badge for flag status (small red badge with "Flagged by Police")
  - Data request card (border, shadow, clean layout)
  - Toggle switch for "Show Flagged Only" filter

- Color scheme:
  - Primary: #c62828 (red - already in header)
  - Success: #2e7d32 (green)
  - Warning: #f57c00 (orange)
  - Info: #1976d2 (blue)
  - Gray: #757575

- Use modern CSS:
  - Flexbox/Grid for layouts
  - Box shadows for depth
  - Smooth transitions
  - Responsive design (min-width breakpoints)

- Keep it clean and not over-designed
- Maintain existing header styling
- Use similar structure to Police CSS for consistency
```

---

## Phase 5: Documentation & Testing

### Prompt 5.1: Create Frontend README

```
Create a comprehensive README for the frontend setup and usage.

File: docs/FRONTEND.md

Requirements:
- Title: "Frontend Development Guide"
- Sections:

1. **Overview**
   - Brief description of both UIs
   - Technology stack (React, TypeScript)
   - Port numbers (3000 for police, 3001 for hospital)

2. **Prerequisites**
   - Node.js 18+
   - npm
   - Backend services running

3. **Installation**
   - Step-by-step for both systems
   - npm install commands
   - Note about ports

4. **Running the Applications**
   - npm start commands
   - How to run both simultaneously
   - Expected output

5. **Project Structure**
   - Explanation of folder structure
   - Key files and their purposes
   - Services vs Components

6. **Features**
   - Police System features list
   - Hospital System features list
   - Inter-system communication features

7. **API Integration**
   - How services connect to backend
   - Error handling approach
   - CORS considerations

8. **Development Tips**
   - Hot reload
   - Debugging in browser
   - Common issues and solutions

9. **Testing**
   - Manual testing checklist
   - Expected behaviors
   - How to verify inter-system communication

10. **Future Enhancements**
    - Authentication/Authorization UI
    - Better error handling with toast notifications
    - Form validation improvements
    - Loading state improvements
    - Responsive design enhancements

Make it clear, professional, and easy to follow.
```

### Prompt 5.2: Create Frontend Testing Checklist

```
Create a comprehensive testing checklist for the frontend.

File: docs/FRONTEND-TESTING.md

Requirements:
- Title: "Frontend Testing Checklist"
- Sections:

1. **Prerequisites**
   - Backend services must be running
   - Databases must be seeded
   - Both frontends must be running

2. **Police System Tests**
   - Load application (http://localhost:3000)
   - View suspects list
   - Create new suspect
   - Edit existing suspect
   - Delete suspect
   - Toggle flag (and verify auto-sync to hospital)
   - Check hospital records via DataRequest
   - Validate form validation works
   - Test error states

3. **Hospital System Tests**
   - Load application (http://localhost:3001)
   - View patients list
   - Create new patient
   - Edit existing patient
   - Delete patient
   - View flagged patients only (filter)
   - Check police records via DataRequest
   - Verify flags are read-only
   - Test error states

4. **Inter-System Communication Tests**
   - Flag a suspect in police system
   - Verify flag appears immediately in hospital system
   - Check hospital records from police UI
   - Check police records from hospital UI
   - Test with non-existent personal_id (should show "not found")

5. **Browser Testing**
   - Chrome
   - Firefox
   - Safari
   - Responsive design (mobile/tablet/desktop)

6. **Error Scenario Tests**
   - Backend service down
   - Invalid personal_id format
   - Network errors
   - Duplicate personal_id

7. **Performance Tests**
   - Load time for lists
   - Responsiveness of actions
   - Memory leaks (open DevTools)

8. **User Experience Tests**
   - Forms are intuitive
   - Error messages are clear
   - Success feedback is visible
   - Loading states are shown
   - Buttons are clearly labeled

Include expected results for each test. Make it practical and easy to follow.
```

### Prompt 5.3: Create Frontend Development Troubleshooting Guide

```
Create a troubleshooting guide for common frontend issues.

File: docs/FRONTEND-TROUBLESHOOTING.md

Requirements:
- Title: "Frontend Troubleshooting Guide"
- Sections:

1. **Installation Issues**
   - npm install fails
   - Dependency conflicts
   - Node version mismatch

2. **Runtime Issues**
   - Port already in use
   - Cannot connect to backend (CORS errors)
   - Backend service not running
   - Blank page on load

3. **API Issues**
   - 404 Not Found errors
   - 500 Internal Server Error
   - Network request failed
   - CORS policy errors

4. **TypeScript Issues**
   - Type errors in services
   - Missing type definitions
   - Import errors

5. **React Issues**
   - Component not updating
   - State not persisting
   - useEffect infinite loop
   - Event handlers not firing

6. **Styling Issues**
   - CSS not loading
   - Styles not applying
   - Layout broken
   - Responsive design issues

7. **Data Synchronization Issues**
   - Flag not syncing between systems
   - Stale data in UI
   - Refresh needed to see changes

8. **Browser Console Errors**
   - Common error messages and solutions
   - How to use browser DevTools
   - Network tab inspection

For each issue, provide:
- Symptom
- Likely cause
- Solution
- Prevention tips

Make it practical with copy-paste commands where applicable.
```

---

## Phase 6: Enhancements (Optional)

### Prompt 6.1: Add Loading Spinner Component

```
Create a reusable loading spinner component for both systems.

File: frontend/police-ui/src/components/LoadingSpinner.tsx
File: frontend/hospital-ui/src/components/LoadingSpinner.tsx

Requirements:
- No props needed
- Display a CSS-only loading spinner
- Center it on the page
- Use system colors (blue for police, red for hospital)
- Add text below spinner: "Loading..."
- Make it smooth animation
- Export as default

Create two identical files with different colors.
```

### Prompt 6.2: Add Toast Notification Component

```
Create a simple toast notification component to replace window.alert.

Files:
- frontend/police-ui/src/components/Toast.tsx
- frontend/hospital-ui/src/components/Toast.tsx

Requirements:
- Props:
  - message: string
  - type: 'success' | 'error' | 'info'
  - onClose: () => void

- Features:
  - Auto-dismiss after 3 seconds
  - Manual close button
  - Slide in animation from top
  - Different colors based on type
  - Fixed position at top-center of screen

- Use useEffect for auto-dismiss timer
- Clear timeout on unmount
- Export as default

Then update App.tsx files to use Toast instead of window.alert.
```

### Prompt 6.3: Add Search/Filter Component

```
Create a search and filter component for suspect/patient lists.

Files:
- frontend/police-ui/src/components/SearchFilter.tsx
- frontend/hospital-ui/src/components/SearchFilter.tsx

Requirements:
- Props:
  - onSearch: (query: string) => void
  - onFilterFlag: (showFlagged: boolean | null) => void

- Features:
  - Search input (searches by name or personal_id)
  - Filter dropdown: All | Flagged | Not Flagged
  - Clear filters button
  - Debounce search input (300ms)

- Styling:
  - Horizontal layout
  - Clean, modern design
  - Proper spacing

- Use useState for local input state
- Use useEffect for debouncing
- Export as default

Then update App.tsx to integrate search/filter functionality.
```

## Usage Instructions

Run prompts sequentially by phase:

Phase 1: Type definitions and API services (foundation)
Phase 2: Reusable components
Phase 3: Main application components
Phase 4: Styling improvements
Phase 5: Documentation
Phase 6: Optional enhancements

Test after each phase:

After Phase 1: Test API services in browser console
After Phase 2: Test individual components
After Phase 3: Test complete applications
After Phase 4: Verify styling
After Phase 5: Follow testing checklist

Critical dependencies:

Backend must be running before testing frontend
Complete Phase 1 before Phase 2
Complete Phase 2 before Phase 3
Phase 4-6 can be done in any order after Phase 3

Installation after code generation:

```
   # Install dependencies for both systems
   cd frontend/police-ui
   npm install

   cd ../hospital-ui
   npm install
```

Running both applications:

```
   # Terminal 1 - Police UI
   cd frontend/police-ui
   npm start

   # Terminal 2 - Hospital UI
   cd frontend/hospital-ui
   npm start
```

Verification:

Police UI: http://localhost:3000
Hospital UI: http://localhost:3001
Both should load without errors
Check browser console for any issues

## Expected Final State

After completing all prompts:

* Complete Police System UI with CRUD operations
* Complete Hospital System UI with CRUD operations
* Inter-system data requests working
* Flag synchronization visible in UI
* Clean, professional design
* Type-safe TypeScript throughout
* Proper error handling
* Loading states
* Form validation
* Responsive design
* Comprehensive documentation
* Testing checklist
* Troubleshooting guide

## Notes

* All prompts are designed for AI assistants (Claude, GPT-4, etc.)
* Each prompt is self-contained with full context
* Prompts specify exact file paths
* TypeScript strict mode is assumed
* React 18+ features (no class components)
* Functional components with hooks only
* No external UI libraries (Material-UI, etc.) - keep it simple
* CORS must be configured in backend for cross-origin requests
