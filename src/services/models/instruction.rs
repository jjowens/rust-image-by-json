use serde::{Deserialize, Serialize};
use crate::services::models::blur_type::BlurType;
use crate::services::models::process_type::ProcessType;

#[derive(Serialize, Deserialize)]
pub struct Instruction {
    pub(crate) process: ProcessType,
    pub(crate) value: Option<String>,
    pub(crate) blurtype: Option<BlurType>
}