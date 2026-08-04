mod shared;
#[cfg(test)]
mod unsharpen_test {
    use crate::shared;

    #[test]
    fn unsharpen_json() {
       let _ = shared::run_json_instructions("test-json/unsharpen/unsharpen.json");
       let _ = shared::run_json_instructions("test-json/unsharpen/unsharpen_batch.json");
    }

    #[test]
    fn unsharpen_only_json() {
        let _ = shared::run_json_instructions("test-json/unsharpen/unsharpen_20.json");
        let _ = shared::run_json_instructions("test-json/unsharpen/unsharpen_50.json");
        let _ = shared::run_json_instructions("test-json/unsharpen/unsharpen_100.json");
    }

    #[test]
    fn unsharpen_threshold_20_json() {
        let _ = shared::run_json_instructions("test-json/unsharpen/unsharpen_10_threshold_20.json");
        let _ = shared::run_json_instructions("test-json/unsharpen/unsharpen_10_threshold_20_batch.json");
    }

    #[test]
    fn unsharpen_threshold_100_json() {
        let _ = shared::run_json_instructions("test-json/unsharpen/unsharpen_10_threshold_100.json");
        let _ = shared::run_json_instructions("test-json/unsharpen/unsharpen_10_threshold_100_batch.json");
    }

}