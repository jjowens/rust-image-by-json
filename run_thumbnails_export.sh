#!/usr/bin/env bash

cargo run -- --json-file-path ./doc-thumbnails-generator/thumbnails_repo/resize_25_no_measurement.json
cargo run -- --json-file-path ./doc-thumbnails-generator/thumbnails_repo/resize_25_percent.json
cargo run -- --json-file-path ./doc-thumbnails-generator/thumbnails_repo/resize_25_percent_by_height_width.json
cargo run -- --json-file-path ./doc-thumbnails-generator/thumbnails_repo/resize_30_percent_catmullrom.json
cargo run -- --json-file-path ./doc-thumbnails-generator/thumbnails_repo/resize_30_percent_gaussian.json
cargo run -- --json-file-path ./doc-thumbnails-generator/thumbnails_repo/resize_30_percent_lanczos3.json
cargo run -- --json-file-path ./doc-thumbnails-generator/thumbnails_repo/resize_30_percent_nearest.json
cargo run -- --json-file-path ./doc-thumbnails-generator/thumbnails_repo/resize_30_percent_triangle.json
cargo run -- --json-file-path ./doc-thumbnails-generator/thumbnails_repo/resize_50_percent.json
cargo run -- --json-file-path ./doc-thumbnails-generator/thumbnails_repo/resize_varied_percent_height_width.json
cargo run -- --json-file-path ./doc-thumbnails-generator/thumbnails_repo/resize_varied_pixels_height_width.json

# search_dir=./doc-thumbnails-generator/thumbnails_repo/
# for entry in "$search_dir"/*
# do
#   echo "$entry"
#   cargo run -- --json-file-path $entry
# done

 read -p "Press any key to continue"