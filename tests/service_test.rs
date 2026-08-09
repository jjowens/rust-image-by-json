mod shared;
#[cfg(test)]
mod service_test {
    use rust_image_by_json::services::image_service;
    use rust_image_by_json::services::models::instruction::Instruction;
    use rust_image_by_json::services::models::config::Config;
    use rust_image_by_json::services::models::process_type::ProcessType;

    #[test]
    fn grayscale_test () {
        let open_file_path = "test-assets/test-images/dog1.png";
        let save_file_path = "test-assets/test-output/image-service/grayscale/dog1_basic_update.png";

        let config: Config = Config::new(Some(open_file_path.to_string()), Some(save_file_path.to_string()));

        let mut instructions: Vec<Instruction> = Vec::new();
        instructions.push(Instruction::new(ProcessType::Grayscale, None));

        let _ = image_service::read_instructions(&open_file_path.to_string(), &save_file_path.to_string(), &instructions, &config);
    }

}