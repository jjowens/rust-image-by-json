use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProcessType {
    #[serde(rename = "rotate")]
    Rotate,
    #[serde(rename = "huerotate")]
    HueRotate,
    #[serde(rename = "grayscale")]
    Grayscale,
    #[serde(rename = "brighten")]
    Brighten,
    #[serde(rename = "contrast")]
    Contrast,
    #[serde(rename = "flip")]
    Flip,
    #[serde(rename = "blur")]
    Blur,
    #[serde(rename = "fastblur")]
    FastBlur,
    #[serde(rename = "bluradvanced")]
    BlurAdvanced,
}