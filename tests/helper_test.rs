#[cfg(test)]
mod helper_test {
    use rust_image_by_json::services::helper::create_directory_from_file_path;

    fn create_directory(file_path: &str, expected_directory_path: &str) {
        let _ = create_directory_from_file_path(file_path).unwrap();

        assert!(std::path::Path::new(expected_directory_path).exists());
    }

    #[test]
    fn create_directory_test() {
        create_directory("test-output/new-directory/new-file.png", "test-output/new-directory");
    }

    #[test]
    fn create_subdirectory_test() {
        create_directory("test-output/new-directory/sub-dir/new-file.png", "test-output/new-directory/sub-dir");
    }

    #[test]
    fn create_subdirectories_test() {
        create_directory("test-output/new-directory/sub-dir-1/sub-dir-2/sub-dir-3/new-file.png", "test-output/new-directory/sub-dir-1/sub-dir-2/sub-dir-3");
    }

    #[test]
    fn create_subdirectory_already_exists_test() {
        let file_path = "test-output/new-directory/already-exists/new-file.png";
        let dir_path = "test-output/new-directory/already-exists";

        create_directory(file_path, dir_path);

        // REPEAT STEP TO CHECK IF IT ALREADY EXISTS
        create_directory(file_path, dir_path);

        let file_path = "test-output/new-directory/already-exists/alpha/beta/new-file.png";
        let dir_path = "test-output/new-directory/already-exists/alpha/beta/";

        create_directory(file_path, dir_path);

        // REPEAT STEP TO CHECK IF IT ALREADY EXISTS
        create_directory(file_path, dir_path);
    }


}