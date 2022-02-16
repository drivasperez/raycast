use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/js/index.js")]
extern "C" {
    pub fn load_texture_data(id: String, width: u32, height: u32) -> Vec<u8>;

    pub fn set_debug_message(msg: String);
}
