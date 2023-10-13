// Thoughts about this code

// Would it be better to give these Connectors a connection to use
// rather than having these Connectors establish their own connection?
// This may require splitting the functionality into create_database() and create_tables()

// Have create table call private sub function like add_default() for separating concerns

// For Validation:
// -----------------------------------
// Only one auto_increment per table
// varchar default have to be between single quotes ''
// A table can have three types of keys:
//     Pri -> Primary Key
//     Uni -> Unique Key (Index)
//     Mul -> Non unique index (Index where multiple values are possible)

use anyhow::Result;
use async_trait::async_trait;
use sqlx::{mysql::MySqlConnectOptions, ConnectOptions};
use sqlx::{Connection, MySqlConnection};

use super::{ConnectionSettings, DbReader, DbWriter};
use super::{Table, User};

pub struct MySQLConnector {}

#[async_trait]
impl DbWriter for MySQLConnector {
    type C = MySqlConnection;

    // Creates a new database from the passed name on the connected DBMS.
    // Returns a new Connection pointing to the database.
    async fn create_database(
        connection_settings: &mut ConnectionSettings,
        database_name: &str,
    ) -> Result<Self::C> {
        // Create connection to the dbms
        let mut connection = MySqlConnectOptions::new()
            .host(&connection_settings.address)
            .username(&connection_settings.user_name)
            .password(&connection_settings.password)
            .connect()
            .await?;

        // Bind database name to CREATE DATABASE statement and execute the statement
        sqlx::query("CREATE DATABASE ?;")
            .bind(database_name)
            .execute(&mut connection)
            .await?;

        // Close connection and establish a new one to the new database
        connection.close();

        // Update connection address in the ConnectionSettings to point ot the just created database
        // Create character sequence to append to address
        // Consisting of '/' and database_name
        let add_on = "/".to_string() + database_name;
        connection_settings.address.push_str(&add_on);

        // Re-establish connection using the updated address of the ConnectionSettings
        connection = MySqlConnectOptions::new()
            .host(&connection_settings.address)
            .username(&connection_settings.user_name)
            .password(&connection_settings.password)
            .connect()
            .await?;

        // Return the Connection pointing to the new database
        Ok(connection)
    }

    // Creates a new table from the passed Table for the connected database.
    // Takes ownership of the connection and returns it.
    // This is done for consistency purposes so that this function is called/used in the same way as the other functions in DbWriter for MySQLConnector.
    async fn create_table(mut connection: Self::C, table: &Table) -> Result<Self::C> {
        /*
            Structure of CREATE TABLE statement
            Every word beginnig with !!! is data given by the user and needs to be bound to a placeholder for security
            The other words are fixed

            CREATE TABLE !!!tbl_name (
                !!!col_name datatype(!!!size) NOT NULL DEFAULT !!!def_value AUTO_INCREMENT,
                ...
                PRIMARY KEY (!!!col_name_1, ...),
                FOREIGN KEY (!!!col_name) REFERENCES !!!foreign_tbl_name(!!!foreign_col_name),
                ...
                UNIQUE (!!!col_name),
                ...
                INDEX (!!!col_name_1, ...)
                ...
            );
        */

        // Vec to store the information to be bound to the CREATE TABLE statement
        let mut sql_data: Vec<&str> = Vec::new();

        // String to store the statement for creating the table
        let mut sql_statement = String::from("CREATE TABLE ? (");

        // Add table name to data
        sql_data.push(&table.table_name);

        // Add each column's details to sql statement
        for (i, column) in table.columns.iter().enumerate() {
            // For each column add this section
            // col_name datatype(size) NOT NULL DEFAULT def_value AUTO_INCREMENT,

            // Add placholders and data to sql statement and data to be bound
            // Add column name
            sql_statement += "? ";
            sql_data.push(&column.column_name);

            // Add datatype and the size if varchar
            // For varchar add it's size to data
            // For enum and set also add their possible values to the statement and the data
            // Don't add space add the end of this section because it could be the last section
            match &column.datatype {
                crate::db_mocker::DataType::Int8(_) => sql_statement += "TINYINT",
                crate::db_mocker::DataType::Uint8(_) => sql_statement += "TINYINT UNSIGNED",
                crate::db_mocker::DataType::Int16(_) => sql_statement += "SMALLINT",
                crate::db_mocker::DataType::Uint16(_) => sql_statement += "SMALLINT UNSIGNED",
                crate::db_mocker::DataType::Int32(_) => sql_statement += "INT",
                crate::db_mocker::DataType::Uint32(_) => sql_statement += "INT UNSIGNED",
                crate::db_mocker::DataType::Int64(_) => sql_statement += "BIGINT",
                crate::db_mocker::DataType::Uint64(_) => sql_statement += "BIGINT UNSIGNED",
                crate::db_mocker::DataType::Float(_) => sql_statement += "FLOAT",
                crate::db_mocker::DataType::Double(_) => sql_statement += "DOUBLE",
                crate::db_mocker::DataType::Boolean(_) => sql_statement += "BOOLEAN",
                crate::db_mocker::DataType::Varchar(size, _) => {
                    sql_statement += "VARCHAR (?)";
                    sql_data.push(&size);
                }
                crate::db_mocker::DataType::Date(_) => sql_statement += "DATE",
                crate::db_mocker::DataType::Time(_) => sql_statement += "TIME",
                crate::db_mocker::DataType::DateTime(_) => sql_statement += "DATETIME",
                crate::db_mocker::DataType::Year(_) => sql_statement += "YEAR",
                crate::db_mocker::DataType::Enum(values) => {
                    // Start value list
                    sql_statement += "(";
                    // Add each value to sql statement and data
                    for (i, value) in values.iter().enumerate() {
                        sql_statement += "'?'";
                        sql_data.push(value);

                        // For each value that isn't the last add a comma and space as a separator to the next value
                        if i + 1 != values.len() {
                            sql_statement += ", ";
                        }
                    }
                    // End value list
                    sql_statement += ")";
                }
                crate::db_mocker::DataType::Set(values) => {
                    // Start value list
                    sql_statement += "(";
                    // Add each value to sql statement and data
                    for (i, value) in values.iter().enumerate() {
                        sql_statement += "'?'";
                        sql_data.push(value);

                        // For each value that isn't the last add a comma and space as a separator to the next value
                        if i + 1 != values.len() {
                            sql_statement += ", ";
                        }
                    }
                    // End value list
                    sql_statement += ")";
                }
            }

            // Add NOT NULL if needed
            if column.not_null {
                sql_statement += " NOT NULL";
            }

            // Add DEFAULT and the default value if needed
            match &column.default {
                Some(value) => {
                    sql_statement += " DEFAULT ?";
                    sql_data.push(&value);
                }
                None => {}
            }

            // Add AUTO_INCREMENT if needed
            if column.auto_increment {
                sql_statement += " AUTO_INCREMENT";
            }

            // Add ", " if not the last column to separate the next column
            if i + 1 != table.columns.len() {
                sql_statement += ", ";
            }
        }

        // Add primary key to table
        // Check if there is a primary key before starting to generate the primary key section
        // In a proper database each table should have a primary key
        // Eventually a missing priamry key should create a warning but for now nothing will be done
        if table.primary_keys.len() > 0 {
            // Start PRIMARY KEY section of sql statement
            sql_statement += ", PRIMARY KEY (";
            // Add primary key details to sql statement
            for (i, col_name) in table.primary_keys.iter().enumerate() {
                // Add placeholder to sql statement and column name to data
                sql_statement += "?";
                sql_data.push(&col_name);

                // If this column is not the last column of the primary key then add a comma and space to separate the next column
                // This is done so that there is no extra trailing comma if the CREATE TABLE statement ends after the primary key section
                if i + 1 != table.primary_keys.len() {
                    sql_statement += ", ";
                }
            }
            // End PRIMARY KEY section
            sql_statement += ")";
        }

        // Add foreign key details to sql statement
        // For each foreign key one FOREIGN KEY section is created
        // Eventually this should create errors if a non existing primary key gets referenced but for now nothing will be done
        for fk in table.foreign_keys.iter() {
            // Add FOREIGN KEY section to statement
            sql_statement += ", FOREIGN KEY (?) REFERENCES ?(?)";

            // Add data to be bound
            // Add column name of column in this table that has the reference to the foreign primary key
            sql_data.push(&fk.fk_column);
            // Add name of foreign table where the referenced primary key lies
            sql_data.push(&fk.origin_table);
            // Add column name of referenced primary key
            sql_data.push(&fk.origin_column);
        }

        // Add indices to sql statement
        // For each index one INDEX section is created
        // Eventually this should create warnings but for now nothing will be done
        for index in table.indices.iter() {
            // Start INDEX section
            sql_statement += ", INDEX (";
            // Add columns data of this index to sql statement and data
            for (i, column) in index.columns.iter().enumerate() {
                // Add placeholder to sql statement and column name to data
                sql_statement += "?";
                sql_data.push(&column);

                // If this column is not the last column of the index then add a comma and space to separate the next column
                // This is done so that there is no extra trailing comma if the INDEX section ends after this column
                if i + 1 != index.columns.len() {
                    sql_statement += ", ";
                }
            }
            // End INDEX section
            sql_statement += ")";
        }

        // Add unique constraints to sql statement
        // For each unique constraint one UNIQUE section is created
        // Eventually a unique constraint on a non existing column should create an error but for now nothing will be done
        for unique in table.uniques.iter() {
            // Add placeholder to sql statement and column name to data
            sql_statement += ", UNIQUE (?)";
            sql_data.push(&unique);
        }

        // End CREATE TABLE statement
        sql_statement += ");";

        // Statement execution
        // First create QUery
        let mut query = sqlx::query(&sql_statement);

        // Then add all binds by iterating over them
        // Because query doesn't implement copy query needs to be assinged teh result of binding to it
        for element in sql_data.iter() {
            query = query.bind(element);
        }

        // Finally execute the query
        query.execute(&mut connection).await?;

        Ok(connection)
    }

    // Takes a connection and a User to create a new user from the passed User using the connection.
    // Takes ownership of the connection and returns it.
    // This is done because multiple queries have to be executed on this connection.
    async fn create_user(
        mut connection: Self::C,
        user: &User,
        database_name: &str,
    ) -> Result<Self::C> {
        // Create user from User struct
        sqlx::query("CREATE USER '?'@'?' IDENTIFIED BY '?';")
            .bind(&user.user_name)
            .bind(&user.host)
            .bind(&user.password)
            .execute(&mut connection)
            .await?;

        // Grant user rigths depending if it is an admin or not
        match user.user_type {
            // Admin user gets all privileges
            crate::db_mocker::UserType::Admin => {
                sqlx::query("GRANT ALL ON ?.* TO '?'@'?' WITH GRANT OPTION;")
                    .bind(database_name)
                    .bind(&user.user_name)
                    .bind(&user.host)
                    .execute(&mut connection)
                    .await?;
            }
            // Non admin only gets rights to execute CRUD statements
            crate::db_mocker::UserType::CRUD => {
                sqlx::query("GRANT INSERT, SELECT, UPDATE, DELETE ON ?.* TO '?'@'?';")
                    .bind(database_name)
                    .bind(&user.user_name)
                    .bind(&user.host)
                    .execute(&mut connection)
                    .await?;
            }
        }

        Ok(connection)
    }
}

#[async_trait]
impl DbReader for MySQLConnector {
    async fn get_databases<MySqlConnection>(
        connection_settings: &mut MySqlConnection,
    ) -> Result<()> {
        todo!()
    }

    async fn get_database_schema<MySqlConnection>(
        connection_settings: &mut MySqlConnection,
        database_name: &str,
    ) -> Result<()> {
        todo!()
    }

    async fn get_tables<MySqlConnection>(connection_settings: &mut MySqlConnection) -> Result<()> {
        todo!()
    }

    async fn get_table_schema<MySqlConnection>(
        connection_settings: &mut MySqlConnection,
        table_name: &str,
    ) -> Result<()> {
        todo!()
    }
}
