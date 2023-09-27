# DB_Mocker

## Introduction

Lib to create a database and fill it with suitable fake dummy data.

Available frontends:
 - [ ] Simple CLI
 - [ ] Complex CLI with TUI
 - [ ] Desktop GUI

## How to use

WIP

## Progress

Lib:
  - [ ] Create virtual database and it's users
    - [ ] Write virtual db to dbms
    - [ ] Fill database with dummy data
  - [ ] Virtual DatabaseReader/-Writer traits
  - [ ] Implementations of DatabaseReader/-Writer traits
    - [ ] FileWriter - (Saves vdb to disk)
    - [ ] FileReader - (Read vdb to disk)
    - [ ] DbmsWriter - (Create real db in dbms)
    - [ ] DbmsReader - (Read schema of real db from dbms)
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