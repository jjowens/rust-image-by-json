#[cfg(test)]
mod basic_test {
    use assert_cmd::Command;
    const APP_NAME: &str = "rust-image-by-json";

    fn run_json_instructions(json_file_path: &str) -> Result<(), String> {
        let mut cmd = Command::cargo_bin(APP_NAME).unwrap();

        cmd.arg("--json-file-path").arg(json_file_path);

        let output = cmd.unwrap();

        Ok(())
    }

    #[test]
    fn basic_json() {
       let _ = run_json_instructions("test-json/basic.json").unwrap();
    }

    #[test]
    fn basic_batch_json() {
        let _ = run_json_instructions("test-json/basic_batch.json").unwrap();
    }

}