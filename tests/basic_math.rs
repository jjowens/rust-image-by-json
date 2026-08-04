mod shared;
#[cfg(test)]
mod basic_test {
    use crate::shared;

    fn calculate_by_percent(val: u32, expected_result: u32, percent_as_string: String) {
        let percent = percent_as_string.parse::<f32>().unwrap();
        let actual_result: u32 = (val as f32 * (percent / 100.0)).floor() as u32;

        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn should_calculate_width() {
        calculate_by_percent(640, 320, "50".to_string());
    }

    #[test]
    fn should_calculate_height() {
        calculate_by_percent(480, 240, "50".to_string());
    }

}