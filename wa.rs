// cargo build --target wasm32-unknown-unknown --release --bin wa

#[no_mangle]
pub extern "C" fn add_one(x: i32) -> i32 {
    x + 1
}
