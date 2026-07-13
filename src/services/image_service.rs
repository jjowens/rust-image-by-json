use std::fs::{read_dir, read_to_string};
use std::path::PathBuf;
use clap::error::ErrorKind::Format;
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct ImageConfig {
    open_file_path: String,
    save_file_path: String,
    instructions: Vec<Instruction>,
    open_directory_path: Option<String>,
    save_directory_path: Option<String>
}

#[derive(Serialize, Deserialize)]
struct Instruction {
    process: String,
    value: String,
}

pub fn image_service(json_file_path: String) -> Result<()> {
    let file_contents = read_to_string(json_file_path);
    let json: ImageConfig = serde_json::from_str(&file_contents.unwrap())?;

    let mut img = image::open(json.open_file_path.clone()).unwrap();

    if json.open_directory_path.is_some() && json.save_directory_path.is_some() {
        let files = read_dir(json.open_directory_path.clone().unwrap());

        for file in files.unwrap() {
            let current_file_path = file.unwrap().path().to_str();

            let save_file_path : PathBuf = [ json.save_directory_path.clone().unwrap().to_string(), file.unwrap().file_name().into_string().unwrap()].iter().collect();


            let _ = read_instructions(current_file_path, save_file_path, json.instructions);
        }
    } else {
        let _ = read_instructions(json.open_file_path, json.save_file_path, json.instructions);
    }

    Ok(())
}

pub fn read_instructions(open_file_path: String, save_file_path: String, instructions: Vec<Instruction>) ->  Result<()>  {
    let mut img = image::open(open_file_path).unwrap();

    for instruction in &instructions {
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