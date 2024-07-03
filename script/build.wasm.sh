#!/bin/bash

cd crates/image-wasm
wasm-pack build --release --target web
