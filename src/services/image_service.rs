use std::fs::{read_dir, read_to_string};
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
pub struct ImageConfig {
    open_file_path: Option<String>,
    save_file_path: Option<String>,
    instructions: Vec<Instruction>,
    open_directory_path: Option<String>,
    save_directory_path: Option<String>
}

#[derive(Serialize, Deserialize)]
pub struct Instruction {
    process: String,
    value: String,
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

            let mut save_file_path = PathBuf::new();
            save_file_path.push(json.save_directory_path.clone().unwrap());
            save_file_path.push(save_file_name);

            let _ = read_instructions(&current_file_path, &save_file_path.to_str().unwrap().to_string(), &json.instructions);
        }
    } else {
        let _ = read_instructions(&json.open_file_path.unwrap().to_string(),
                                  &json.save_file_path.unwrap().to_string(),
                                  &json.instructions);
    }

    Ok(())
}

pub fn read_instructions(open_file_path: &String, save_file_path: &String, instructions: &Vec<Instruction>) ->  Result<()>  {
    let mut img = image::open(open_file_path).unwrap();

    for instruction in instructions {
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

    img.save(save_file_path).unwrap();

    Ok(())
}