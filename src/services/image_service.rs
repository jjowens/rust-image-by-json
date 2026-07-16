use std::fs::{read_dir, read_to_string};
use std::path::PathBuf;
use image::codecs::avif::ColorSpace;
use image::ImageFormat;
use image::metadata::Cicp;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use crate::services::helper::{get_image_format, get_save_file_type};

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
    Flip
}

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

#[derive(Serialize, Deserialize)]
pub struct ImageConfig {
    open_file_path: Option<String>,
    save_file_path: Option<String>,
    instructions: Vec<Instruction>,
    open_directory_path: Option<String>,
    save_directory_path: Option<String>,
    save_file_type: Option<SaveFileType>,
}

#[derive(Serialize, Deserialize)]
pub struct Instruction {
    process: ProcessType,
    value: Option<String>,
}

pub fn image_service(json_file_path: String) -> Result<()> {
    let file_contents = read_to_string(json_file_path);
    let json: ImageConfig = serde_json::from_str(&file_contents.unwrap())?;

    if json.open_directory_path.is_some() && json.save_directory_path.is_some() {
        let paths = read_dir(json.open_directory_path.clone().unwrap()).unwrap();

        for path in paths {
            let current_val = path.unwrap();
            let current_file_path = current_val.path().to_str().unwrap().to_string();
            let save_file_name = current_val.file_name().to_str().unwrap().to_string();
            let current_file_type = current_val.path().extension().unwrap().to_str().unwrap().to_string();
            let default_save_file_type = get_save_file_type(&current_file_path);
            let save_file_type = json.save_file_type.unwrap_or(default_save_file_type);

            let mut save_file_path = PathBuf::new();
            save_file_path.push(json.save_directory_path.clone().unwrap());
            save_file_path.push(save_file_name);

            let _ = read_instructions(&current_file_path,
                                      &save_file_path.to_str().unwrap().to_string(),
                                      &json.instructions,
                                      SaveFileType::Png);
        }
    } else {
        let _ = read_instructions(&json.open_file_path.unwrap().to_string(),
                                  &json.save_file_path.unwrap().to_string(),
                                  &json.instructions,
                                  SaveFileType::Png);
    }

    Ok(())
}

pub fn read_instructions(open_file_path: &String, save_file_path: &String, instructions: &Vec<Instruction>, save_file_type: SaveFileType) ->  Result<()>  {
    let mut img = image::open(open_file_path).unwrap();

    for instruction in instructions {
        let current_val = instruction.value.clone().unwrap_or("".to_string());

        match instruction.process {
            ProcessType::HueRotate => {
                let value = current_val.parse::<i32>().unwrap_or(0);

                img = img.huerotate(value);
            },
            ProcessType::Grayscale => {
                img = img.grayscale();
            },
            ProcessType::Brighten => {
                let value = current_val.parse::<i32>().unwrap_or(0);

                img = img.brighten(value);
            },
            ProcessType::Contrast => {
                let value = current_val.parse::<f32>().unwrap_or(0.0);

                img = img.adjust_contrast(value);
            },
            ProcessType::Flip => {
                match current_val.as_str().to_lowercase().as_str() {
                    "h" => {
                        img = img.fliph();
                    },
                    "v" => {
                        img = img.flipv();
                    },
                    _ => {
                        // DO NOTHING
                    }
                }
                
            },
            ProcessType::Rotate => {
                let value = current_val.parse::<i32>().unwrap_or(0);
                
                match value {
                    90 => {
                        img = img.rotate90()
                    }
                    180 => {
                        img = img.rotate180()
                    },
                    270 => {
                        img = img.rotate270()
                    }
                    _ => {
                        // DO NOTHING
                    }
                }
            }
        }
    }

    //img.save(save_file_path).unwrap();

    let image_format = get_image_format(save_file_type);

    img.save_with_format(save_file_path, image_format).unwrap();

    Ok(())
}

