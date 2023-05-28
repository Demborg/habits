#!/bin/bash
set -e

pushd frontend
trunk build
rm -rf ../backend/dist
cp -r dist ../backend/
popd

pushd backend
cargo shuttle run
