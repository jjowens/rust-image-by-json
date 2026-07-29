mod shared;
#[cfg(test)]
mod save_as_test {
    use crate::shared;

    #[test]
    fn save_as_basic() {
        let _ = shared::run_json_instructions("test-json/save-format/basic.json").unwrap();
        let _ = shared::run_json_instructions("test-json/save-format/basic_batch.json").unwrap();
    }

    #[test]
    fn save_as_png() {
        let _ = shared::run_json_instructions("test-json/save-format/basic_png.json").unwrap();
        let _ = shared::run_json_instructions("test-json/save-format/basic_batch_png.json").unwrap();
    }

    #[test]
    fn save_as_bmp() {
        let _ = shared::run_json_instructions("test-json/save-format/basic_bmp.json").unwrap();
        let _ = shared::run_json_instructions("test-json/save-format/basic_batch_bmp.json").unwrap();
    }

}