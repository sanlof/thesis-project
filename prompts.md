# 1 - Project Planning (with GPT-5)

## Prompt 1.1

Hi, can you help us make a 4 week plan for the following project?

#### Minimum Viable Product

1. Two IT systems with one database each
2. Basic user interfaces to display data (in the form of user registries)
3. AI-generated code which enables data transfer between the two

We plan on creating two databases containing information of varying sensitivity levels connected to made-up individuals – one for a fictional police department and one for a fictional hospital. We want to design the user interface as simple user registries in order to visualize this data.

We also want to be able to display data between the databases in a secure way. Both systems will hold data that may be relevant or beneficial to the other. For this thesis, our chosen scenario will be flagging persons to indicate they require special attention. For instance, an individual that is known to be violent is flagged by the police, which is useful for a hospital system to display, as a heads-up without disclosing sensitive details.

#### Definition of IT System

An IT system is used to collect, store, use and distribute information within a domain (most often this domain is an organisation or a sub-division of an organisation, but it can also be information about an individual such as their catamnesis etc.). The purpose of an IT system is to enable communication of the information. Sometimes the term is used in a narrower sense, only describing the information technology components of the system – databases, which can store data, and process managers, which are used to manage the flow of information within the system. In this thesis, when we refer to IT systems, we mean it in the narrower sense. We will build two very basic systems with one database each, a simple UI that can display the information, and try to find ways to communicate information between the two.

#### Implementing AI

Exploring its possibilities and limitations, in this thesis we want to make sure to include AI as much as possible throughout the whole process, though, here are the main things we will use it for:

1. Suggest what tech stack to use
2. Generate code
3. Code reviewing (to identify bugs and vulnerabilities) with Copilot and SNYK Code Checker

#### Tech Stack

Our chosen tech stack consists of Rust and PostgreSQL for the backend, paired with React and TypeScript for the frontend.

## Prompt 1.2

Please scaffold a code structure following this brief!

# 2 - Project Setup (with Claude Sonnet 4.5)

## Prompt 2.1

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

## Prompt 2.2

Make a complete setup guide according to the simple project structure you just generated

## Prompt 2.3

Make a readme-file for the file structure
(Claude named this ARCHITECTURE.md)
