mod shared;

#[cfg(test)]
mod grayscale_test {
    use crate::shared;

    #[test]
    fn basic_grayscale_json() {
        let _ = shared::run_json_instructions("test-json/grayscale/grayscale.json");
    }

    #[test]
    fn grayscale_combo() {
        let _ = shared::run_json_instructions("test-json/grayscale/grayscale_combo.json");
    }

    #[test]
    fn grayscale_batch() {
        let _ = shared::run_json_instructions("test-json/grayscale/grayscale_batch.json");
    }

}