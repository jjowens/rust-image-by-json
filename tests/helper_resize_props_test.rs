#[cfg(test)]
mod helper_resize_props_test {
    use rust_image_by_json::services::resize_helper::get_resize_properties;
    use rust_image_by_json::services::models::resize_props::ResizeProperties;

    const DEFAULT_IMAGE_WIDTH: u32 = 640;
    const DEFAULT_IMAGE_HEIGHT: u32 = 480;

    fn validate_resize_properties(value: Option<String>, width: Option<String>, height: Option<String>, original_image_width: u32, original_image_height: u32, expected_properties: ResizeProperties) {
        let actual_result = get_resize_properties(value, width, height, original_image_width, original_image_height);

        assert_eq!(actual_result, expected_properties);
    }

    #[test]
    fn create_default_resize_properties_if_no_inputs() {
        let expected_props = ResizeProperties {
            width: DEFAULT_IMAGE_WIDTH,
            height: DEFAULT_IMAGE_HEIGHT
        };

        validate_resize_properties(None, None, None, DEFAULT_IMAGE_WIDTH, DEFAULT_IMAGE_HEIGHT, expected_props);
    }

    #[test]
    fn create_resize_properties_as_100_pixels_width_and_height() {
        let expected_props = ResizeProperties {
            width: 100,
            height: 100
        };

       validate_resize_properties(Some("100px".to_string()), None, None,  DEFAULT_IMAGE_WIDTH, DEFAULT_IMAGE_HEIGHT, expected_props);
    }

    #[test]
    fn create_resize_properties_as_100_pixels_width_and_height_no_pixels_given() {
        let expected_props = ResizeProperties {
            width: 100,
            height: 100
        };

        validate_resize_properties(Some("100".to_string()), None, None,  DEFAULT_IMAGE_WIDTH, DEFAULT_IMAGE_HEIGHT, expected_props);
    }

    #[test]
    fn create_resize_properties_as_50_percent() {
        let expected_props = ResizeProperties {
            width: 320,
            height: 240
        };

        validate_resize_properties(Some("50%".to_string()), None, None,  DEFAULT_IMAGE_WIDTH, DEFAULT_IMAGE_HEIGHT, expected_props);
    }

    #[test]
    fn create_resize_properties_as_exact_with_pixels() {
        let expected_props = ResizeProperties {
            width: 1200,
            height: 300
        };

        validate_resize_properties(None, Some("1200px".to_string()), Some("300px".to_string()),  DEFAULT_IMAGE_WIDTH, DEFAULT_IMAGE_HEIGHT, expected_props);
    }

    #[test]
    fn create_resize_properties_as_exact_without_pixels_percent() {
        let expected_props = ResizeProperties {
            width: 500,
            height: 210
        };

        validate_resize_properties(None, Some("500".to_string()), Some("210".to_string()),  DEFAULT_IMAGE_WIDTH, DEFAULT_IMAGE_HEIGHT, expected_props);
    }

    #[test]
    fn create_resize_properties_as_exact_with_percent() {
        let expected_props = ResizeProperties {
            width: 160,
            height: 120
        };

        validate_resize_properties(None, Some("25%".to_string()), Some("25%".to_string()),  DEFAULT_IMAGE_WIDTH, DEFAULT_IMAGE_HEIGHT, expected_props);
    }

}