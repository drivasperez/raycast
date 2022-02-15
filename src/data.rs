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
    pub(crate) colors: Vec<RgbColor>,
}

#[derive(Clone)]
pub struct FileTexture {
    pub(crate) width: f32,
    pub(crate) height: f32,
    pub(crate) data: Vec<RgbColor>,
}
impl FileTexture {
    fn load_from_id(id: &str, width: f32, height: f32) -> Self {
        let bytes = util::load_texture_data(id.to_string(), width, height);
        let rgb_data: Vec<RgbColor> = bytes.array_chunks::<4>().map(|s| s.into()).collect();

        FileTexture {
            width,
            height,
            data: rgb_data,
        }
    }
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
}

#[derive(Clone, Copy)]
pub struct RgbColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl RgbColor {
    pub fn rgb(red: u8, green: u8, blue: u8) -> Self {
        Self {
            red,
            green,
            blue,
            alpha: 255,
        }
    }
}

impl From<&[u8; 4]> for RgbColor {
    fn from(bytes: &[u8; 4]) -> Self {
        Self {
            red: bytes[0],
            green: bytes[1],
            blue: bytes[2],
            alpha: bytes[3],
        }
    }
}

impl ToString for RgbColor {
    fn to_string(&self) -> String {
        let Self {
            red,
            green,
            blue,
            alpha,
        } = *self;
        format!("rgb({red},{green},{blue},{alpha})")
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
    pub(crate) backgrounds: Vec<FileTexture>,
}

impl Default for GameData {
    fn default() -> Self {
        let texture = Texture::File(FileTexture::load_from_id("texture", 16.0, 16.0));

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
                    colors: vec![RgbColor::rgb(0, 0, 0), RgbColor::rgb(255, 255, 255)],
                }),
                texture,
            ],
            backgrounds: vec![FileTexture::load_from_id("background", 360.0, 60.0)],
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
