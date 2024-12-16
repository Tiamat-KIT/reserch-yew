wasm-pack build --target web --out-name wasm --out-dir ./static --dev
miniserve ./static --index index.html