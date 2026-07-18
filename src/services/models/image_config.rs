use crate::services::models::instruction::Instruction;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ImageConfig {
    pub(crate) open_file_path: Option<String>,
    pub(crate) save_file_path: Option<String>,
    pub(crate) instructions: Vec<Instruction>,
    pub(crate) open_directory_path: Option<String>,
    pub(crate) save_directory_path: Option<String>
}