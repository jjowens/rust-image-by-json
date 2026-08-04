mod shared;
#[cfg(test)]
mod huerotate_test {
    use crate::shared;

    #[test]
    fn huerotate_json() {
        let _ = shared::run_json_instructions("test-assets/test-json/huerotate/huerotate_10.json");
        let _ = shared::run_json_instructions("test-assets/test-json/huerotate/huerotate_90.json");
        let _ = shared::run_json_instructions("test-assets/test-json/huerotate/huerotate_180.json");
        let _ = shared::run_json_instructions("test-assets/test-json/huerotate/huerotate_270.json");
        let _ = shared::run_json_instructions("test-assets/test-json/huerotate/huerotate_360.json");

        let _ = shared::run_json_instructions("test-assets/test-json/huerotate/huerotate_10_batch.json");
        let _ = shared::run_json_instructions("test-assets/test-json/huerotate/huerotate_90_batch.json");
        let _ = shared::run_json_instructions("test-assets/test-json/huerotate/huerotate_180_batch.json");
        let _ = shared::run_json_instructions("test-assets/test-json/huerotate/huerotate_270_batch.json");
        let _ = shared::run_json_instructions("test-assets/test-json/huerotate/huerotate_360_batch.json");
    }

    #[test]
    fn huerotate_stacked_json() {
        let _ = shared::run_json_instructions("test-assets/test-json/huerotate/huerotate_90_doggreen.json");
        let _ = shared::run_json_instructions("test-assets/test-json/huerotate/huerotate_450.json");
        let _ = shared::run_json_instructions("test-assets/test-json/huerotate/huerotate_stacked.json");
        let _ = shared::run_json_instructions("test-assets/test-json/huerotate/huerotate_stacked.json");
    }
}
