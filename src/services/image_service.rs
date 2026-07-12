use std::fs;
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct ImageConfig {
    open_file_path: String,
    save_file_path: String,
    instructions: Vec<Instruction>
}

#[derive(Serialize, Deserialize)]
struct Instruction {
    process: String,
    value: String,
}

pub fn image_service(json_file_path: String) -> Result<()> {
    let file_contents = fs::read_to_string(json_file_path);
    let json: ImageConfig = serde_json::from_str(&file_contents.unwrap())?;

    let mut img = image::open(json.open_file_path).unwrap();

    for instruction in &json.instructions {
        if instruction.process == "huerotate" {
            let value = instruction.value.parse::<i32>().unwrap();
            img = img.huerotate(value);
        }
        if instruction.process == "rotate" {
            let value = instruction.value.parse::<i32>().unwrap();

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

    img.save(json.save_file_path).unwrap();

    Ok(())
}