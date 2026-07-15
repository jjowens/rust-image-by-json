mod shared;
#[cfg(test)]
mod basic_test {
    use crate::shared;

    #[test]
    fn basic_json() {
       let _ = shared::run_json_instructions("test-json/basic/basic.json");
    }

    #[test]
    fn basic_batch_json() {
        let _ = shared::run_json_instructions("test-json/basic/basic_batch.json").unwrap();
    }

}