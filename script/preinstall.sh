#!/bin/bash
if ! type cargo > /dev/null 2>&1; then
    echo 'please ensure rust installed!';
fi

if ! type wasm-pack > /dev/null 2>&1; then
    echo 'installing wasm-pack';
    cargo install wasm-pack
fi