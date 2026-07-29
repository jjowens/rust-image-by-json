use std::path::{Path, PathBuf};
use image::ImageFormat;
use image::imageops::GaussianBlurParameters;
use crate::services::models::blur_type::BlurType;

pub fn get_file_extension_on_image_format(image_format: ImageFormat) -> String {
    let file_extension = match image_format {
        ImageFormat::Tiff => { "tiff" },
        ImageFormat::Png => { "png" },
        ImageFormat::Gif => { "gif" },
        ImageFormat::WebP => { "webp"},
        ImageFormat::Jpeg => { "jpg" },
        ImageFormat::Bmp => { "bmp" },
        ImageFormat::Tga => { "tga" },
        ImageFormat::Ico => { "ico" },
        ImageFormat::Hdr => { "hdr" },
        ImageFormat::Avif => { "avif" }
        _ => { "png" }
    };

    file_extension.to_string()
}

pub fn get_image_format_on_file_extension(file_extension: &String) -> ImageFormat {
    let file_extension = match file_extension.as_str() {
        "tiff" => { ImageFormat::Tiff },
        "png" => { ImageFormat::Png },
        "gif" => { ImageFormat::Gif },
        "webp" => { ImageFormat::WebP},
        "jpeg" => { ImageFormat::Jpeg },
        "jpg" => { ImageFormat::Jpeg },
        "bmp" => { ImageFormat::Bmp },
        "tga" => { ImageFormat::Tga },
        "ico" => { ImageFormat::Ico },
        "hdr" => { ImageFormat::Hdr },
        "avif" => { ImageFormat::Avif }
        _ => { ImageFormat::Png }
    };

    file_extension
}

pub fn get_gaussian_blur(blur_type: &BlurType, val: String) -> GaussianBlurParameters {
    let arr = &val.split(",").collect::<Vec<&str>>();

    let gauss_parameters = match blur_type {
        BlurType::Smooth3 => { GaussianBlurParameters::SMOOTHING_3 },
        BlurType::Smooth5 => { GaussianBlurParameters::SMOOTHING_5 },
        BlurType::Smooth7 => { GaussianBlurParameters::SMOOTHING_7 },
        BlurType::Radius => { GaussianBlurParameters::new_from_radius(cast_str_to_f32(arr[0], 10.0)) },
        BlurType::Sigma => { GaussianBlurParameters::new_from_sigma(cast_str_to_f32(arr[0], 10.0)) },
        BlurType::Kernel => { GaussianBlurParameters::new_from_kernel_size(cast_str_to_f32(arr[0], 10.0)) },
        BlurType::Anisotropic => { GaussianBlurParameters::new_anisotropic_kernel_size(cast_str_to_f32(arr[0], 10.0), cast_str_to_f32(arr[1], 10.0)) }
    };

    gauss_parameters
}

pub fn cast_str_to_f32(val: &str, default_val: f32) -> f32 {
    val.parse::<f32>().unwrap_or_else(|_| default_val)
}

pub fn create_directory_from_file_path(file_path: &str) -> Result<(), String> {
    let path = Path::new(file_path);

    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).unwrap();

    Ok(())
}

pub fn check_save_file_path(save_file_path: &str, save_as_image_format: Option<ImageFormat>) -> String {
    let current_image_format = ImageFormat::from_path(save_file_path).unwrap();

    if save_as_image_format.is_none() {
        save_file_path.to_string()
    } else {
        if save_as_image_format == Some(current_image_format) {
            save_file_path.to_string()
        } else {
            let file_extension = get_file_extension_on_image_format(save_as_image_format.unwrap());
            let mut path = PathBuf::from(save_file_path);
            path.set_extension(file_extension);

            let result = path.to_str().unwrap();

            result.to_string()
        }
    }
}
