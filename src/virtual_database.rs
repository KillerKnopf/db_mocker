// TODO for this file:
// -------------------------
// Implement error handling with the crates 'anyhow' or 'thisError'

// Implement database creation

// Implement table filling

// Struct fields will be made pub when needed
// -------------------------

// THOUGTS for this file:
// -------------------------

// Maybe validate functions are not needed because the DBMS will complain with "invalid syntax" or "field not found"
// Maybe create them so that they are always called when changes are made to the virtual database so that a live error tracking can be made in the CLI
// For now they are set to allow unused and I am going to take the first approach

// Maybe create a DatabaseWriter and DatabaseReader trait
// Each will be implemented by two structs:
// A FileWriter / -Reader and a DbmsWriter / -Reader
// The DatabaseHandler would just call the respective struct depending on what the user wants to do
// -------------------------

use sqlx::AnyConnection;
use std::{collections::HashMap, rc::Weak};

// Struct interfacing between DBMS driver and VirtualDatabase
// Later on also interfaces between VirtualDatabase and Save/Load manager
#[derive(Debug)]
pub struct DatabaseHandler {
    database: VirtualDatabase,
    connection: AnyConnection,
}

// Top level instance describing a database that the user wants to create
#[derive(Debug, Default)]
pub struct VirtualDatabase {
    name: String,
    tables: HashMap<String, Table>,
}

// Instance describing a table of the database the user wants to create
#[derive(Debug, Default)]
pub struct Table {
    database: Weak<VirtualDatabase>,
    name: String,
    columns: HashMap<String, Column>,
    primary_keys: Vec<String>,
    foreign_keys: Vec<(String, String)>,
    entry_count: Option<usize>,
}

// Instance describing a column of a table in the database the user wants to create
#[derive(Debug, Default)]
pub struct Column {
    name: String,
    datatype: Datatype,
    size: u16,
    default: String,
    auto_increment: bool,
    not_null: bool,
}

// Supported SQL datatypes that a Column can have
#[derive(Debug)]
pub enum Datatype {
    Int,
    Float,
    Bool,
    Varchar,
    Date,
    Time,
    Datetime,
    Year,
    Enum(Vec<String>),
    Set(Vec<String>),
}

impl Default for Datatype {
    fn default() -> Self {
        Datatype::Int
    }
}

impl DatabaseHandler {
    // Creates a new DatabaseHandler taking ownership of the VirtualDatabase and the Connection
    pub fn new(database: VirtualDatabase, connection: AnyConnection) -> Self {
        DatabaseHandler {
            database,
            connection,
        }
    }

    // Function to change the VirtualDatabase
    // Will become necesarry when VirtualDatabase can be stored
    pub fn change_database(&mut self, new_database: VirtualDatabase) {
        self.database = new_database;
    }

    // Function that changes the connection
    pub fn change_connection(&mut self, new_connection: AnyConnection) {
        self.connection = new_connection;
    }

    // Function that connects to a DBMS and creates a new database from the VirtualDatabase
    pub fn create_database(&self) -> Result<(), sqlx::Error> {
        // Depending on version will start the validation before attempting to create the database using connection
        todo!()
    }

    // Function that iterates over every table in the database and looks how many entries should be generated if any
    fn fill_database(&self) {
        todo!()
    }

    #[allow(unused)]
    fn validate(&self) {
        todo!()
    }
}

impl VirtualDatabase {
    #[allow(unused)]
    fn validate(&self) {
        todo!();
    }
}

impl Table {
    #[allow(unused)]
    fn validate(&self) {
        todo!();
    }
}

impl Column {
    #[allow(unused)]
    fn validate(&self) {
        todo!();
    }
}
