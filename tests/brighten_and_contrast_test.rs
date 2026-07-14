#[cfg(test)]
mod brighten_and_contrast_test {
    use assert_cmd::Command;
    const APP_NAME: &str = "rust-image-by-json";

    fn run_json_instructions(json_file_path: &str) -> Result<(), String> {
        let mut cmd = Command::cargo_bin(APP_NAME).unwrap();

        cmd.arg("--json-file-path").arg(json_file_path);

        let _ = cmd.unwrap();

        Ok(())
    }

    #[test]
    fn brighten() {
       let _ = run_json_instructions("test-json/brighten-and-contrast/brighten.json").unwrap();
    }

    #[test]
    fn constrast() {
        let _ = run_json_instructions("test-json/brighten-and-contrast/contrast.json").unwrap();
    }

    #[test]
    fn brighten_and_contrast() {
        let _ = run_json_instructions("test-json/brighten-and-contrast/brighten_and_contrast.json").unwrap();
    }

    #[test]
    fn brighten_and_contrast_reverse() {
        let _ = run_json_instructions("test-json/brighten-and-contrast/brighten_and_contrast_reverse.json").unwrap();
    }

}