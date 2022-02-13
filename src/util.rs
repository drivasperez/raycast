use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/js/index.js")]
extern "C" {
    pub fn load_texture_data(id: String, width: f32, height: f32) -> Vec<u8>;
}
