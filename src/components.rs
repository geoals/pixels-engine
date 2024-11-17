use crate::movement_util::Direction;

use crate::vec2::Vec2;

use image::RgbaImage;

pub struct Sprite(pub Box<RgbaImage>);

pub type Position = Vec2;

pub struct Movement {
    pub speed: f32,
    pub direction: Direction,
    pub is_moving: bool,
}

impl Default for Movement {
    fn default() -> Self {
        Self {
            speed: 256.0,
            direction: Direction::Down,
            is_moving: false,
        }
    }
}
