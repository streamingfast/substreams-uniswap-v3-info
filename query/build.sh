#!/bin/bash -e

ROOT="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

cd $ROOT
cargo build --target wasm32-wasi --release
