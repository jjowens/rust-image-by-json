use std::fs;
use image::{DynamicImage, GenericImageView};

pub struct ImageProcess {
    current_image: DynamicImage,
    open_file_path: String,
    save_file_path: String,
}

impl ImageProcess {
    pub fn new(open_file_path: String, save_file_path: String) -> ImageProcess {
        fs::copy(open_file_path.clone(), save_file_path.clone()).unwrap();

        let img = image::open(save_file_path.clone()).unwrap();
        ImageProcess { current_image: img, open_file_path, save_file_path }
    }

    pub fn make_grayscale(&self) -> image::DynamicImage {
        self.current_image.grayscale()
    }

    // pub fn hue_rotate(&self, rotate : i32) -> image::DynamicImage {
    //     self.current_image.huerotate(rotate)
    // }

    pub fn hue_rotate(&self, rotate : i32) -> Result<(), String> {
        self.current_image.huerotate(rotate).save(self.save_file_path.clone()).unwrap();

        Ok(())
    }

    pub fn rotate(&self, rotate : i32) -> Result<(), String> {
        self.current_image.rotate90().save(self.save_file_path.clone()).unwrap();

        Ok(())
    }

    pub fn save(&self, save_file_path : &str) -> Result<(), String> {
        self.current_image.save(save_file_path).unwrap();

        Ok(())
    }

}