# Frontend Generation Prompts

Here's a series of prompts to generate a minimal frontend for your police-hospital system. Use these prompts sequentially with an AI assistant.

## Prompt 1: Project Setup

```
Create a minimal React + TypeScript frontend project structure for displaying data from two backend APIs (police system on port 8000 and hospital system on port 8001).

Requirements:
- Use Vite as the build tool for simplicity
- TypeScript for type safety
- React 18
- No UI frameworks or CSS libraries
- No routing, authentication, or forms
- Minimal dependencies (only React, ReactDOM, TypeScript)

Generate:
1. package.json with minimal dependencies
2. tsconfig.json with strict type checking
3. vite.config.ts for development server
4. index.html as the entry point
5. src/main.tsx as the React entry point
6. src/App.tsx as the root component (empty for now)
7. A README.md with setup instructions

The dev server should run on port 3000 and proxy API requests to avoid CORS issues.
```

---

## Prompt 2: TypeScript Type Definitions

```
Based on the backend API documentation, create TypeScript type definitions for the data models.

From the backend code, the data structures are:

**Suspect (Police System):**
- id: number
- full_name: string | null
- personal_id: string | null
- flag: boolean | null

**Patient (Hospital System):**
- id: number
- full_name: string | null
- personal_id: string | null
- flag: boolean | null

Generate a file `src/types.ts` that exports these interfaces:
- Suspect
- Patient

Note: All fields except id can be null based on the backend models.
```

---

## Prompt 3: Police Data Component

```
Create a React component that fetches and displays all suspects from the police system API.

Requirements:
- Component name: PoliceData
- Fetch from: http://localhost:8000/suspects
- Display data in a plain HTML table with these columns: ID, Full Name, Personal ID, Flag
- Show loading state as plain text "Loading police data..."
- Show error state as plain text "Error loading police data: [error message]"
- No styling, no CSS classes
- Use the Suspect type from src/types.ts
- Handle null values by displaying "N/A"
- Display flag as "Yes" or "No" instead of true/false

Generate the complete component in `src/components/PoliceData.tsx`.
```

---

## Prompt 4: Hospital Data Component

```
Create a React component that fetches and displays all patients from the hospital system API.

Requirements:
- Component name: HospitalData
- Fetch from: http://localhost:8001/patients
- Display data in a plain HTML table with these columns: ID, Full Name, Personal ID, Flag
- Show loading state as plain text "Loading hospital data..."
- Show error state as plain text "Error loading hospital data: [error message]"
- No styling, no CSS classes
- Use the Patient type from src/types.ts
- Handle null values by displaying "N/A"
- Display flag as "Yes" or "No" instead of true/false

Generate the complete component in `src/components/HospitalData.tsx`.
```

---

## Prompt 5: Main App Component

```
Update the App.tsx component to display both PoliceData and HospitalData components.

Requirements:
- Import both PoliceData and HospitalData components
- Display a simple heading "Police and Hospital Data System"
- Show PoliceData under an <h2> heading "Police System - Suspects"
- Show HospitalData under an <h2> heading "Hospital System - Patients"
- Add a horizontal rule (<hr>) between the two sections
- No styling, no CSS
- Keep everything as plain HTML

Generate the complete updated `src/App.tsx` file.
```

