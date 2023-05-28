#!/bin/bash
trunk build frontend/index.html -d ./dist --release
cargo shuttle deploy