mod shared;
#[cfg(test)]
mod basic_test {
    use crate::shared;

    #[test]
    fn rotate_json() {
       let _ = shared::run_json_instructions("test-json/rotate/rotate_90.json");
       let _ = shared::run_json_instructions("test-json/rotate/rotate_180.json");
       let _ = shared::run_json_instructions("test-json/rotate/rotate_270.json");
       let _ = shared::run_json_instructions("test-json/rotate/rotate_360.json");
    }


}