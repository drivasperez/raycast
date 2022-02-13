use crate::util;
use wasm_bindgen::prelude::*;

const MAP: &[&[u8]] = &[
    &[2, 2, 1, 1, 1, 2, 2, 2, 2, 2],
    &[2, 0, 0, 0, 0, 0, 0, 0, 0, 2],
    &[2, 0, 0, 0, 0, 0, 0, 0, 0, 2],
    &[2, 0, 0, 1, 1, 0, 2, 0, 0, 2],
    &[2, 0, 0, 2, 0, 0, 2, 0, 0, 2],
    &[2, 0, 0, 2, 0, 0, 2, 0, 0, 2],
    &[2, 0, 0, 2, 0, 2, 2, 0, 0, 2],
    &[2, 0, 0, 0, 0, 0, 0, 0, 0, 2],
    &[2, 0, 0, 0, 0, 0, 0, 0, 0, 2],
    &[2, 2, 2, 2, 2, 2, 2, 2, 2, 2],
];

#[derive(Clone)]
pub struct InMemoryTexture {
    pub(crate) width: f32,
    pub(crate) height: f32,
    pub(crate) bitmap: &'static [&'static [u8]],
    pub(crate) colors: Vec<String>,
}

#[derive(Clone)]
pub struct FileTexture {
    pub(crate) width: f32,
    pub(crate) height: f32,
    pub(crate) data: Vec<RgbColor>,
}

#[derive(Clone)]
pub enum Texture {
    InMemory(InMemoryTexture),
    File(FileTexture),
}

impl Texture {
    pub fn height(&self) -> f32 {
        match self {
            Texture::InMemory(t) => t.height,
            Texture::File(t) => t.height,
        }
    }

    pub fn width(&self) -> f32 {
        match self {
            Texture::InMemory(t) => t.width,
            Texture::File(t) => t.width,
        }
    }

    fn load_from_id(id: &str, width: f32, height: f32) -> Self {
        let bytes = util::load_texture_data(id.to_string(), width, height);
        let rgb_data: Vec<RgbColor> = bytes.array_chunks::<4>().map(|s| s.into()).collect();

        Self::File(FileTexture {
            width,
            height,
            data: rgb_data,
        })
    }
}

#[derive(Clone)]
pub struct RgbColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl From<&[u8; 4]> for RgbColor {
    fn from(bytes: &[u8; 4]) -> Self {
        Self {
            red: bytes[0],
            green: bytes[1],
            blue: bytes[2],
        }
    }
}

impl ToString for RgbColor {
    fn to_string(&self) -> String {
        let Self { red, green, blue } = *self;
        format!("rgb({red},{green},{blue})")
    }
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
    pub(crate) player_speed_movement: f32,
    pub(crate) player_speed_rotation: f32,
    pub(crate) player_radius: f32,

    pub(crate) scale: f32,
    pub(crate) textures: Vec<Texture>,
}

impl Default for GameData {
    fn default() -> Self {
        let texture = Texture::load_from_id("texture", 16.0, 16.0);

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
            player_speed_movement: 0.05,
            player_speed_rotation: 3.0,
            player_radius: 10.0,
            textures: vec![
                Texture::InMemory(InMemoryTexture {
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
                }),
                texture,
            ],
        }
    }
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
