#!/usr/bin/env bash

 search_dir=./doc-thumbnails-generator/thumbnails_repo/
 for entry in "$search_dir"/*
 do
   echo "$entry"
   cargo run -- --json-file-path $entry
 done

 read -p "Press any key to continue"