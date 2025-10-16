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

# 1 - Project Setup (with GPT-5)

## Prompt 1.4

Write a complete PostgreSQL SQL setup script that creates two separate databases — police_db and hospital_db. Each database should contain only one table: police_db.individuals and hospital_db.patients. Both tables should include columns id, full_name, personal_id, and flag. The flag column should be a boolean in both tables, and any change to the flag in police_db.individuals should automatically mirror to hospital_db.patients where personal_id matches. Use postgres_fdw to implement the synchronization. Include everything needed for setup, including extension creation, foreign server setup, user mapping and trigger definition. Do not include comments, make it copy-and-paste friendly.
