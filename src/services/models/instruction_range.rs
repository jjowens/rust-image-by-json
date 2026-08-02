use serde::{Deserialize, Serialize};
use crate::services::models::process_type::ProcessType;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct InstructionRange {
    pub(crate) process: ProcessType,
    pub(crate) minimum_range: i32,
    pub(crate) maximum_range: i32,
    pub(crate) step_range: Option<StepRange>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StepRange {
    #[serde(rename = "increment")]
    Increment,
    #[serde(rename = "decrement")]
    Decrement,
}