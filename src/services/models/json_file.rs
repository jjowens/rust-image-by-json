use crate::services::models::instruction::Instruction;

use serde::{Deserialize, Serialize};
use crate::services::models::config::Config;

#[derive(Serialize, Deserialize)]
pub struct JsonFile {
    pub(crate) config: Config,
    pub(crate) instructions: Vec<Instruction>,
}