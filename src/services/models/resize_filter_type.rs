use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ResizeFilterType {
    #[serde(rename = "catmullrom")]
    CatmullRom,
    #[serde(rename = "lanczos3")]
    Lanczos3,
    #[serde(rename = "gaussian")]
    Gaussian,
    #[serde(rename = "nearest")]
    Nearest,
    #[serde(rename = "triangle")]
    Triangle
}