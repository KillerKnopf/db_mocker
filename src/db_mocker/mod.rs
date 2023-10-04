#![allow(dead_code)]

use chrono::{DateTime, NaiveDate, NaiveTime};
use chrono_tz::Tz;
use sqlx::Connection;

pub mod db_handling;
pub mod file_handling;

pub enum Target {
    Database(db_handling::ConnectionSettings),
    File(file_handling::FileSystemSettings),
}

pub struct VirtualDatabase {
    pub database_name: String,
    pub users: Vec<User>,
    pub tables: Vec<Table>,
}

impl VirtualDatabase {
    // pub fn from_database(connection_settings: &ConnectionSettings, database_name: &str) -> Self {
    pub fn from_database<T: Connection>(connection: &mut T) {
        // Get schema of the database

        // Get list of tables

        // For each table get it's schema

        // Construct VirtualDatabase instance from the schemas
    }

    // pub fn from_database(connection_settings: &ConnectionSettings, database_name: &str) -> Self {
    pub fn from_file(file_path: &str) {
        // Get content of the database
        // let file_content = file_handling::YamlReader::read_vdb_from_file();

        // Parse VirtualDatabase instance from content
    }
}

pub struct User {
    pub user_name: String,
    pub password: String,
    pub host: String,
    pub user_type: UserType,
}

pub enum UserType {
    Admin,
    CRUD,
}

pub struct Table {
    pub table_name: String,
    pub columns: Vec<Column>,
    pub primary_keys: Vec<String>,
    pub foreign_keys: Vec<ForeignKey>,
    pub uniques: Vec<String>,
    pub indices: Vec<Index>,
    pub datasets: Vec<Vec<DataType>>,
}

pub struct Column {
    pub column_name: String,
    pub datatype: DataType,
    pub default: Option<String>,
    pub auto_increment: bool,
    pub not_null: bool,
}

pub struct ForeignKey {
    pub fk_column: String,
    pub origin_table: String,
    pub origin_column: String,
}

pub struct Index {
    pub columns: Vec<String>,
}

pub enum DataType {
    Int8(i8),
    Uint8(i8),
    Int16(i16),
    Uint16(i16),
    Int32(i32),
    Uint32(i32),
    Int64(i64),
    Uint64(i64),
    Float(f32),
    Double(f64),
    Boolean(bool),
    Varchar(String, u16),
    Date(NaiveDate),
    Time(NaiveTime),
    DateTime(DateTime<Tz>),
    Year(u16),
    Enum(Vec<String>),
    Set(Vec<String>),
}
