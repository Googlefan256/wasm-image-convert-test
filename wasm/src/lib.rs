use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn guess(buf: Vec<u8>) -> Option<String> {
    lib::guess(buf).map(|fmt| fmt.to_string())
}

#[wasm_bindgen]
pub fn convert(buf: Vec<u8>, fmt: String) -> Option<Vec<u8>> {
    lib::convert(buf, fmt.into())
}
