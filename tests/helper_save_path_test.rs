#[cfg(test)]
mod helper_save_path_test {
    use image::ImageFormat;
    use rust_image_by_json::services::helper::check_save_file_path;

    fn test_save_path(save_file_path: &str, expected_file_path: &str, default_image_format: Option<ImageFormat>) -> Result<(), String> {
        let result = check_save_file_path(save_file_path, default_image_format);

        assert_eq!(expected_file_path, result);

        Ok(())
    }

    #[test]
    fn should_save_as_png() {
        let _ = test_save_path("test-output/dog1.png", "test-output/dog1.png", Some(ImageFormat::Png));
    }

    #[test]
    fn should_save_incorrect_filename_as_bmp() {
        let _ = test_save_path("test-output/dog1.png", "test-output/dog1.bmp", Some(ImageFormat::Bmp));
    }

    #[test]
    fn should_save_incorrect_filename_as_tiff() {
        let _ = test_save_path("test-output/dog1.png", "test-output/dog1.tiff", Some(ImageFormat::Tiff));
    }

    #[test]
    fn should_not_change_filename() {
        let _ = test_save_path("test-output/dog1.png", "test-output/dog1.png", None);
    }

}
