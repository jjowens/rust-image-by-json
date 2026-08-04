mod shared;
#[cfg(test)]
mod workflow_test {
    use crate::shared;

    #[test]
    fn filter_1_example_json() {
       let _ = shared::run_json_instructions("test-assets/test-json/workflow/filter_1_example.json");
       let _ = shared::run_json_instructions("test-assets/test-json/workflow/filter_1_example_batch.json");
    }

    #[test]
    fn filter_2_grayscale_and_contrast_json() {
        let _ = shared::run_json_instructions("test-assets/test-json/workflow/filter_2_grayscale_and_contrast.json");
        let _ = shared::run_json_instructions("test-assets/test-json/workflow/filter_2_grayscale_and_contrast_batch.json");
    }

    #[test]
    fn filter_3_grayscale_and_contrast_json() {
        let _ = shared::run_json_instructions("test-assets/test-json/workflow/filter_3_grayscale_and_contrast.json");
        let _ = shared::run_json_instructions("test-assets/test-json/workflow/filter_3_grayscale_and_contrast_batch.json");
    }

    #[test]
    fn filter_4_hue_rotate_json() {
        let _ = shared::run_json_instructions("test-assets/test-json/workflow/filter_4_hue_rotate.json");
        let _ = shared::run_json_instructions("test-assets/test-json/workflow/filter_4_hue_rotate_batch.json");
    }

    #[test]
    fn filter_5_json() {
        let _ = shared::run_json_instructions("test-assets/test-json/workflow/filter_5.json");
        let _ = shared::run_json_instructions("test-assets/test-json/workflow/filter_5_batch.json");
        let _ = shared::run_json_instructions("test-assets/test-json/workflow/filter_5_in_reverse.json");
        let _ = shared::run_json_instructions("test-assets/test-json/workflow/filter_5_batch_in_reverse.json");
    }

    #[test]
    fn filter_6_json() {
        let _ = shared::run_json_instructions("test-assets/test-json/workflow/filter_6.json");
        let _ = shared::run_json_instructions("test-assets/test-json/workflow/filter_6_batch.json");
        let _ = shared::run_json_instructions("test-assets/test-json/workflow/filter_6_in_reverse.json");
        let _ = shared::run_json_instructions("test-assets/test-json/workflow/filter_6_batch_in_reverse.json");
    }


}