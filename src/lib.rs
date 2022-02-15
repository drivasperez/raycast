#![feature(array_chunks)]
use crate::data::Texture;
use data::{GameData, RgbColor};
use wasm_bindgen::prelude::*;

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
    screen_buffer: Vec<u8>,
}

#[derive(Clone, Copy, PartialEq)]
struct Ray {
    x: f32,
    y: f32,
}

#[wasm_bindgen]
impl Game {
    pub fn new() -> Self {
        let data = GameData::default();
        let buf_len = 4 * (data.projection_height() * data.projection_width()) as usize;

        Self {
            data,
            held_inputs: [0; 16],
            screen_buffer: vec![125; buf_len],
        }
    }

    pub fn data(&self) -> GameData {
        self.data.clone()
    }

    pub fn inputs_ptr(&mut self) -> *mut u32 {
        &mut self.held_inputs as *mut _
    }

    pub fn screen_buffer_ptr(&self) -> *const u8 {
        self.screen_buffer.as_ptr()
    }

    pub fn screen_buffer_len(&self) -> usize {
        4 * (self.data.projection_height() * self.data.projection_width()) as usize
    }

    fn draw_pixel(&mut self, x: usize, y: usize, color: RgbColor) {
        let offset = 4 * (x + y * self.data.projection_width() as usize);

        // Only actually draw if onscreen.
        if offset < self.screen_buffer_len() {
            self.screen_buffer[offset] = color.red;
            self.screen_buffer[offset + 1] = color.green;
            self.screen_buffer[offset + 2] = color.blue;
            self.screen_buffer[offset + 3] = color.alpha;
        }
    }

    fn draw_line(&mut self, x1: usize, y1: usize, y2: usize, color: RgbColor) {
        for y in y1..y2 {
            self.draw_pixel(x1, y, color)
        }
    }

    fn draw_texture(
        &mut self,
        x: usize,
        wall_height: f32,
        texture_pos_x: usize,
        texture_idx: usize,
    ) {
        let height = self.data.textures[texture_idx].height();
        let y_incrementer = (wall_height * 2.0) / height;
        let mut y = self.data.projection_half_height() - wall_height;

        for i in 0..height as usize {
            let texture = &self.data.textures[texture_idx];
            let color = match texture {
                Texture::InMemory(texture) => {
                    texture.colors[texture.bitmap[i][texture_pos_x] as usize]
                }
                Texture::File(texture) => texture.data[texture_pos_x + i * texture.width as usize],
            };
            self.draw_line(
                x,
                y.floor() as usize,
                (y + (y_incrementer + 0.5)).floor() as usize,
                color,
            );
            y += y_incrementer;
        }
    }

    fn draw_background(&mut self, x: usize, y1: usize, y2: usize, background_idx: usize) {
        let offset = self.data.player_angle + x as f32;
        for y in y1..y2 {
            let background = &self.data.backgrounds[background_idx];
            let texture_x = (offset % background.width).ceil() as usize;
            let texture_y = (y as f32 % background.height).ceil() as usize;

            let color = background.data[texture_x + texture_y * background.width as usize];
            self.draw_pixel(x, y, color);
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
        let GameData {
            player_angle,
            player_x,
            player_y,
            raycasting_precision,
            map,
            ..
        } = self.data;
        let player_half_fov = self.data.player_half_fov();
        let projection_width = self.data.projection_width();
        let projection_height = self.data.projection_height();
        let projection_half_height = self.data.projection_half_height();

        let mut ray_angle = player_angle - player_half_fov;

        for i in 0..projection_width as usize {
            let mut ray = Ray {
                x: player_x as f32,
                y: player_y as f32,
            };
            let ray_cos = ray_angle.to_radians().cos() / raycasting_precision;
            let ray_sin = ray_angle.to_radians().sin() / raycasting_precision;

            let mut wall = 0;
            while wall == 0 {
                ray.x += ray_cos;
                ray.y += ray_sin;
                wall = map[ray.y.floor() as usize][ray.x.floor() as usize];
            }

            let mut distance =
                ((player_x as f32 - ray.x).powi(2) + (player_y as f32 - ray.y).powi(2)).sqrt();

            // Correct fish-eye effect. adj = cos * hyp
            distance *= (ray_angle - player_angle).to_radians().cos();

            let wall_height = f32::floor(projection_half_height / distance);
            let texture = &self.data.textures[wall as usize - 1];
            let texture_pos_x =
                (texture.width() * (ray.x + ray.y) % texture.width()).floor() as usize;

            let ray_count = i;

            // Walls
            self.draw_texture(ray_count, wall_height, texture_pos_x, wall as usize - 1);
            // Sky
            self.draw_background(
                ray_count,
                0,
                (projection_half_height - wall_height).floor() as usize,
                0,
            );
            // Floor
            self.draw_line(
                ray_count,
                (projection_half_height + wall_height).floor() as usize,
                projection_height as usize,
                RgbColor::rgb(95, 87, 79),
            );

            ray_angle += self.data.increment_angle();
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

    Ok(())
}
