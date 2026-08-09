use serde::{Deserialize, Serialize};
use crate::services::models::blur_type::BlurType;
use crate::services::models::instruction;
use crate::services::models::process_type::ProcessType;
use crate::services::models::resize_filter_type::ResizeFilterType;

#[derive(Serialize, Deserialize)]
pub struct Instruction {
    pub(crate) process: ProcessType,
    pub(crate) value: Option<String>,
    pub(crate) blurtype: Option<BlurType>,
    pub(crate) width: Option<String>,
    pub(crate) height: Option<String>,
    pub(crate) resizefiltertype: Option<ResizeFilterType>,
    pub(crate) unsharpenthreshold: Option<String>
}

impl Instruction {
    pub fn new(process: ProcessType, value: Option<String>) -> Instruction {
      let instruction = Instruction {
            process,
            value,
            blurtype: None,
            width: None,
            height: None,
            resizefiltertype: None,
            unsharpenthreshold: None,
      };instruction
    }
}