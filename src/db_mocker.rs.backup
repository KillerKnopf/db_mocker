#![allow(dead_code)]

use anyhow::{bail, Context, Result as AnyResult};
use sqlx::AnyConnection;
use std::{
    collections::{HashMap, HashSet},
    rc::{Rc, Weak},
};

// mod validator;

// Struct interfacing between DBMS driver and VirtualDatabase
// Later on also interfaces between VirtualDatabase and Save/Load manager
#[derive(Debug)]
pub struct DatabaseManager {
    database: VirtualDatabase,
    connection: AnyConnection,
}

// Top level instance describing a database that the user wants to create
#[derive(Debug, Default)]
pub struct VirtualDatabase {
    name: String,
    pub tables: Vec<Rc<Table>>,
    users: Rc<User>,
}

// Instance to store users for this database
#[derive(Debug, Default)]
struct User {
    name: String,
    password: String,
    is_admin: bool,
}

// Instance describing a table of the database the user wants to create
#[derive(Debug, Default)]
pub struct Table {
    database: Weak<VirtualDatabase>,
    name: String,
    columns: HashMap<String, Column>,
    primary_keys: HashSet<String>,
    foreign_keys: HashSet<(String, String)>,
    entry_count: Option<usize>,
}

// Instance describing a column of a table in the database the user wants to create
#[derive(Debug, Default)]
pub struct Column {
    name: String,
    pub datatype: Datatype,
    default: String,
    auto_increment: bool,
    pub not_null: bool,
}

// Supported SQL datatypes that a Column can have
#[derive(Debug, PartialEq)]
pub enum Datatype {
    Int,
    Float,
    Bool,
    Varchar(u16),
    Date,
    Time,
    Datetime,
    Year,
    Enum(Vec<String>),
    Set(Vec<String>),
}

impl PartialEq for Table {
    fn eq(&self, other: &Self) -> bool {
        self.get_name() == other.get_name()
    }
}

impl Default for Datatype {
    fn default() -> Self {
        Datatype::Int
    }
}

impl DatabaseManager {
    // Creates a new DatabaseHandler taking ownership of the VirtualDatabase and the Connection
    pub fn new(database: VirtualDatabase, connection: AnyConnection) -> Self {
        DatabaseManager {
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
}

impl VirtualDatabase {}

impl User {}

impl Table {
    // primary_keys: HashSet<String>,
    // foreign_keys: HashSet<(String, String)>,
    // entry_count: Option<usize>,

    // Handlers for field 'name'
    pub fn set_name(&mut self, new_name: &str) {
        // !!! needs validation
        self.name = new_name.to_lowercase();
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }

    // Handlers for field 'primary_keys'
    pub fn add_primary_key(&mut self, new_pk: &str) -> AnyResult<()> {
        // Check if this column exists in columns
        // If it doesn not exist return error
        if !self.columns.contains_key(new_pk) {
            bail!("Column {} does not exist", new_pk);
        }
        // Insert new_pk since the column exist
        self.primary_keys.insert(new_pk.to_string());

        Ok(())
    }
    pub fn remove_primary_key(&mut self, old_pk: &str) {
        // If old_pk exists in primary_keys then get its id otherwise return error
        self.primary_keys.remove(old_pk);
        //self.primary_keys.
    }
    pub fn list_primary_keys(&self) -> Vec<&str> {
        self.primary_keys.iter().map(|pk| pk.as_str()).collect()
    }

    // Handlers for field 'foreign_keys'
    pub fn add_foreign_key(&mut self, new_fk: (&str, &str)) -> AnyResult<()> {
        // Check if this column exists in the table (new_fk.0) in it's columns (new_fk.1)
        // If table or column doesn't exist return error

        // Get tables in parent VirtualDatabase
        let parent_tables = &self
            .database
            .upgrade()
            .context("Failure to reach parent VirtualDatabase")?
            .tables;
        // Iterate over the tables
        for table in parent_tables {
            // Check if
            if table.get_name() == new_fk.0 {
                if table.columns.contains_key(new_fk.1) {
                    // Insert new_pk since the column new_fk.1 exists in table new_fk.0
                    self.foreign_keys
                        .insert((new_fk.0.to_string(), new_fk.1.to_string()));
                    return Ok(());
                }
            }
        }
        // Reaching here means either table (new_fk.0) or column (new_fk.1) didn't exist
        // Don't add foreign key and return error
        bail!("Table {} or Column {} do not exist", new_fk.0, new_fk.1);
    }
    pub fn remove_foreign_key(&mut self, old_pk: &str) {
        // If old_pk exists in primary_keys then get its id otherwise return error
        self.primary_keys.remove(old_pk);
        //self.primary_keys.
    }
    pub fn list_foreign_keys(&self) -> Vec<&str> {
        self.primary_keys.iter().map(|pk| pk.as_str()).collect()
    }
}

impl Column {
    // Handlers for field 'name'
    pub fn set_name(&mut self, new_name: &str) {
        // !!! needs validation
        self.name = new_name.to_lowercase();
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }

    // Handlers for field 'default'
    pub fn set_default(&mut self, new_default: &str) {
        // !!! needs validation if default value fits Datatype
        self.default = new_default.to_lowercase();
    }
    pub fn get_default(&self) -> &str {
        &self.default
    }

    // Handlers for field 'auto_increment'
    pub fn set_auto_increment(&mut self, new_auto_increment: bool) {
        // Only integers can be auto incremented
        // Since self.auto_increment starts at false all other Datatypes will never become true
        if self.datatype != Datatype::Int {
            return;
        }
        // Only with a Datatype Int will this statement be executed
        // So no non-Int column will ever be auto_incremented = true
        self.auto_increment = new_auto_increment;
    }
    pub fn get_auto_increment(&self) -> bool {
        self.auto_increment
    }
}
