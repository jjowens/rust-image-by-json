use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub(crate) open_file_path: Option<String>,
    pub(crate) save_file_path: Option<String>,
    pub(crate) open_directory_path: Option<String>,
    pub(crate) save_directory_path: Option<String>,
    pub(crate) save_as_format: Option<String>
}