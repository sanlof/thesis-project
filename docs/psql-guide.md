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

1. In order to set up the database, you first need to create a new user called postgres by using the following command:

```
psql -U USERNAME -d postgres
```

**NB: Change USERNAME to match whatever username you use on your computer, i.e. John, John-Doe etc.**

When you are done, quit by using this command: `\q`.

2. Now that there is a user with the username "postgres", you may run the following command in order to setup the database schema:

```
sql -U postgres -f shared/database-schemas/schema.sql
```

3. To confirm that the database was successfully created, run:

```
psql -U postgres -l
```
