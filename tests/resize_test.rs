mod shared;
#[cfg(test)]
mod resize_test {
    use crate::shared;
    #[test]
    fn resize_50_percent_json() {
       let _ = shared::run_json_instructions("test-json/resize/resize_50_percent.json");
       let _ = shared::run_json_instructions("test-json/resize/resize_50_percent_catmullrom.json");
       let _ = shared::run_json_instructions("test-json/resize/resize_50_percent_nearest.json");
       let _ = shared::run_json_instructions("test-json/resize/resize_50_percent_triangle.json");
       let _ = shared::run_json_instructions("test-json/resize/resize_50_percent_gaussian.json");
       let _ = shared::run_json_instructions("test-json/resize/resize_50_percent_lanczos3.json");
    }

}