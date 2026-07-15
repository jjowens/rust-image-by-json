mod shared;
#[cfg(test)]
mod flip_image_test {
    use crate::shared;

    #[test]
    fn flip_v_test() {
        let _ = shared::run_json_instructions("test-json/flip/flip_v.json");
    }

    #[test]
    fn flip_h_test() {
        let _ = shared::run_json_instructions("test-json/flip/flip_h.json");
    }

    #[test]
    fn flip_h_and_v_test() {
        let _ = shared::run_json_instructions("test-json/flip/flip_h_and_v.json");
    }

    #[test]
    fn flip_h_and_v_batch_test() {
        let _ = shared::run_json_instructions("test-json/flip/flip_h_and_v_batch.json");
    }

}