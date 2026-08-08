mod shared;
#[cfg(test)]
mod bad_json_test {
    use crate::shared;

    #[test]
    fn empty_instructions() {
        let _ = shared::run_json_instructions_and_fail("test-assets/test-json/bad-json/basic_empty_instructions.json");
    }

    #[test]
    fn no_instructions() {
        let _ = shared::run_json_instructions_and_fail("test-assets/test-json/bad-json/basic_no_instructions.json");
    }

    #[test]
    fn file_does_not_exists() {
        let _ = shared::run_json_instructions_and_fail("test-assets/test-json/bad-json/file_does_not_exists.json");
    }

}