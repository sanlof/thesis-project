# Thesis Project - Secure Data Sharing Between IT Systems

A proof-of-concept system demonstrating automatic data synchronization between independent organizations using PostgreSQL Foreign Data Wrapper.

## 🎯 Project Overview

This project showcases how two independent IT systems (police and hospital) can securely share data while maintaining data sovereignty. The key innovation is using **PostgreSQL's Foreign Data Wrapper (FDW)** to enable automatic flag synchronization at the database level, eliminating complex API-level coordination.

**Real-World Scenario:**

- Police flag a suspect for monitoring
- Hospital system automatically receives the flag update
- Hospital staff can see flagged patients without manual coordination
- No application code needed for synchronization

## ✨ Key Features

- **Automatic Flag Synchronization** - Database triggers instantly sync flag updates from police to hospital
- **Independent Backend Services** - Separate Rust/Actix services for police and hospital systems
- **Cross-System Queries** - Each system can query the other's data via shared API endpoints
- **Type-Safe APIs** - Rust with SQLx prevents SQL injection and ensures data integrity
- **React Frontend** - Simple web interface to visualize data from both systems
- **Swedish Personal IDs** - Uses Swedish format (YYYYMMDD-XXXX) for identity management

## 🏗️ Technology Stack

| Component     | Technology                       |
| ------------- | -------------------------------- |
| **Backend**   | Rust, Actix-web, SQLx, Tokio     |
| **Frontend**  | React 18, TypeScript, Vite       |
| **Database**  | PostgreSQL 15+ with postgres_fdw |
| **Dev Tools** | Cargo, npm, Homebrew             |

## 📋 Prerequisites

Before you begin, ensure you have:

- **Rust** (latest stable) - [Install Rust](https://rustup.rs/)
- **Node.js 18+** - [Install Node.js](https://nodejs.org/)
- **PostgreSQL 15+** - Install via Homebrew:
  ```bash
  brew install postgresql@15
  ```

## 🚀 Quick Start

### 1. Install PostgreSQL and Start Service

```bash
# Install PostgreSQL (if not already installed)
brew install postgresql@15

# Start PostgreSQL service
brew services start postgresql@15

# Verify it's running
brew services list
```

### 2. Create Databases and Load Seed Data

```bash
# Create a PostgreSQL superuser
createuser -s postgres

# Run schema file to create databases, tables, and FDW setup
psql -U postgres -f shared/database-schemas/schema.sql

# Load seed data (8 shared records + 2 police-only records)
psql -U postgres -f shared/database-schemas/seed-data.sql

# Verify database creation
psql -U postgres -l | grep -E "police_db|hospital_db"
```

**Expected output:**

```
 hospital_db | postgres | UTF8     | ...
 police_db   | postgres | UTF8     | ...
```

### 3. Configure Backend Services

```bash
# Police system
cd backend/police-system
cp .env.example .env
# Edit .env if needed (defaults should work)

# Hospital system
cd ../hospital-system
cp .env.example .env
# Edit .env if needed (defaults should work)
```

**Default .env configuration:**

```env
# Police system (.env)
DATABASE_URL=postgresql://postgres@localhost/police_db
SERVER_PORT=8000
HOSPITAL_API_URL=http://localhost:8001

# Hospital system (.env)
DATABASE_URL=postgresql://postgres@localhost/hospital_db
SERVER_PORT=8001
POLICE_API_URL=http://localhost:8000
```

### 4. Start Backend Services

Open **two terminal windows:**

**Terminal 1 - Police System:**

```bash
cd backend/police-system
cargo run
```

**Expected output:**

```
🚔 Police System Starting...
Connecting to database...
✅ Database connection established
📋 Configuring routes:
   - GET    /suspects
   - POST   /suspects
   ...
🚀 Starting HTTP server at http://127.0.0.1:8000
```

**Terminal 2 - Hospital System:**

```bash
cd backend/hospital-system
cargo run
```

**Expected output:**

```
🏥 Hospital System Starting...
Connecting to database...
✅ Database connection established
📋 Configuring routes:
   - GET    /patients
   - POST   /patients
   ...
🚀 Starting HTTP server at http://127.0.0.1:8001
```

### 5. Start Frontend (Optional)

Open a **third terminal window:**

```bash
cd frontend
npm install
npm run dev
```

**Expected output:**

```
  VITE v5.4.0  ready in 500 ms

  ➜  Local:   http://localhost:3000/
  ➜  Network: use --host to expose
```

### 6. Verify Everything is Running

```bash
# Test police system
curl http://localhost:8000/health

# Test hospital system
curl http://localhost:8001/health

# View suspects
curl http://localhost:8000/suspects

# View patients
curl http://localhost:8001/patients
```

**Open in browser:**

- Frontend: http://localhost:3000
- Police API: http://localhost:8000/suspects
- Hospital API: http://localhost:8001/patients

## 🧪 Testing Flag Synchronization

This is the **core feature** of the system. Let's test it:

### Step 1: Check Initial State

```bash
# Check suspect in police system
curl http://localhost:8000/suspects/personal/19850312-2398 | jq '.flag'
# Output: false

# Check same person in hospital system
curl http://localhost:8001/patients/personal/19850312-2398 | jq '.flag'
# Output: false
```

### Step 2: Flag the Suspect

```bash
curl -X PUT http://localhost:8000/suspects/19850312-2398/flag \
  -H "Content-Type: application/json" \
  -d '{"flag": true}'
```

### Step 3: Verify Automatic Synchronization

```bash
# Check hospital system (should now be true)
curl http://localhost:8001/patients/personal/19850312-2398 | jq '.flag'
# Output: true
```

**✨ The flag synchronized automatically via database trigger!**

### Step 4: View All Flagged Patients

```bash
curl http://localhost:8001/patients/flagged | jq
```

## 📁 Project Structure

```
thesis-project/
├── backend/
│   ├── police-system/        # Rust backend for police (port 8000)
│   └── hospital-system/      # Rust backend for hospital (port 8001)
├── frontend/                 # React + TypeScript UI (port 3000)
├── shared/
│   └── database-schemas/     # SQL schemas and seed data
├── docs/
│   ├── API.md               # Complete API documentation
│   ├── TESTING.md           # Backend testing guide
│   └── psql-guide.md        # PostgreSQL setup guide
├── ARCHITECTURE.md          # Detailed architecture documentation
└── README.md                # This file
```

## 📚 Documentation

| Document                                 | Description                                                         |
| ---------------------------------------- | ------------------------------------------------------------------- |
| [ARCHITECTURE.md](ARCHITECTURE.md)       | Complete architecture, design decisions, and implementation details |
| [docs/API.md](docs/API.md)               | REST API reference for both backend systems                         |
| [docs/TESTING.md](docs/TESTING.md)       | Manual testing guide with test scenarios                            |
| [docs/psql-guide.md](docs/psql-guide.md) | PostgreSQL setup and troubleshooting                                |

## 🔧 Common Commands

### Database Management

```bash
# Connect to police database
psql -U postgres -d police_db

# Connect to hospital database
psql -U postgres -d hospital_db

# Reset databases (start fresh)
psql -U postgres -c "DROP DATABASE IF EXISTS police_db;"
psql -U postgres -c "DROP DATABASE IF EXISTS hospital_db;"
psql -U postgres -f shared/database-schemas/schema.sql
psql -U postgres -f shared/database-schemas/seed-data.sql

# Stop PostgreSQL
brew services stop postgresql@15
```

### Backend Development

```bash
# Run police system
cd backend/police-system
cargo run

# Run hospital system
cd backend/hospital-system
cargo run

# Run tests (when implemented)
cargo test

# Format code
cargo fmt

# Check for errors without running
cargo check
```

### Frontend Development

```bash
cd frontend

# Install dependencies
npm install

# Start dev server
npm run dev

# Build for production
npm run build

# Preview production build
npm run preview
```

## 🌐 API Endpoints

### Police System (Port 8000)

| Method  | Endpoint                             | Description                        |
| ------- | ------------------------------------ | ---------------------------------- |
| GET     | `/suspects`                          | List all suspects                  |
| POST    | `/suspects`                          | Create new suspect                 |
| GET     | `/suspects/{id}`                     | Get suspect by ID                  |
| PUT     | `/suspects/{id}`                     | Update suspect                     |
| DELETE  | `/suspects/{id}`                     | Delete suspect                     |
| GET     | `/suspects/personal/{personal_id}`   | Get suspect by Swedish personal ID |
| **PUT** | **`/suspects/{personal_id}/flag`**   | **Update flag (triggers sync)**    |
| GET     | `/api/shared/suspects/{personal_id}` | Check suspect (for hospital)       |
| GET     | `/health`                            | Health check                       |

### Hospital System (Port 8001)

| Method  | Endpoint                             | Description                        |
| ------- | ------------------------------------ | ---------------------------------- |
| GET     | `/patients`                          | List all patients                  |
| POST    | `/patients`                          | Create new patient                 |
| GET     | `/patients/{id}`                     | Get patient by ID                  |
| PUT     | `/patients/{id}`                     | Update patient                     |
| DELETE  | `/patients/{id}`                     | Delete patient                     |
| GET     | `/patients/personal/{personal_id}`   | Get patient by Swedish personal ID |
| **GET** | **`/patients/flagged`**              | **Get all flagged patients**       |
| GET     | `/api/shared/patients/{personal_id}` | Check patient (for police)         |
| GET     | `/health`                            | Health check                       |

See [docs/API.md](docs/API.md) for complete API documentation with examples.

## 🛠️ Troubleshooting

### PostgreSQL Issues

**Error: "role does not exist"**

```bash
# Solution: Create the postgres user
createuser -s postgres
```

**Error: "database already exists"**

```bash
# Solution: Drop and recreate
psql -U postgres -c "DROP DATABASE IF EXISTS police_db;"
psql -U postgres -c "DROP DATABASE IF EXISTS hospital_db;"
psql -U postgres -f shared/database-schemas/schema.sql
```

**PostgreSQL not running**

```bash
# Check status
brew services list

# Start PostgreSQL
brew services start postgresql@15
```

### Backend Issues

**Error: "Failed to connect to database"**

- Check PostgreSQL is running: `brew services list`
- Verify DATABASE_URL in `.env` file
- Check database exists: `psql -U postgres -l`

**Error: "Address already in use"**

```bash
# Find process using port 8000
lsof -i :8000

# Kill the process
kill -9 <PID>
```

### Frontend Issues

**Error: "Port 3000 already in use"**

- Change port in `vite.config.ts`
- Or kill the process: `lsof -i :3000` then `kill -9 <PID>`

**CORS errors**

- Ensure backend CORS settings allow localhost:3000
- Check Vite proxy configuration in `vite.config.ts`

### General Debugging

1. **Check logs** - Each service outputs detailed logs
2. **Test backends separately** - Use curl to test API endpoints
3. **Verify database** - Connect with psql to check data
4. **Restart services** - Sometimes a fresh start helps

See [docs/TESTING.md](docs/TESTING.md) for comprehensive testing guide.

## 🔐 Security Notice

**⚠️ This is a development/thesis project with no security features:**

- No authentication
- No authorization
- No encryption
- CORS allows all origins
- No rate limiting
- No input validation

**Do NOT use in production without implementing:**

- JWT authentication
- HTTPS/TLS
- Input validation
- Rate limiting
- Audit logging
- GDPR compliance

See [ARCHITECTURE.md](ARCHITECTURE.md#security-considerations) for production requirements.

## 📊 Sample Data

The seed data includes:

**Shared Records (in both databases):**

1. Erik Andersson - 19850312-2398
2. Anna Karlsson - 19900204-1457 (flagged)
3. Johan Lindström - 19781123-5634
4. Maria Svensson - 19891215-0912 (flagged)
5. Lars Johansson - 19670630-8841
6. Emma Nilsson - 19950419-3325 (flagged)
7. Oskar Berg - 19801005-7420
8. Elin Eriksson - 20010122-2183 (flagged)

**Police-Only Records:** 9. Simon Nyberg - 19930808-4417 (flagged) 10. Carina Dahl - 19870527-6675

## 🎓 Academic Context

This project demonstrates:

- Database-level synchronization using Foreign Data Wrapper
- Separation of concerns between independent systems
- Type-safe API development with Rust
- Async/await patterns for non-blocking I/O
- RESTful API design
- Cross-system data sharing patterns

**Thesis Focus Areas:**

- Secure data sharing between organizations
- Database triggers for automatic synchronization
- Maintaining data sovereignty while enabling collaboration
- Performance implications of FDW vs API-based sync

## 📝 Future Enhancements

**Backend:**

- [ ] JWT authentication
- [ ] Rate limiting
- [ ] Pagination
- [ ] WebSocket for real-time updates

**Frontend:**

- [ ] Create/Update/Delete forms
- [ ] Flag toggle interface
- [ ] Cross-system query UI
- [ ] Real-time sync visualization
- [ ] Proper styling

**Database:**

- [ ] Audit logging
- [ ] Performance indexes
- [ ] Backup strategy

**Testing:**

- [ ] Unit tests
- [ ] Integration tests
- [ ] E2E tests

See [ARCHITECTURE.md](ARCHITECTURE.md#implementation-status) for complete roadmap.

## 🤝 Contributing

This is a thesis project, but suggestions are welcome:

1. Follow Rust style guidelines (`cargo fmt`)
2. Update documentation for changes
3. Add tests for new functionality
4. Use meaningful commit messages

## 🙏 Acknowledgments

- PostgreSQL team for Foreign Data Wrapper
- Actix-web and SQLx communities
- React and Vite teams

## 📞 Support

For issues:

1. Check [docs/TESTING.md](docs/TESTING.md) for testing guide
2. Check [docs/psql-guide.md](docs/psql-guide.md) for PostgreSQL help
3. Review server logs for error details
4. Verify PostgreSQL is running

## 🎉 Quick Commands Reference

```bash
# Start everything
brew services start postgresql@15
cd backend/police-system && cargo run &
cd backend/hospital-system && cargo run &
cd frontend && npm run dev

# Test flag synchronization
curl -X PUT http://localhost:8000/suspects/19850312-2398/flag \
  -H "Content-Type: application/json" \
  -d '{"flag": true}'

# View flagged patients
curl http://localhost:8001/patients/flagged | jq

# Reset database
psql -U postgres -c "DROP DATABASE IF EXISTS police_db;"
psql -U postgres -c "DROP DATABASE IF EXISTS hospital_db;"
psql -U postgres -f shared/database-schemas/schema.sql
psql -U postgres -f shared/database-schemas/seed-data.sql
```

---

**Happy coding! 🚀**

_For detailed architecture information, see [ARCHITECTURE.md](ARCHITECTURE.md)_

_Last Updated: January 2025_
