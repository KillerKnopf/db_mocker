use anyhow::Result;
use async_trait::async_trait;
use sqlx::Connection;

use super::{Table, User, VirtualDatabase};

// pub mod ms_sql;
pub mod my_sql;
// pub mod pg_sql;
// pub mod sql_lite;

#[async_trait]
pub trait DbWriter {
    async fn create_database(
        conn_settings: &ConnectionSettings,
        virtual_db: &VirtualDatabase,
    ) -> Result<()>;

    async fn create_database_v2<T: Connection>(
        connection: &mut T,
        database_name: &str,
    ) -> Result<()>;

    async fn create_table<T: Connection>(connection: &mut T, database_name: &Table) -> Result<()>;

    async fn create_user<T: Connection>(connection: &mut T, user: &User) -> Result<()>;
}

#[async_trait]
pub trait DbReader {
    async fn get_databases<T: Connection>(connection: &mut T) -> Result<()>;

    async fn get_database_schema<T: Connection>(
        connection: &mut T,
        database_name: &str,
    ) -> Result<()>;

    async fn get_tables<T: Connection>(connection: &mut T) -> Result<()>;

    async fn get_table_schema<T: Connection>(connection: &mut T, table_name: &str) -> Result<()>;
}

pub enum DbmsType {
    MySQL,
    // PostgreSQl,
    // MsSQL,
    // SQLLite,
}

pub struct ConnectionSettings {
    pub target_dbms: DbmsType,
    pub user_name: String,
    pub password: String,
    pub address: String,
}
