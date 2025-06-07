wasm-pack build --target web --out-name emartident_rust_wasm --out-dir ./dist/
bunx http-server . -a 0.0.0.0 -p 8080

