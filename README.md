# Offline Image Converter

The **Offline Image Converter** allows you to convert between image types directly in your browser without uploading them anywhere.  
It uses the Rust crate [`image`](https://crates.io/crates/image), compiled to WebAssembly.  

Try it here: [https://imageconvert.mhh.dev/](https://imageconvert.mhh.dev/)

## Run Locally

1. Build the WebAssembly files:
   ```bash
   wasm-pack build --target web
   ```
2. Serve the files with any HTTP server, for example:
   ```bash
   python3 -m http.server
   ```