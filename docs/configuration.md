# Configuration

This will show you can process a single file or batch process a directory of images.

Important note: When batch processing images, it's best not to save those images in the same directory where you are processing
images from. It won't break the application. It's just messy.

## Examples

## Process a single image

| Name    | Data Type | Description                               |
|---------|-----------|-------------------------------------------|
| open_file_path | String    | Set file path to process image            |
| save_file_path   | String    | Save the processed image to new file path |

Configuration: Open a single file and save it in another folder
````
"config": {
    "open_file_path": "test-assets/test-images/dog1.png",
    "save_file_path": "test-assets/test-output/basic/dog1_basic_update.png"
}
````

## Batch process images

| Name    | Data Type | Description                                   |
|---------|-----------|-----------------------------------------------|
| open_directory_path | String    | Set the directory path to process images from |
| save_directory_path   | String    | Save processed images to a new directory path |

Configuration: Open a directory of images, process each one, and then save it in another folder
````
"config": {
    "open_directory_path": "test-assets/test-images/batch",
    "save_directory_path": "test-assets/test-output/basic/batch"
}
````

## Save a single image as in different format

| Name             | Data Type | Description                               |
|------------------|-----------|-------------------------------------------|
| open_file_path   | String    | Set file path to process image            |
| save_file_path   | String    | Save the processed image to new file path |
| save_as_format   | String    | Save image in a different format          |

Configuration: Open a single file and save it as another format. In this configuration, you can open a PNG file and
then save it as JPG. This is not just renaming an image with a different file extension. Rust images sdk formats the image as the
chosen format.

Single Image
````
"config": {
    "open_file_path": "test-assets/test-images/dog1.png",
    "save_file_path": "test-assets/test-output/save-format/basic/dog1.jpg",
    "save_as_format": "jpg"
}
````


## Save images as other format

| Name             | Data Type | Description                                   |
|------------------|-----------|-----------------------------------------------|
| open_directory_path | String    | Set the directory path to process images from |
| save_directory_path | String    | Save processed images to a new directory path |
| save_as_format   | String    | Save images in a different format             |

Configuration: Open a directory of images, batch process them, and then save it as another format. In this configuration, you can open each image and
then save it as a JPG. This is not just renaming an image with a different file extension. Rust images sdk formats the image as the
chosen format.

Batch process images
````
"config": {
    "open_directory_path": "test-assets/test-images/batch",
    "save_directory_path": "test-assets/test-output/save-format/basic/batch",
    "save_as_format": "jpg"
}
````

