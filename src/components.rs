use crate::movement_util::Direction;

use crate::vec2::Vec2;

use image::RgbaImage;

pub struct Sprite(pub Box<RgbaImage>);

pub struct Position(pub Vec2);

pub struct Movement {
    pub speed: f32,
    pub direction: Direction,
    pub is_moving: bool,
}

impl Movement {
    pub fn default() -> Self {
        Self {
            speed: 256.0,
            direction: Direction::Down,
            is_moving: false,
        }
    }
}
