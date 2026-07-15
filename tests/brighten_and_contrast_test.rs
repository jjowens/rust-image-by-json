mod shared;

#[cfg(test)]
mod brighten_and_contrast_test {
    use crate::shared;

    #[test]
    fn brighten() {
       let _ = shared::run_json_instructions("test-json/brighten-and-contrast/brighten.json");
    }

    #[test]
    fn constrast() {
        let _ = shared::run_json_instructions("test-json/brighten-and-contrast/contrast.json");
    }

    #[test]
    fn brighten_and_contrast() {
        let _ = shared::run_json_instructions("test-json/brighten-and-contrast/brighten_and_contrast.json");
    }

    #[test]
    fn brighten_and_contrast_reverse() {
        let _ = shared::run_json_instructions("test-json/brighten-and-contrast/brighten_and_contrast_reverse.json");
    }

}