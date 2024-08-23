#!/usr/bin/env bash

if [ -z "$1" ]; then
  echo "Please provide a version number"
  exit 1
fi

echo "setting to version v$1"

sed -i "s/\"version\": \".*\"/\"version\": \"$1\"/" data/current_version.json
sed -i "s/\"version\": \".*\"/\"version\": \"$1\"/" package.json
sed -i "s/\"version\": \".*\"/\"version\": \"$1\"/" src-tauri/tauri.conf.json
sed -i "0,/^version = \".*\"/{s/version = \".*\"/version = \"$1\"/}" src-tauri/Cargo.toml
sed -i "0,/^version = \".*\"/{s/version = \".*\"/version = \"$1\"/}" shared/Cargo.toml

echo "Make sure to update the changelog and data/current_version.json file after running this script."
