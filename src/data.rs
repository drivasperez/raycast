use wasm_bindgen::prelude::*;

#[derive(Clone)]
pub struct Texture {
    pub(crate) width: f32,
    pub(crate) height: f32,
    pub(crate) bitmap: &'static [&'static [u8]],
    pub(crate) colors: Vec<String>,
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct GameData {
    pub(crate) screen_width: f32,
    pub(crate) screen_height: f32,
    pub(crate) player_fov: f32,

    pub(crate) player_x: f32,
    pub(crate) player_y: f32,
    pub(crate) player_angle: f32,
    pub(crate) raycasting_precision: f32,
    pub(crate) map: &'static [&'static [u8]],
    pub(crate) held_key: Option<u32>,
    pub(crate) player_speed_movement: f32,
    pub(crate) player_speed_rotation: f32,

    pub(crate) scale: f32,
    pub(crate) textures: Vec<Texture>,
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

    pub fn player_x(&self) -> f32 {
        self.player_x
    }

    pub fn player_y(&self) -> f32 {
        self.player_y
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

            scale: 1.0,
            held_key: None,
            player_speed_movement: 0.5,
            player_speed_rotation: 5.0,
            textures: vec![Texture {
                width: 8.0,
                height: 8.0,
                bitmap: &[
                    &[1, 1, 1, 1, 1, 1, 1, 1],
                    &[0, 0, 0, 1, 0, 0, 0, 1],
                    &[1, 1, 1, 1, 1, 1, 1, 1],
                    &[0, 1, 0, 0, 0, 1, 0, 0],
                    &[1, 1, 1, 1, 1, 1, 1, 1],
                    &[0, 0, 0, 1, 0, 0, 0, 1],
                    &[1, 1, 1, 1, 1, 1, 1, 1],
                    &[0, 1, 0, 0, 0, 1, 0, 0],
                ],
                colors: vec!["brown".to_string(), "orange".to_string()],
            }],
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
