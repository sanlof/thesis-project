# PostgreSQL Quick Guide

For this guide to work, you need Mac OS, Homebrew and VSCode.
Also make sure that you have PostgreSQL installed (find download link in README.md).

## Start running PostgreSQL

Run the following command in the terminal in VSCode:

```
brew services start postgresql
```

or, if you want to specify a version, add @XX like so:

```
brew services start postgresql@15
```

## Stop running PostgreSQL

To stop running PostgreSQL in VSCode, use the following command:

```
brew services stop postgresql
```

## Set Up Database Schema

1. First, make sure PostgreSQL is running (see "Start running PostgreSQL" above).

2. Create a new user called "postgres". Run this command in your terminal:

```
createuser -s postgres
```

This creates a superuser role named "postgres" that you'll use for database operations.

3. Now set up the database schema by running:

```
psql -U postgres -f shared/database-schemas/schema.sql
```

4. To confirm that the database was successfully created, run:

```
psql -U postgres -l
```

This will list all databases. You should see your newly created database in the list.

## Troubleshooting

If you get a "role does not exist" error, it means the postgres user wasn't created successfully. Try connecting with your Mac username first:

```
psql -d postgres
```

Then create the postgres role manually:

```sql
CREATE ROLE postgres WITH LOGIN SUPERUSER;
```

Type `\q` to quit, then proceed with step 3 above.
