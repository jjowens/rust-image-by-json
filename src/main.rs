use clap::{Parser};
use rust_image_by_json::services::image_service::ImageProcess;

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

    let img : ImageProcess = ImageProcess::new("test-images/dog1.png".to_string(), "test-output/dog1_hue_rotate_copy.png".to_string());

    img.hue_rotate(150);
    img.rotate(150);
    //img.save("test-output/dog1_hue_rotate.png").unwrap();

}
