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

_Prompt submitted with repository access granted to the AI._

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

_Prompt submitted with repository access granted to the AI._

Design a minimal frontend architecture that integrates cleanly with the existing project. The frontend should serve only as a demo interface, focused primarily on data transfer and integration with the backend. Avoid adding any unnecessary features or UI complexity.

The frontend should be built using **React with TypeScript**. No actual code implementation is required at this stage — only a detailed outline of how the frontend should be structured, including recommended file/folder organization, component layout, and integration points with the backend.

Finally, add this proposed architecture and its explanation to the project’s **ARCHITECTURE.md** file, ensuring it fits cohesively with the existing documentation style and structure.

## Prompt 3.2 (with Claude Sonnet 4.5)

_Prompt submitted with repository access granted to the AI._

Using this repository — including the architecture.md file for guidance on structure and design — create a detailed sequence of AI prompts that will generate the complete frontend code for this project. Each prompt should focus on a specific part of the frontend (e.g., layout setup, routing, UI components, API integration, and state management) and ensure alignment with the backend endpoints and architecture defined in the repo. The goal is to produce a frontend consistent with the overall system architecture.

_See `frontend-prompts.md` for results._
