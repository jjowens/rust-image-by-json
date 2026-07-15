use assert_cmd::Command;
pub fn run_json_instructions(json_file_path: &str) -> Result<(), String> {
    let app_name: &str = "rust-image-by-json";
    let mut cmd = Command::cargo_bin(app_name).unwrap();

    cmd.arg("--json-file-path").arg(json_file_path);

    let _ = cmd.unwrap();

    Ok(())
}