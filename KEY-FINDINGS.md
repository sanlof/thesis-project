# Key Findings

Presented below are the key findings derived from the research conducted in this thesis project.

## 1. AI Can Build Basic Functionality, Not Secure Integrated Systems

AI successfully generated:

- A Rust + PostgreSQL backend
- A React + TypeScript frontend
- PostgreSQL triggers for sharing data between systems

This was enough for a minimal viable product.  
However, AI could **not** maintain both functionality _and_ security.

---

## 2. AI Outputs Were Inconsistent and Overengineered

Across multiple models and identical prompts, AI:

- Produced conflicting or incompatible code
- Overcomplicated solutions
- Added unnecessary styling or features
- Delivered inconsistent results

This made coherent development difficult.

---

## 3. Frontend–Backend Integration Frequently Broke

Security updates that worked in the backend:

- Broke the frontend
- Introduced mismatches the AI couldn’t resolve

AI struggled with:

- Coordinating changes between systems
- Implementing or handling TLS, CSRF, or secure headers
- Maintaining a stable full-stack architecture

---

## 4. AI Helped With Backend Security but Was Not Reliable

AI could:

- Identify some vulnerabilities
- Suggest security improvements

But:

- Fixes often introduced new issues
- Re-running the same audit prompts produced different results
- AI repeatedly found new vulnerabilities in its own corrected code

---

## 5. Traditional Tools Outperformed AI Security Tools

AI-based tools (Codacy, Snyk) were ineffective for this project’s goals.

Effective tools:

- **OWASP ZAP** (frontend ↔ backend vulnerabilities)
- **Nmap** (backend port/configuration analysis)

AI struggled to guide security testing safely or accurately.

---

## 6. AI Lacked Legal and Ethical Awareness

AI sometimes recommended:

- Running scans on non-local targets
- Actions that could violate cybersecurity laws

It only acknowledged these risks when explicitly prompted.

---

## 7. Rate Limits and Technical Constraints Slowed Progress

Issues included:

- Model switching due to free-tier limits
- Prompt caps even on paid versions
- Limited ability to fully access the GitHub repository
- Loss of context across iterations

These interruptions hindered workflow continuity.

---

## 8. Overall Conclusion

AI can be a helpful assistant, but is not a reliable autonomous developer—especially for:

- Security-critical code
- Multi-system integration
- Complex architectural decisions
- Maintaining consistent, secure behavior over time

Human oversight, validation, and ethical judgment remain essential.
