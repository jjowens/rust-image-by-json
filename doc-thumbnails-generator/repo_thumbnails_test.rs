

// THIS IS SET UP TO CREATE THUMBNAILS FOR REPO DOCUMENTATION
#[cfg(test)]
mod repo_thumbnails_test {
    use std::path::Path;
    use assert_cmd::Command;
    pub fn run_json_instructions(json_file_path: &str) -> Result<(), String> {
        let app_name: &str = "rust-image-by-json";
        let mut cmd = Command::cargo_bin(app_name).unwrap();

        cmd.arg("--json-file-path").arg(json_file_path);

        let _ = cmd.unwrap();

        Ok(())
    }

    #[test]
    fn all_json() {
        let thumbnail_json_dir = "./thumbnails_repo";

        for element in Path::read_dir(Path::new(thumbnail_json_dir)).unwrap() {
            if element.is_ok() {
                let file = element.unwrap();

                if file.file_name().to_str().unwrap().ends_with(".json") {
                    let _ = run_json_instructions(file.path().to_str().unwrap());
                }
            }
        }
    }

}