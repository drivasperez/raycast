use wasm_bindgen::prelude::*;
use web_sys::console;

mod util;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[derive(Clone)]
pub struct GameData {
    screen_width: f32,
    screen_height: f32,
    player_fov: f32,

    player_x: f32,
    player_y: f32,
    player_angle: f32,
    raycasting_precision: f32,
    map: &'static [&'static [u8]],

    scale: f32,
}

#[wasm_bindgen]
impl GameData {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn screen_width(&self) -> f32 {
        self.screen_width
    }
    pub fn screen_height(&self) -> f32 {
        self.screen_height
    }

    pub fn half_width(&self) -> f32 {
        self.screen_width / 2.0
    }

    pub fn half_height(&self) -> f32 {
        self.screen_height / 2.0
    }

    pub fn player_half_fov(&self) -> f32 {
        self.player_fov / 2.0
    }

    pub fn increment_angle(&self) -> f32 {
        self.player_fov / self.projection_width()
    }

    pub fn projection_width(&self) -> f32 {
        self.screen_width / self.scale
    }

    pub fn projection_height(&self) -> f32 {
        self.screen_height / self.scale
    }

    pub fn projection_half_width(&self) -> f32 {
        self.projection_width() / 2.0
    }

    pub fn projection_half_height(&self) -> f32 {
        self.projection_height() / 2.0
    }

    pub fn scale(&self) -> f32 {
        self.scale
    }
}

impl Default for GameData {
    fn default() -> Self {
        Self {
            screen_width: 640.0,
            screen_height: 480.0,
            player_fov: 60.0,

            player_x: 2.0,
            player_y: 2.0,
            player_angle: 90.0,
            raycasting_precision: 64.0,
            map: MAP,

            scale: 4.0,
        }
    }
}

const MAP: &[&[u8]] = &[
    &[1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    &[1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    &[1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    &[1, 0, 0, 1, 1, 0, 1, 0, 0, 1],
    &[1, 0, 0, 1, 0, 0, 1, 0, 0, 1],
    &[1, 0, 0, 1, 0, 0, 1, 0, 0, 1],
    &[1, 0, 0, 1, 0, 1, 1, 0, 0, 1],
    &[1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    &[1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    &[1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
];

#[wasm_bindgen]
pub struct Game {
    data: GameData,
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
        }
    }

    pub fn data(&self) -> GameData {
        self.data.clone()
    }

    pub fn move_player(&mut self, x: f32, y: f32) {
        self.data.player_x += x;
        self.data.player_y += y;
    }

    pub fn turn_player(&mut self, deg: f32) {
        self.data.player_angle += deg;
    }

    pub fn ray_casting(&mut self) {
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

            let distance = ((data.player_x as f32 - ray.x).powi(2)
                + (data.player_y as f32 - ray.y).powi(2))
            .sqrt();
            let wall_height = f32::floor(data.projection_half_height() / distance);

            let ray_count = i as f32;
            util::draw_line(
                ray_count,
                0.0,
                ray_count,
                data.projection_half_height() - wall_height,
                "cyan".to_string(),
            );
            util::draw_line(
                ray_count,
                data.projection_half_height() - wall_height,
                ray_count,
                data.projection_half_height() + wall_height,
                "red".to_string(),
            );
            util::draw_line(
                ray_count,
                data.projection_half_height() + wall_height,
                ray_count,
                data.projection_height(),
                "green".to_string(),
            );

            ray_angle += data.increment_angle();
        }
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
