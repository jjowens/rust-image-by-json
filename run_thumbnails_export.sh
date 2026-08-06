#!/usr/bin/env bash

cargo run -- --json-file-path ./doc-thumbnails-generator/thumbnails_repo/fastblur_1.json
cargo run -- --json-file-path ./doc-thumbnails-generator/thumbnails_repo/fastblur_10.json
cargo run -- --json-file-path ./doc-thumbnails-generator/thumbnails_repo/fastblur_25.json

# search_dir=./doc-thumbnails-generator/thumbnails_repo/
# for entry in "$search_dir"/*
# do
#   echo "$entry"
#   cargo run -- --json-file-path $entry
# done

 read -p "Press any key to continue"