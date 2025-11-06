# PostgreSQL Quick Guide

A practical guide for setting up and using PostgreSQL with VS Code on macOS.

## Prerequisites

Before you begin, ensure you have the following installed:

- **macOS** (any recent version)
- **Homebrew** - Package manager for macOS
- **VS Code** - Your code editor
- **PostgreSQL** - Installed via Homebrew

To install PostgreSQL if you haven't already:

```bash
brew install postgresql@15
```

## Starting PostgreSQL

### Generic Command (Latest Version)

```bash
brew services start postgresql
```

### Version-Specific Command

If you have a specific version installed (e.g., PostgreSQL 15):

```bash
brew services start postgresql@15
```

To verify PostgreSQL is running:

```bash
brew services list
```

Look for `postgresql` with a status of `started`.

## Stopping PostgreSQL

### Generic Command

```bash
brew services stop postgresql
```

### Version-Specific Command

```bash
brew services stop postgresql@15
```

## Setting Up Your Database Schema

Follow these steps to create and configure your database.

### Step 1: Create a PostgreSQL Superuser

First, create a superuser with your macOS username:

```bash
createuser -s postgres
```

This creates a superuser named `postgres` with full permissions.

### Step 2: Run Your Schema File

Execute your schema file to create databases, tables, and structure:

```bash
psql -U postgres -f shared/database-schemas/schema.sql
```

**Command breakdown:**

- `-U postgres` - Connect as the postgres user
- `-f shared/database-schemas/schema.sql` - Execute the SQL file

This command will:

- Create both `police_db` and `hospital_db` databases
- Set up all tables and relationships
- Configure postgres_fdw for cross-database synchronization

### Step 3: Seed the Databases with Sample Data

After creating the schema, populate your databases with seed data:

```bash
psql -U postgres -f shared/database-schemas/seed-data.sql
```

This will insert:

- 8 matching records in both `police_db.suspects` and `hospital_db.patients`
- 2 additional police-only records (Simon Nyberg and Carina Dahl)
- Swedish names with Swedish personal ID format (YYYYMMDD-XXXX)

### Step 4: Verify Database Creation

Connect to the hospital database to verify everything worked:

```bash
psql -U postgres -d hospital_db
```

Once connected, you can list all tables:

```sql
\dt
```

Check the patients data:

```sql
SELECT * FROM patients;
```

To exit and check the police database:

```sql
\q
```

```bash
psql -U postgres -d police_db
```

Check the suspects data:

```sql
SELECT * FROM suspects;
```

To see all databases:

```sql
\l
```

To exit the PostgreSQL prompt:

```sql
\q
```

## Testing Flag Synchronization

One of the key features of this project is automatic flag synchronization between databases. Here's how to test it:

### Connect to the police database:

```bash
psql -U postgres -d police_db
```

### Update a flag in the police database:

```sql
UPDATE suspects SET flag = true WHERE personal_id = '19850312-2398';
```

### Verify it synced to the hospital database:

```sql
\c hospital_db
SELECT full_name, personal_id, flag FROM patients WHERE personal_id = '19850312-2398';
```

You should see the flag is now `true` in the hospital database as well!

### Test multiple updates:

```sql
\c police_db
UPDATE suspects SET flag = true WHERE personal_id IN ('19781123-5634', '19670630-8841');

\c hospital_db
SELECT full_name, personal_id, flag FROM patients WHERE flag = true;
```

## Resetting Your Database

If you need to start fresh (useful during development):

### Option 1: Drop and recreate everything

```bash
psql -U postgres -c "DROP DATABASE IF EXISTS police_db;"
psql -U postgres -c "DROP DATABASE IF EXISTS hospital_db;"
psql -U postgres -f shared/database-schemas/schema.sql
psql -U postgres -f shared/database-schemas/seed-data.sql
```

### Option 2: Just clear the data

```bash
psql -U postgres -d hospital_db -c "TRUNCATE patients RESTART IDENTITY CASCADE;"
psql -U postgres -d police_db -c "TRUNCATE suspects RESTART IDENTITY CASCADE;"
```

Then re-run the seed data file:

```bash
psql -U postgres -f shared/database-schemas/seed-data.sql
```

## Troubleshooting

### Error: "role does not exist"

If you encounter an error like `psql: error: FATAL: role "your_username" does not exist`, try these solutions:

**Solution 1: Create a role with your macOS username**

```bash
createuser -s $(whoami)
```

This creates a superuser with your current macOS username.

**Solution 2: Always specify the user explicitly**

When connecting to PostgreSQL, always use the `-U` flag:

```bash
psql -U postgres -d your_database_name
```

**Solution 3: Set a default user in your shell profile**

Add this to your `~/.zshrc` or `~/.bash_profile`:

```bash
export PGUSER=postgres
```

Then reload your shell:

```bash
source ~/.zshrc
```

### Error: "database already exists"

If you get this error when running the schema file:

```bash
# Drop existing databases first
psql -U postgres -c "DROP DATABASE IF EXISTS police_db;"
psql -U postgres -c "DROP DATABASE IF EXISTS hospital_db;"

# Then run the schema file again
psql -U postgres -f shared/database-schemas/schema.sql
```

### Checking Current Roles

To see all existing roles in PostgreSQL:

```bash
psql -U postgres -c "\du"
```

## Quick Reference

| Action              | Command                                                     |
| ------------------- | ----------------------------------------------------------- |
| Start PostgreSQL    | `brew services start postgresql`                            |
| Stop PostgreSQL     | `brew services stop postgresql`                             |
| Run schema file     | `psql -U postgres -f shared/database-schemas/schema.sql`    |
| Seed databases      | `psql -U postgres -f shared/database-schemas/seed-data.sql` |
| Connect to database | `psql -U postgres -d database_name`                         |
| List databases      | `\l` (inside psql prompt)                                   |
| List tables         | `\dt` (inside psql prompt)                                  |
| Switch database     | `\c database_name` (inside psql prompt)                     |
| Exit psql           | `\q`                                                        |
| Drop database       | `psql -U postgres -c "DROP DATABASE database_name;"`        |

## Additional Tips

- **VS Code Extension**: Install the "PostgreSQL" extension by Chris Kolkman for database management within VS Code
- **Connection String Format**: `postgresql://postgres@localhost:5432/your_database_name`
- **Default Port**: PostgreSQL runs on port `5432` by default
- **Data Directory**: Homebrew stores PostgreSQL data in `/opt/homebrew/var/postgresql@15/`
- **Separate Files**: The schema and seed data are in separate files for easier management during development

---

_Need more help? Check the official PostgreSQL documentation at [postgresql.org/docs](https://www.postgresql.org/docs/)_
