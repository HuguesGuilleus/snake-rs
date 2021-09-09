# Snake in Rust

A simple snake game in Rust. COmposed with a core lib, an binary for linux and a WebAssmbly.

## WebAssembly compilation

Tools need: an standard Rust installation with cargo, a simple file web server.

```sh
# Install the rust target for web assembly
rustup target add wasm32-unknown-unknown

# Build
cd snake-wasm/
cargo build --target wasm32-unknown-unknown --release
# Create a link to the compiled module.
cd web/
ln -s ln -s ../../target/wasm32-unknown-unknown/release/snake_wasm.wasm snake.wasm

# Run the web server because WebAssmbly usage need it.
python3 -m http.server 8000
# or an other http server
# Then open your browser at localhost:8000
```
