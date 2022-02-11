use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/js/index.js")]
extern "C" {
    pub fn draw_line(x1: f32, y1: f32, x2: f32, y2: f32, css_color: String);

    pub fn clear_screen();

    pub fn set_stroke_style(s: String);
}

/// Cast degrees to radians
pub fn degree_to_radians(degree: f32) -> f32 {
    let pi = std::f32::consts::PI;
    degree * pi / 180.0
}
