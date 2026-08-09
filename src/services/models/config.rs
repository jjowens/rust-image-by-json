use serde::{Deserialize, Serialize};
use crate::services::models::instruction::Instruction;
use crate::services::models::process_type::ProcessType;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub(crate) open_file_path: Option<String>,
    pub(crate) save_file_path: Option<String>,
    pub(crate) open_directory_path: Option<String>,
    pub(crate) save_directory_path: Option<String>,
    pub(crate) save_as_format: Option<String>
}

impl Config {

    pub fn new(open_file_path: Option<String>, save_file_path: Option<String>) -> Config {
        let config = Config {
            open_file_path,
            save_file_path,
            open_directory_path: None,
            save_directory_path: None,
            save_as_format: None
        };config
    }
}