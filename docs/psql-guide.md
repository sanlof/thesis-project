# PostgreSQL Quick Guide

For this guide to work, you need Mac OS, Homebrew and VSCode.
Also make sure that you have PostgreSQL installed (find download link in README.md).

## Start running PostgreSQL in VSCode

Run the following command in the terminal in VSCode:

`brew services start postgresql`

or

`brew services start postgresql@15`

You can change @XX depending on what version you have installed.

## Stop running PostgreSQL in VSCode

`brew services stop postgresql`

## Set Up Database Schema

First, you'll need to make a new user called postgres by using the following command:

`psql -U USERNAME -d postgres`

Change USERNAME to match whatever username you use on your computer, i.e. John, John-Doe etc.
When you are done, quit by using this command: `\q`.

Now that you have a new user with the username "postgres", you can run the following command in order to setup the database schema:

`sql -U postgres -f shared/database-schemas/schema.sql`

To confirm that the database was successfully created, run:

`psql -U postgres -l`
