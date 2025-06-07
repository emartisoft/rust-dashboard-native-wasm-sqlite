wasm-pack build --target web --out-name emartident_rust_wasm --out-dir ./dist/
deno run --allow-net --allow-read jsr:@std/http/file-server --addr 0.0.0.0:8000

