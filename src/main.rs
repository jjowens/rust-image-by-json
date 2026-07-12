use clap::{Parser};
use rust_image_by_json::services::image_service::image_service;

#[derive(Parser, Debug)]
#[command(name = "myapp", author, version, about, long_about = None)]
struct Args {
    /// Set json file path to load instructions
    #[arg(long)]
    json_file_path: String
}

fn main() {
    println!("Image By JSON");
    println!("Version {}", env!("CARGO_PKG_VERSION"));

    let args = Args::parse();

    println!("JSON file path: {}", args.json_file_path);

    let _ = image_service(args.json_file_path);
}
