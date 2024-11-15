use crate::input::Input;
use crate::vec2::Vec2;
use crate::{Render, TILE_SIZE};
use image::RgbaImage;
use std::time::Duration;

type Position = Vec2;

impl Position {
    fn tile_coordinate(&self) -> (i32, i32) {
        let x = (self.x / TILE_SIZE as f32).floor() as i32;
        let y = (self.y / TILE_SIZE as f32).floor() as i32;
        (x, y)
    }
}

#[derive(PartialEq, Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn to_vector(&self) -> Vec2 {
        match self {
            Direction::Up => Vec2::new(0.0, -1.0),
            Direction::Down => Vec2::new(0.0, 1.0),
            Direction::Left => Vec2::new(-1.0, 0.0),
            Direction::Right => Vec2::new(1.0, 0.0),
        }
    }

    fn from_vector(vector: (i32, i32)) -> Option<Self> {
        match vector {
            (0, 1) => Some(Direction::Down),
            (0, -1) => Some(Direction::Up),
            (1, 0) => Some(Direction::Right),
            (-1, 0) => Some(Direction::Left),
            _ => None,
        }
    }

    fn axis(&self) -> Axis {
        match self {
            Direction::Up | Direction::Down => Axis::Vertical,
            Direction::Left | Direction::Right => Axis::Horizontal,
        }
    }
}

#[derive(PartialEq, Debug)]
enum Axis {
    Horizontal,
    Vertical,
}

// Pixels per second
const GENGAR_SPEED: f32 = 256.0;

pub struct Gengar {
    pixels: Box<RgbaImage>,
    direction: Direction,
    is_moving: bool,
    position: Vec2,
}

impl Gengar {
    pub fn new() -> Self {
        let image_path = "./assets/gengar-64.png";
        let image = image::open(image_path).unwrap();
        let pixels = Box::new(image.as_rgba8().unwrap().to_owned());

        Self {
            pixels,
            direction: Direction::Down,
            is_moving: false,
            position: Vec2::ZERO,
        }
    }

    fn snap_to_grid(&mut self) {
        self.position.x = (self.position.x / TILE_SIZE as f32).round() * TILE_SIZE as f32;
        self.position.y = (self.position.y / TILE_SIZE as f32).round() * TILE_SIZE as f32;
    }

    fn will_reach_next_tile_in_next_update(&self, delta_time: f32) -> bool {
        let current_tile = self.position.tile_coordinate();
        let movement_vector = self.direction.to_vector();
        let movement_step = movement_vector.mul(GENGAR_SPEED * delta_time);
        let next_position = self.position.add(movement_step);
        let next_tile = next_position.tile_coordinate();

        current_tile != next_tile
    }

    fn apply_movement(&mut self, delta_time: f32) {
        let movement_vector = self.direction.to_vector();
        let movement_step = movement_vector.mul(GENGAR_SPEED * delta_time);
        self.position = self.position.add(movement_step);
    }
}

impl Render for Gengar {
    fn pixels(&self) -> Option<&RgbaImage> {
        Some(self.pixels.as_ref())
    }

    fn position(&self) -> Vec2 {
        self.position
    }

    fn update(&mut self, input: &Input, delta_time: Duration) {
        let delta_time = delta_time.as_secs_f32();

        let should_stop = match self.direction.axis() {
            Axis::Horizontal if input.x() == 0 => true,
            Axis::Vertical if input.y() == 0 => true,
            _ => false,
        };

        if should_stop && self.will_reach_next_tile_in_next_update(delta_time) {
            self.is_moving = false;
            self.snap_to_grid();
        }

        if !self.is_moving {
            if let Some(new_direction) = Direction::from_vector(input.vector()) {
                self.is_moving = true;
                self.direction = new_direction;
            }
        }

        if self.is_moving {
            self.apply_movement(delta_time);
        }
    }
}
