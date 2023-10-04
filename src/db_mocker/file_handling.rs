pub trait FileReader {
    fn load_vdb_from_file(file_path: &str);
}

pub trait FileWriter {
    fn write_vdb_to_file(file_path: &str);
}

pub enum FileTypeVdb {
    Yaml,
    // Json,
}

pub enum FileTypeDatasets {
    Yaml,
    // Json,
    // Csv,
}

pub struct FileSystemSettings {
    pub filepath_vdb: String,
    pub filepath_db: String,
    pub filepath_datasets: String,
    pub file_type_vdb: FileTypeVdb,
    pub file_type_datasets: FileTypeDatasets,
}

pub struct YamlReader {}

impl FileReader for YamlReader {
    fn load_vdb_from_file(file_path: &str) {}
}
