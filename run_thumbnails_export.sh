#!/usr/bin/env bash
#
#cargo run -- --json-file-path ./doc-thumbnails-generator/thumbnails_repo/huerotate_100.json
#cargo run -- --json-file-path ./doc-thumbnails-generator/thumbnails_repo/huerotate_150.json
#cargo run -- --json-file-path ./doc-thumbnails-generator/thumbnails_repo/huerotate_200.json
#cargo run -- --json-file-path ./doc-thumbnails-generator/thumbnails_repo/huerotate_240.json
cargo run -- --json-file-path ./doc-thumbnails-generator/thumbnails_repo/original.json

# search_dir=./doc-thumbnails-generator/thumbnails_repo/
# for entry in "$search_dir"/*
# do
#   echo "$entry"
#   cargo run -- --json-file-path $entry
# done

 read -p "Press any key to continue"