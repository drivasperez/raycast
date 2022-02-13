use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/js/index.js")]
extern "C" {
    pub fn draw_line(x1: f32, y1: f32, x2: f32, y2: f32, css_color: String);

    pub fn clear_screen();

    pub fn set_stroke_style(s: String);

    pub fn load_texture_data(id: String, width: f32, height: f32) -> Vec<u8>;
}
