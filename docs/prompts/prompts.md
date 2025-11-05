# 1 - Project Setup (with Claude Sonnet 4.5)

## Prompt 1.1

Task: Generate Simple Project Structure
Create a minimal project structure for a thesis project where two IT systems (police and hospital) share data securely.
Requirements:

- Backend: Rust + PostgreSQL (two separate databases)
- Frontend: React + TypeScript
- Two simple user interfaces (one for police, one for hospital)
- Basic API for data exchange between systems
  Deliverables:

1. Simple folder structure (monorepo with both systems)
2. Brief description of key folders only
3. Basic architecture explanation
   Focus on:

- Minimal viable structure - only essential folders and files
- Clear separation between police and hospital systems
- Simple, beginner-friendly organization
- Easy to understand and navigate
  Avoid: Docker, advanced DevOps, complex tooling configurations, over-engineering.

## Prompt 1.2

Make a complete setup guide according to the simple project structure you just generated

## Prompt 1.3

Make a readme-file for the file structure
(Claude named this ARCHITECTURE.md)

## Prompt 1.4 (with Claude Sonnet 4.5)

Create a comprehensive quick guide document for setting up and using PostgreSQL with VS Code on Mac OS. The guide should be titled 'PostgreSQL Quick Guide' and include the following sections:

1. Prerequisites (Mac OS, Homebrew, VSCode, PostgreSQL)
2. How to start PostgreSQL (with both generic and version-specific commands using brew services)
3. How to stop PostgreSQL
4. Step-by-step instructions for setting up a database schema, including:
   - Creating a postgres superuser
   - Running a schema file located at 'shared/database-schemas/schema.sql'
   - Verifying the database was created successfully
5. A troubleshooting section for the common 'role does not exist' error
   Format it in clear markdown with code blocks for all terminal commands. Keep the tone professional but concise. Make it practical and easy to follow for developers who are new to PostgreSQL.

# 2 - Building the Backend

## Prompt 2.1 (with GPT-5)

Write a complete PostgreSQL SQL setup script that creates two separate databases — police_db and hospital_db. Each database should contain only one table: police_db.individuals and hospital_db.patients. Both tables should include columns id, full_name, personal_id, and flag. The flag column should be a boolean in both tables, and any change to the flag in police_db.individuals should automatically mirror to hospital_db.patients where personal_id matches. Use postgres_fdw to implement the synchronization. Include everything needed for setup, including extension creation, foreign server setup, user mapping and trigger definition. Do not include comments, make it copy-and-paste friendly.

## Prompt 2.2 (with GPT-5)

Generate PostgreSQL seed data for two databases (police_db.individuals and hospital_db.patients) with columns: id, full_name, personal_id, flag. Create 8 matching records in both databases and 2 police-only records. Use Swedish names and personal IDs (YYYYMMDD-XXXX format).

## Prompt 2.3 (with Claude Sonnet 4.5)

_Prompt submitted with repository access granted to the AI (docs/prompts excluded)_

Using this repo, create a series of AI prompts in order to generate code for the entire backend of this project.

_See `backend-prompts.md` for results._

## Prompt 2.4 (with Claude Sonnet 4.5)

A Rust backend project with multiple services (e.g., police-system and hospital-system) is failing to build and run due to sqlx errors such as missing relations or columns (e.g., relation "cases" does not exist, column "patient_id" does not exist) and unresolved module errors like use of unresolved module or unlinked crate 'log'.

The database schema and seed data should remain unchanged.

Write a detailed GitHub issue that:

- Describes these compilation and runtime errors clearly,
- Explains that the cause is backend queries and dependencies not matching the existing database schema,
- Proposes a code-based solution (adjusting SQL queries and adding missing crates),
- Includes example fixes or snippets,
- And follows a structured format suitable for posting as a GitHub issue (with sections for description, cause, proposed fix, and testing).

# 3 - Building the Frontend

## Prompt 3.1 (with Claude Sonnet 4.5)

_Prompt submitted with repository access granted to the AI (docs/prompts excluded)_

Using this repository as context, generate a series of AI prompts that will produce a minimal frontend for the project.

Requirements:

- The frontend should only fetch and display data from the police and hospital databases via the existing backend API endpoints in the repository.
- Display this data in simple HTML tables without styling (no css, no frameworks, no advanced UI).
- Exclude any CRUD functionality, authentication, routing, forms, or interactive features.
- The generated code should be as lightweight and dependency-free as possible, using only TypeScript and React.
- Ensure endpoint URLs and data structures match the backend routes defined in the repository.

Include prompts that:

- Set up the basic frontend structure (i.e. index.html).
- Fetch and display police data.
- Fetch and display hospital data.

Goal: Produce a barebones, functional UI that simply renders backend data in tabular format — nothing more.

_See `frontend-prompts.md` for results._

## Prompt 3.2 (with Claude Sonnet 4.5)

_Prompt submitted with repository access granted to the AI (docs/prompts excluded)_

Add flag toggle functionality to the Police Data component (PoliceData.tsx) in this React + TypeScript frontend.

Requirements:

1. Add a toggle button/checkbox in each suspect row to change the flag status
2. On toggle, send PUT request to /api/police/suspects/{personal_id}/flag with JSON body: {"flag": boolean}
3. Update local state after successful API response
4. Show loading state during request (disable toggle button)
5. Handle and display errors if request fails
6. Use existing FlagUpdate type from types.ts
7. Keep styling minimal (no CSS frameworks)

The backend endpoint already exists and automatically syncs flags to the hospital database via PostgreSQL triggers. Generate only the updated PoliceData.tsx component with the flag toggle feature implemented.

## Prompt 3.3 (with Claude Sonnet 4.5)

_Prompt submitted with repository access granted to the AI (docs/prompts excluded)_

## Context

Thesis project with police/hospital systems (Rust backend, React frontend). When police flag suspects, the database syncs instantly via PostgreSQL triggers, but the hospital frontend doesn't auto-refresh.

## Task

Add **smart polling** to `frontend/src/components/HospitalData.tsx` that:

- Polls `/api/hospital/patients` every 3 seconds
- Pauses when tab is inactive (Page Visibility API)
- Shows a subtle "refreshing" indicator
- Cleans up on unmount
- Handles errors gracefully

## Requirements

- Create custom hook `usePolling` in `frontend/src/hooks/usePolling.ts`
- Update `HospitalData.tsx` to use the hook
- TypeScript with proper types
- No jarring UI updates
- Configurable polling interval
- **No CSS**
- **Do not add, change, or remove any existing styling**
- **Do not include any code comments**

## Deliverables

1. `usePolling.ts` – Custom hook with polling logic
2. Updated `HospitalData.tsx` – Using the hook with refresh indicator
3. Brief explanation of implementation

## Success Criteria

- Polling starts/stops correctly
- Pauses when tab inactive
- Flag changes appear within 3 seconds
- No memory leaks
- No CSS
- Clean, maintainable code without comments or styling changes

Existing code uses `useEffect` with one-time fetch. Keep it simple and functional for a thesis demo.

# 4 Security Review

## Prompt 4.1

_Prompt submitted with single files access granted to the AI_

Review this Rust file for security issues in the data transfer path (police → hospital). Focus only on transport, message-level encryption/signature, auth between services, replay protection, validation, DB handling of PII, logging, secrets, and unsafe code. Produce prioritized findings with code/patch suggestions.
