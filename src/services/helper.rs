use clap::builder::Str;
use image::ImageFormat;
use crate::services::models::save_file_type::SaveFileType;
use image::imageops::GaussianBlurParameters;

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

pub fn get_gaussian_blur(val: String) -> GaussianBlurParameters {
    let arr = &val.split(",").collect::<Vec<&str>>();

    let gauss_paraameters = match arr[0].to_lowercase().as_str() {
        "smooth_3" => { GaussianBlurParameters::SMOOTHING_3 },
        "smooth_5" => { GaussianBlurParameters::SMOOTHING_5 },
        "smooth_7" => { GaussianBlurParameters::SMOOTHING_7 },
        "radius" => { GaussianBlurParameters::new_from_radius(cast_str_to_f32(arr[1], 10.0)) },
        "sigma" => { GaussianBlurParameters::new_from_sigma(cast_str_to_f32(arr[1], 10.0)) },
        "kernel" => { GaussianBlurParameters::new_from_kernel_size(cast_str_to_f32(arr[1], 10.0)) },
        "anisotropic" => { GaussianBlurParameters::new_anisotropic_kernel_size(cast_str_to_f32(arr[1], 10.0), cast_str_to_f32(arr[2], 10.0)) },
        _ =>  { GaussianBlurParameters::SMOOTHING_3 },
    };

    gauss_paraameters
}

pub fn cast_str_to_f32(val: &str, default_val: f32) -> f32 {
    val.parse::<f32>().unwrap_or_else(|_| default_val)
}