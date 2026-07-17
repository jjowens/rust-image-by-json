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
    fn fast_blur_advanced_radius() {
        let _ = shared::run_json_instructions("test-json/blur/bluradvanced_radius_10.json");
        let _ = shared::run_json_instructions("test-json/blur/bluradvanced_radius_30.json");
        let _ = shared::run_json_instructions("test-json/blur/bluradvanced_radius_60.json");
        let _ = shared::run_json_instructions("test-json/blur/bluradvanced_radius_100.json");
    }

    #[test]
    fn fast_blur_advanced_sigma() {
        let _ = shared::run_json_instructions("test-json/blur/bluradvanced_sigma_10.json");
        let _ = shared::run_json_instructions("test-json/blur/bluradvanced_sigma_50.json");
        let _ = shared::run_json_instructions("test-json/blur/bluradvanced_sigma_100.json");
    }

    #[test]
    fn fast_blur_advanced_smooth() {
        let _ = shared::run_json_instructions("test-json/blur/bluradvanced_smooth_3.json");
        let _ = shared::run_json_instructions("test-json/blur/bluradvanced_smooth_5.json");
        let _ = shared::run_json_instructions("test-json/blur/bluradvanced_smooth_7.json");
    }

    #[test]
    fn fast_blur_advanced_anisotropic() {
        let _ = shared::run_json_instructions("test-json/blur/bluradvanced_anisotropic_10x10.json");
        let _ = shared::run_json_instructions("test-json/blur/bluradvanced_anisotropic_30x30.json");
        let _ = shared::run_json_instructions("test-json/blur/bluradvanced_anisotropic_10x50.json");
        let _ = shared::run_json_instructions("test-json/blur/bluradvanced_anisotropic_10x100.json");
    }

}