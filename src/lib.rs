// <><><><><><><><><><><><><><><><><><><><><><><><><>
// Thoughts about the library structure
// <><><><><><><><><><><><><><><><><><><><><><><><><>

// ------------------------------
// Controller - Systems handling data processing and IO to a DBMS or filesystem
// ------------------------------
// Enum : TargetDBMS
// Variants
//     -> MySQL, PostgreSQL, Oracle, MSSQL, SQLite?, Redis? (mariaDb and phpMyAdmin are MySQL dbms)

// Enum : TargetFileType
// Variants
//     -> Yaml, Json, Xml (Xml support as very last task if at all)

// Trait : VdbWriter
// Specialized traits:
//   - VdbDatabaseWriter
//   - VdbFileWriter
// Implementations
//     -> MySQLWriter, PostgreSQLWriter, YamlWriter, ...
// Description:
//     Writes the state of the virtual database state to its target.
//     VdbDatabasewriter will create a new database (with data inserts if datasets exists) on the connected DBMS from the state.
//     VdbFileWriter will store the state in a text file like .yaml.

// Trait : VdbReader
// Specialized traits:
//   - VdbDatabaseReader
//   - VdbFileReader
// Implementations
//     -> MySQLReader, PostgreSQLReader, YamlReader, ...
// Description:
//     Read data from its source and creates state from it.
//     VdbDatabasewriter will read the database schema and runs select statements to get the datasets to create the state.
//     VdbFileWriter will read in a text file like .yaml to create the state.

// DatasetExporter
// Implementations -> YamlDatasetExporter, CsvDatasetExporter, JsonDatasetExporter, XmlDatasetExporter (Xml support as very last task if at all)
// Description:
//     Takes the datasets of a table and stores them in a text file like .yaml or .csv
//     This is mainly meant for the case the user is altering a table that contains datasets already.
//     Because it can't be guaranteed that the existing datasets fit the new table structure.
//     This system will backup the previous datasets to the disk.

// Trait : Validator
// Implemenmtations
//     -> MySQLValidator, PostgreSQLValidator, YamlValidator
// Description:
//     Provides functions to validate the states in this library.
//     The validator fills ErrorList and WarningList.
//     An error would be using a keyword for a table name or when creating a key to reference a column that doesn't exist.
//     A warning would be to start a column name with a number since this is allowed in SQL but needs special handling.

// Trait : DatabaseImporter
// Implementations
//     -> MySQLImporter, PostgreSQLImporter
// Description:
//     Connects to a DBMS and creates a database by importing a .sql file (or other files depending on the DBMS).

// Trait : DatabaseExporter
// Implementations
//     -> MySQLExporter, PostgreSQLExporter
// Description:
//     Connects to a database and exports the database into a .sql file (or other files depending on the DBMS).

// DummyDataGenerator
// Description:
//     Takes a table struct and generates appropriate random datasets for it.

// ------------------------------
// Manager - Systems handling data storage at runtime (in process memory, e.g Vec, HashMap)
// ------------------------------

// Struct: VirtualDatabase
// Fields
//     -> database_name
//     -> tables
//     -> users
// Description:
//     Struct representing a database in process memory

// Struct: User
// Fields
//     -> user_name
//     -> password
//     -> host
//     -> user_type
// Description:
//     Struct representing a user for the database and their non default setting.
//     Maybe special user needed for eachtype of dbms.

// Struct: Table
// Fields
//     -> table_name
//     -> columns
//     -> primary_keys (contains list of unique column names)
//     -> foreign_keys (contains list of unique combinations of table names and column names)
//     -> indices (contains list of unique column names)
//     -> datasets (Vec of Vecs of Datatype, 2d Vec)
// Description:
//     Struct representing a table in the virtual database.

// Struct: Column
// Fields
//     -> column_name
//     -> datatype (enum)
//     -> default
//     -> auto_increment
//     -> not_null
// Description:
//     Struct representing a column of a table in the virtual database.

// Enum: Datatype
// Variants
//     -> Int(i8)
//     -> Int(i16)
//     -> Int(i32)
//     -> Int(i64)
//     -> Float(f32)
//     -> Double(f64)
//     -> Boolean(bool)
//     -> Varchar(String, u16)
//     -> Date(Date)
//     -> Time(Time)
//     -> DateTime(DateTime)
//     -> Year(u16)
//     -> Enum(Vec<String>)
//     -> Set(Vec<String>)
// Description:
//     All supported datatypes for a column and a place to contain their value.

// ConnectionSettings
// Fields
//     -> target_dbms
//     -> user_name
//     -> password
//     -> address
// Description:
//     Necessary data to build the connection string sqlx uses to connect to a given dbms.

// FilesystemSettings
// Fields
//     -> filepath_vdb
//     -> filepath_db
//     -> filepath_datasets
//     -> file_type_vdb
//     -> file_type_datasets
// Description:
//     Necessary data to write files to the correct location and in the correct format
//     as well as to correctly read and parse them.

// Struct: ErrorList
// Fields
//     -> errors
// Description:
//     Struct providing access to all errors the Validator has found.
//     Currently only an Vec but may become an Observer or a similar pattern the ui apps can listen to.

// Struct: WarningList
// Fields
//     -> warnings
// Description:
//     Struct providing access to all warnings the Validator has found.
//     Currently only an Vec but may become an Observer or a similar pattern the ui apps can listen to.

// <><><><><><><><><><><><><><><><><><><><><><><><><>
// General thoughts for all ui apps using this library
// <><><><><><><><><><><><><><><><><><><><><><><><><>

// Manager
// ------------------------------
// AppState
// Fields
//     -> virtual_database
//     -> connection_settings
//     -> file_system_settings
// Description:
//     Minimalfield requirements what the ui apps have to track to use this library.
//     Storing an instance to the virtual database.
//     Storing target DBMS and similar data using ConnectionSettings.
//     Storing filepaths and types to interface the filesystem using FilesystemSettings.

mod db_mocker;

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn it_works() {
        initalize_logging();
    }

    fn initalize_logging() {
        // Includes Backtrace feature when running the programm
        // Backtrace shows call stack when panic!
        // 0 = disabled (no backtrace)
        // 1 = partial call stack
        // full = full call stack
        std::env::set_var("RUST_BACKTRACE", "0");
        // env::set_var("RUST_BACKTRACE", "1");
        // env::set_var("RUST_BACKTRACE", "full");

        // Sets-up eyre to generate colorful reports on any panic
        color_eyre::install().expect("Failed to initalize color_eyre");

        // Setting up Tracing
        // Builder for creating a Subscriber instance
        // A Subscriber is used by Tracing to collect data and log it (e.g. to standard output)
        // First a formatter is specified
        // Then the minimum alert level a debug statement should have to be printed
        // The level represents the verbosity of an statement
        // Available Levels:
        // most verbose (lots of information) > least verbose (little information)
        // TRACE > DEBUG > INFO > Warn > Error
        let subscriber = tracing_subscriber::FmtSubscriber::builder()
            .with_max_level(tracing::Level::TRACE)
            .finish();
        // Set defaults for the subscriber
        tracing::subscriber::set_global_default(subscriber)
            .expect("Failed to initialize default subscriber");
    }
}
