#[cfg(test)]
mod helper_image_format {
    use image::ImageFormat;

    fn valid_image_format(file_path: &str, expected_image_format: ImageFormat) {
        let parsed = ImageFormat::from_path(file_path);

        if parsed.is_err() {
            assert!(false, "{:?} is not a valid image format!", file_path);
        } else {
            let result : ImageFormat = parsed.unwrap();

            assert_eq!(result, expected_image_format);
        }
    }

    #[test]
    fn get_png_image_format() {
        let file_path = "test-output/dog1.png";
        let expected_image_format = ImageFormat::Png;

        valid_image_format(file_path, expected_image_format);
    }

    #[test]
    fn get_jpg_image_format() {
        let file_path = "test-output/dog1.jpg";
        let expected_image_format = ImageFormat::Jpeg;

        valid_image_format(file_path, expected_image_format);
    }

    #[test]
    fn get_gif_image_format() {
        let file_path = "test-output/dog1.gif";
        let expected_image_format = ImageFormat::Gif;

        valid_image_format(file_path, expected_image_format);
    }

    #[test]
    fn should_fail() {
        let file_path = "test-output/file.txt";

        let parsed = ImageFormat::from_path(file_path);

        if parsed.is_err() {
            assert!(true, "{:?} is not a valid image format!", file_path);
            println!("{:?}", parsed.err().unwrap().to_string());
        }
    }

}