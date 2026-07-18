use std::path::Path;
use image::ImageFormat;
use crate::services::models::save_file_type::SaveFileType;
use image::imageops::GaussianBlurParameters;
use crate::services::models::blur_type::BlurType;

pub fn get_save_file_type(file_extension: &String) -> SaveFileType {
    let save_file_type = match file_extension.as_str() {
        "tiff" => { SaveFileType::Tiff},
        "png" =>  { SaveFileType::Png},
        "gif" => { SaveFileType::Gif},
        "webp" => { SaveFileType::Webp},
        "jpeg" => { SaveFileType::Jpeg},
        "jpg" => { SaveFileType::Jpg},
        "bmp" => { SaveFileType::Bmp},
        "tga" => { SaveFileType::Tga},
        "ico" => { SaveFileType::Ico},
        "hdr" => { SaveFileType::Hdr},
        "avif" => { SaveFileType::Avif},
        _ => { SaveFileType::Jpg}
    };

    save_file_type
}

pub fn get_image_format(save_file_type: SaveFileType) -> ImageFormat {
    let image_file_type = match save_file_type {
        SaveFileType::Tiff => { ImageFormat::Tiff },
        SaveFileType::Png => { ImageFormat::Png},
        SaveFileType::Gif => { ImageFormat::Gif},
        SaveFileType::Webp => { ImageFormat::WebP},
        SaveFileType::Jpeg => { ImageFormat::Jpeg},
        SaveFileType::Jpg => { ImageFormat::Jpeg},
        SaveFileType::Bmp => { ImageFormat::Bmp},
        SaveFileType::Tga => { ImageFormat::Tga},
        SaveFileType::Ico => { ImageFormat::Ico},
        SaveFileType::Hdr => { ImageFormat::Hdr},
        SaveFileType::Avif => { ImageFormat::Avif}
    };
    
    image_file_type
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