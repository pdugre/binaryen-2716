use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn test_binaryen() -> Vec<u8> {
    let config = argon2::Config::default();
    argon2::hash_raw(b"password", b"someverylongsalt", &config).unwrap()
}
