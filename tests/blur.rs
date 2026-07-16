mod shared;
#[cfg(test)]
mod blur_test {
    use crate::shared;

    #[test]
    fn blur_json() {
       let _ = shared::run_json_instructions("test-json/blur/blur.json");
    }

    #[test]
    fn fast_blur_json() {
        let _ = shared::run_json_instructions("test-json/blur/fastblur.json");
    }

    #[test]
    fn fast_blur_advanced() {
        let _ = shared::run_json_instructions("test-json/blur/bluradvanced_10.json");
        let _ = shared::run_json_instructions("test-json/blur/bluradvanced_30.json");
        let _ = shared::run_json_instructions("test-json/blur/bluradvanced_60.json");
        let _ = shared::run_json_instructions("test-json/blur/bluradvanced_100.json");
    }

}