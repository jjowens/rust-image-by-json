mod shared;

#[cfg(test)]
mod invert_test {
    use crate::shared;

    #[test]
    fn invert_json() {
        let _ = shared::run_json_instructions("test-assets/test-json/invert/invert.json");
    }

    #[test]
    fn invert_batch() {
        let _ = shared::run_json_instructions("test-assets/test-json/invert/invert.json");
    }

    #[test]
    fn invert_combo() {
        let _ = shared::run_json_instructions("test-assets/test-json/invert/invert_combo.json");
    }

}