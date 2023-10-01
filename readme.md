# DB_Mocker

## Introduction

Lib to create a database and fill it with suitable fake dummy data.

Available frontends:
 - [ ] Simple CLI
 - [ ] Complex CLI with TUI
 - [ ] Desktop GUI

## Next steps to do

Implement the simplest minimal working prototype.

This means no validation, no error handling, no nothing.
Don't implement functions and don't need yet. Don't try to write finished software in one go.
Using setters where I plan to put validation is ok.

Implement tests to use the functions.

1. Implement the VirtualDatabase struct and it's component structs.
2. Implement minimal functions for these. Don't bother to consider what may be.
3. Plan the VirtualDatabase Controller.
4. Implement the VirtualDatabase Controller.

The Controllers job is to handle the data and manipulate it. The Managers job is to store it.
The Controller will connect all the smaller sub systems and have them interopt with each other.

5. Implement a database connection.
6. Implement the sql queries for writing to dbms.
7. Implement data generation.

After this refactoring will happen.

- Clean up code and seperate concerns (Manager storing data and giving access to it. Controller manipulating data and interfacing systems)
- Reader/Writer traits and implementations
- ? Validators
- ? Error collectors
- Settings (A lib shouldn't read from env or config files. Instead it should get passed an settings instance from the main app.)
- Saving and loading to and from yaml files
- Import and export of .sql files
- Support for other relational database systems (MySQL / MariaDB, phpMyAdmin, PostgreSQL, Oracle, MSSQL)

Eventually the dreaded last step: Writing documentation

## Progress

Lib:
  - [ ] Create virtual database and it's users
    - [ ] Write virtual db to dbms
    - [ ] Fill database with dummy data
  - [ ] Virtual DatabaseReader/-Writer traits
  - [ ] Implementations of DatabaseReader/-Writer traits
    - [ ] FileWriter - (Saves vdb to disk / Writes .yaml file from vdb)
    - [ ] FileReader - (Read vdb to disk / Constructs vdb from .yaml file)
    - [ ] DbmsWriter - (Create real db in dbms / SQL queries for creating db, tables, users and inserting data)
    - [ ] DbmsReader - (Read schema of real db from dbms / Construct vdb from database schema)
  - [ ] Settings management with config file (Key Value pairs)
    - [ ] Read keys
    - [ ] Edit keys
  - [ ] Save/Load virtual database to disk
    - [ ] Autosave
    - [ ] Export (save virtual database to specific location)
    - [ ] Autoload (start up behaviour)
  - [ ] Import/Export database from/to .sql file
    - [ ] Import
    - [ ] Export

Simple CLI - Planning WIP:
  - [ ] Non db views
    - [ ] Introduction + Main menu
      - Create vdb -> Vdb Overview
      - Load vdb -> Vdb Overview
      - Export/Save vdb
      - Show real databases -> Real databases view
      - Change settings -> Settings menu
      - Exit program
    - [ ] Help view
    - [ ] Settings menu
      - List keys
      - Edit keys
  - [ ] Virtual database views
    - [ ] Overview
      - !!! Further planning
    - [ ] Table
      - !!! Further planning
    - [ ] Column
      - !!! Further planning
    - [ ] User
      - !!! Further planning
  - [ ] Dbms view
    - [ ] Real databases
      - !!! Further planning
    - [ ] Tables of real database
      - !!! Further planning
  - [ ] Containerization support
    - [ ] Docker container
    - [ ] Docker composer to include dbms container

Complex CLI with TUI:
  - No details planned yet

Desktop GUI:
  - No details planned yet