#![feature(array_chunks)]
use crate::data::Texture;
use data::GameData;
use wasm_bindgen::prelude::*;
use web_sys::console;

mod data;
mod util;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Game {
    data: GameData,
    held_inputs: [u32; 16],
}

#[derive(Clone, Copy, PartialEq)]
struct Ray {
    x: f32,
    y: f32,
}

#[wasm_bindgen]
impl Game {
    pub fn new() -> Self {
        Self {
            data: GameData::default(),
            held_inputs: [0; 16],
        }
    }

    pub fn data(&self) -> GameData {
        self.data.clone()
    }

    pub fn inputs_ptr(&mut self) -> *mut u32 {
        &mut self.held_inputs as *mut _
    }

    fn draw_texture(&self, x: f32, wall_height: f32, texture_pos_x: usize, texture: &Texture) {
        let y_incrementer = (wall_height * 2.0) / texture.height();
        let mut y = self.data.projection_half_height() - wall_height;

        for i in 0..texture.height() as usize {
            let color = match texture {
                Texture::InMemory(texture) => {
                    texture.colors[texture.bitmap[i][texture_pos_x] as usize].clone()
                }
                Texture::File(texture) => {
                    texture.data[texture_pos_x + i * texture.width as usize].to_string()
                }
            };
            util::draw_line(x, y, x, y + (y_incrementer + 0.5), color);
            y += y_incrementer;
        }
    }

    fn handle_input(&mut self) {
        for key in self.held_inputs {
            match key {
                // Up
                38 => {
                    let player_cos =
                        self.data.player_angle.to_radians().cos() * self.data.player_speed_movement;
                    let player_sin =
                        self.data.player_angle.to_radians().sin() * self.data.player_speed_movement;

                    let new_x = self.data.player_x + player_cos;
                    let new_y = self.data.player_y + player_sin;
                    let check_x = (new_x + player_cos * self.data.player_radius).floor() as usize;
                    let check_y = (new_y + player_sin * self.data.player_radius).floor() as usize;

                    if self.data.map[check_y][self.data.player_x.floor() as usize] == 0 {
                        self.data.player_y = new_y;
                    }

                    if self.data.map[self.data.player_y.floor() as usize][check_x] == 0 {
                        self.data.player_x = new_x;
                    }
                }
                // Down
                40 => {
                    let player_cos =
                        self.data.player_angle.to_radians().cos() * self.data.player_speed_movement;
                    let player_sin =
                        self.data.player_angle.to_radians().sin() * self.data.player_speed_movement;

                    let new_x = self.data.player_x - player_cos;
                    let new_y = self.data.player_y - player_sin;
                    let check_x = (new_x - player_cos * self.data.player_radius).floor() as usize;
                    let check_y = (new_y - player_sin * self.data.player_radius).floor() as usize;

                    if self.data.map[check_y][self.data.player_x.floor() as usize] == 0 {
                        self.data.player_y = new_y;
                    }

                    if self.data.map[self.data.player_y.floor() as usize][check_x] == 0 {
                        self.data.player_x = new_x;
                    }
                }
                // Left
                37 => {
                    self.data.player_angle -= self.data.player_speed_rotation;
                    self.data.player_angle %= 360.0;
                }
                // Right
                39 => {
                    self.data.player_angle += self.data.player_speed_rotation;
                    self.data.player_angle %= 360.0;
                }
                _ => {}
            }
        }
    }

    fn ray_casting(&mut self) {
        let data = &self.data;
        let mut ray_angle = data.player_angle - data.player_half_fov();

        for i in 0..data.projection_width() as usize {
            let mut ray = Ray {
                x: data.player_x as f32,
                y: data.player_y as f32,
            };
            let ray_cos = ray_angle.to_radians().cos() / data.raycasting_precision;
            let ray_sin = ray_angle.to_radians().sin() / data.raycasting_precision;

            let mut wall = 0;
            while wall == 0 {
                ray.x += ray_cos;
                ray.y += ray_sin;
                wall = data.map[ray.y.floor() as usize][ray.x.floor() as usize];
            }

            let mut distance = ((data.player_x as f32 - ray.x).powi(2)
                + (data.player_y as f32 - ray.y).powi(2))
            .sqrt();

            // Correct fish-eye effect. adj = cos * hyp
            distance *= (ray_angle - data.player_angle).to_radians().cos();

            let wall_height = f32::floor(data.projection_half_height() / distance);
            let texture = &data.textures[wall as usize - 1];
            let texture_pos_x =
                (texture.width() * (ray.x + ray.y) % texture.width()).floor() as usize;

            let ray_count = i as f32;
            // Sky
            util::draw_line(
                ray_count,
                0.0,
                ray_count,
                data.projection_half_height() - wall_height,
                "black".to_string(),
            );
            // Walls
            self.draw_texture(ray_count, wall_height, texture_pos_x, texture);
            // Floor
            util::draw_line(
                ray_count,
                data.projection_half_height() + wall_height,
                ray_count,
                data.projection_height(),
                "rgb(95, 87, 79)".to_string(),
            );

            ray_angle += data.increment_angle();
        }
    }

    pub fn tick(&mut self) {
        self.handle_input();
        self.ray_casting();
    }
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    // Your code goes here!
    console::log_1(&JsValue::from_str("Hello world!"));

    util::draw_line(1.0, 200.0, 1.0, 50.0, "#000000".to_string());

    Ok(())
}
