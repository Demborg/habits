#!/bin/bash
set -e

pushd frontend
trunk build
cp -r dist ../backend/dist
popd

pushd backend
cargo shuttle run
