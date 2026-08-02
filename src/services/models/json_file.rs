use crate::services::models::instruction::Instruction;

use serde::{Deserialize, Serialize};
use crate::services::models::config::Config;
use crate::services::models::instruction_range::InstructionRange;

#[derive(Serialize, Deserialize)]
pub struct JsonFile {
    pub(crate) config: Config,
    pub(crate) instructions: Option<Vec<Instruction>>,
    pub(crate) instruction_range: Option<Vec<InstructionRange>>
}