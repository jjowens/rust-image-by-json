use image::ImageFormat;
use crate::services::models::save_file_type::SaveFileType;

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
        SaveFileType::Avif => { ImageFormat::Avif},
        _ => { ImageFormat::Jpeg},
    };
    
    image_file_type
}