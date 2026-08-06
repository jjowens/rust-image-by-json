# rust-image-by-json
Written in Rust. Reads JSON file to batch update images

## Cargo Instructions

Example: Load basic.json
````
 cargo run -- --json-file-path ./test-assets/test-json/basic/basic.json
````

Example: Load basic_batch.json
````
 cargo run -- --json-file-path ./test-assets/test-json/basic_batch.json
````

Example: Load basic_grayscale.json
````
 cargo run -- --json-file-path ./test-assets/test-json/basic_grayscale.json
````

Example: Load resize.json
````
 cargo run -- --json-file-path ./test-assets/test-json/resize/resize_50_percent_catmullrom.json
 cargo run -- --json-file-path ./test-assets/test-json/resize/resize_50_percent_gaussian.json
 cargo run -- --json-file-path ./test-assets/test-json/resize/resize_50_percent_lanczos3.json
 cargo run -- --json-file-path ./test-assets/test-json/resize/resize_50_percent_triangle.json
 cargo run -- --json-file-path ./test-assets/test-json/resize/resize_50_percent_nearest.json
````

## Documentation

[Read more...](docs/readme.md)