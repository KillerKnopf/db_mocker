use chrono::{DateTime, NaiveDate, NaiveTime};
use chrono_tz::Tz;

use super::db_handling::{ConnectionSettings, DbReader};

pub struct VirtualDatabase {
    pub database_name: String,
    pub users: Vec<User>,
    pub tables: Vec<Table>,
}

impl VirtualDatabase {
    pub fn from_db(connection_settings: &ConnectionSettings) {
        super::db_handling::MySQLReader::create_virtual_database_(connection_settings);
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
    pub indices: Vec<String>,
    pub datasets: Vec<Vec<DataType>>,
}

pub struct Column {
    pub column_name: String,
    pub datatype: DataType,
    pub default: String,
    pub auto_increment: bool,
    pub not_null: bool,
}

pub struct ForeignKey {
    pub ref_table: String,
    pub ref_column: String,
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
