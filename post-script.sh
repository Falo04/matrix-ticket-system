#!/usr/bin/env bash

set -e
# Generating lock file for webserver
cargo update

# Generate package-lock.json for frontend and install packages
pushd frontend
npm update
npm install
popd
