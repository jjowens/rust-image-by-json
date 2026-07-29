mod shared;
#[cfg(test)]
mod save_as_test {
    use crate::shared;

    #[test]
    fn save_as_single_image() {
       let _ = shared::run_json_instructions("test-json/save-format/basic.json");
    }

    #[test]
    fn save_as_batch() {
        let _ = shared::run_json_instructions("test-json/save-format/basic_batch.json").unwrap();
    }

}