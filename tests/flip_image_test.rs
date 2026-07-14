#[cfg(test)]
mod flip_image_test {
    use assert_cmd::Command;
    const APP_NAME: &str = "rust-image-by-json";

    fn run_json_instructions(json_file_path: &str) -> Result<(), String> {
        let mut cmd = Command::cargo_bin(APP_NAME).unwrap();

        cmd.arg("--json-file-path").arg(json_file_path);

        let _ = cmd.unwrap();

        Ok(())
    }

    #[test]
    fn flip_v_test() {
       let _ = run_json_instructions("test-json/flip/flip_v.json").unwrap();
    }

    #[test]
    fn flip_h_test() {
        let _ = run_json_instructions("test-json/flip/flip_h.json").unwrap();
    }

    #[test]
    fn flip_h_and_v_test() {
        let _ = run_json_instructions("test-json/flip/flip_h_and_v.json").unwrap();
    }

    #[test]
    fn flip_h_and_v_batch_test() {
        let _ = run_json_instructions("test-json/flip/flip_h_and_v_batch.json").unwrap();
    }

}