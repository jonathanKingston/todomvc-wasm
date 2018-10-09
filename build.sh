#!/bin/sh

cargo +nightly build --target wasm32-unknown-unknown --verbose --release &&
wasm-bindgen target/wasm32-unknown-unknown/release/todomvc.wasm --out-dir . &&
npm install &&
npm run serve
