use std::fs::read_to_string;
use clap::{Parser};

use serde::{Deserialize, Serialize};
use serde_json::Result;
use rust_image_by_json::services::image_service::image_service;
use rust_image_by_json::services::instruction_service::instruction_service;
use rust_image_by_json::services::models::config::Config;
use rust_image_by_json::services::models::instruction::Instruction;
use rust_image_by_json::services::models::instruction_range::InstructionRange;
use rust_image_by_json::services::range_service::range_service;

#[derive(Parser, Debug)]
#[command(name = "myapp", author, version, about, long_about = None)]
struct Args {
    /// Set json file path to load instructions
    #[arg(long)]
    json_file_path: String
}

#[derive(Serialize, Deserialize)]
pub struct JsonFile {
    pub(crate) config: Config,
    pub(crate) instructions: Option<Vec<Instruction>>,
    pub(crate) instruction_range: Option<Vec<InstructionRange>>
}

fn main() -> Result<()> {
    println!("Image By JSON");
    println!("Version {}", env!("CARGO_PKG_VERSION"));

    let args = Args::parse();

    println!("JSON file path: {}", args.json_file_path);

    let _  = parse_file(&args.json_file_path);

    Ok(())
}

fn parse_file(json_file_path: &String) -> serde_json::Result<()> {
    let file_contents = read_to_string(json_file_path);
    let json_file: JsonFile = serde_json::from_str(&file_contents.unwrap())?;

    if json_file.instructions.is_some() {
        let _ = instruction_service(json_file_path.to_string());
    }

    if json_file.instruction_range.is_some() {
        let _ = range_service(json_file_path.to_string());
    }

    Ok(())
}
