use image::DynamicImage;

pub struct ImageProcess {
    current_image: DynamicImage,
}

impl ImageProcess {
    pub fn new(image_file_path: String) -> ImageProcess {
        let img = image::open(image_file_path).unwrap();
        ImageProcess { current_image: img }
    }

    pub fn make_grayscale(&self) -> image::DynamicImage {
        self.current_image.grayscale()
    }

    pub fn hue_rotate(&self, rotate : i32) -> image::DynamicImage {
        self.current_image.huerotate(rotate)
    }

    pub fn save(&self, save_file_path : &str) -> Result<(), String> {
        self.current_image.save(save_file_path).unwrap();

        Ok(())
    }

}