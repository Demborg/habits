#!/bin/bash
set -e

pushd frontend
trunk build --release
rm -rf ../backend/dist
cp -r dist ../backend/
popd

cargo shuttle deploy --working-directory ./backend
