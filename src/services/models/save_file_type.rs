use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SaveFileType {
    #[serde(rename = "jpeg")]
    Jpeg,
    #[serde(rename = "jpg")]
    Jpg,
    #[serde(rename = "gif")]
    Gif,
    #[serde(rename = "png")]
    Png,
    #[serde(rename = "tiff")]
    Tiff,
    #[serde(rename = "bmp")]
    Bmp,
    #[serde(rename = "ico")]
    Ico,
    #[serde(rename = "webp")]
    Webp,
    #[serde(rename = "tga")]
    Tga,
    #[serde(rename = "hdr")]
    Hdr,
    #[serde(rename = "avif")]
    Avif
}