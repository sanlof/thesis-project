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

### Step 2: Create Your Database

Create a new database (replace `your_database_name` with your desired name):

```bash
createdb your_database_name
```

### Step 3: Run Your Schema File

Execute your schema file to set up tables and structure:

```bash
psql -U postgres -d your_database_name -f shared/database-schemas/schema.sql
```

**Command breakdown:**
- `-U postgres` - Connect as the postgres user
- `-d your_database_name` - Specify which database to use
- `-f shared/database-schemas/schema.sql` - Execute the SQL file

### Step 4: Verify Database Creation

Connect to your database to verify everything worked:

```bash
psql -U postgres -d your_database_name
```

Once connected, you can list all tables:

```sql
\dt
```

To see more database information:

```sql
\l
```

To exit the PostgreSQL prompt:

```sql
\q
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

### Checking Current Roles

To see all existing roles in PostgreSQL:

```bash
psql -U postgres -c "\du"
```

## Quick Reference

| Action | Command |
|--------|---------|
| Start PostgreSQL | `brew services start postgresql` |
| Stop PostgreSQL | `brew services stop postgresql` |
| Create database | `createdb database_name` |
| Connect to database | `psql -U postgres -d database_name` |
| Run SQL file | `psql -U postgres -d database_name -f path/to/file.sql` |
| List databases | `psql -U postgres -c "\l"` |
| List tables | `\dt` (inside psql prompt) |

## Additional Tips

- **VS Code Extension**: Install the "PostgreSQL" extension by Chris Kolkman for database management within VS Code
- **Connection String Format**: `postgresql://postgres@localhost:5432/your_database_name`
- **Default Port**: PostgreSQL runs on port `5432` by default
- **Data Directory**: Homebrew stores PostgreSQL data in `/opt/homebrew/var/postgresql@15/`

---

*Need more help? Check the official PostgreSQL documentation at [postgresql.org/docs](https://www.postgresql.org/docs/)*
