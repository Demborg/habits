#!/bin/bash
set -e

pushd frontend
trunk build --release
cp -r dist ../backend/dist
popd

pushd backend
cargo shuttle deploy
