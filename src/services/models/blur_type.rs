use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BlurType {
    #[serde(rename = "radius")]
    Radius,
    #[serde(rename = "sigma")]
    Sigma,
    #[serde(rename = "smooth_3")]
    Smooth3,
    #[serde(rename = "smooth_5")]
    Smooth5,
    #[serde(rename = "smooth_7")]
    Smooth7,
    #[serde(rename = "anisotropic")]
    Anisotropic,
    #[serde(rename = "kernel")]
    Kernel
}