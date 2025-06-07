wasm-pack build --target web --out-name emartident_rust_wasm --out-dir ./dist/
python -m http.server 8000 --bind 127.0.0.1

