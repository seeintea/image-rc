#!/bin/bash

cd crates/image_wasm
wasm-pack build --release --target web
