mod shared;
#[cfg(test)]
mod experiment_test {
    use crate::shared;

    #[test]
    fn experiment_json() {
       let _ = shared::run_json_instructions("test-json/experiment/experiment.json");
    }


}