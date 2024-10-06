use crate::input::Input;
use crate::vec2::Vec2;
use crate::{Movement, Render, TILE_SIZE};
use image::RgbaImage;
use std::time::Duration;

pub struct Gengar {
    pixels: Box<RgbaImage>,
    direction: Option<Direction>,
    is_moving: bool,
    position: Vec2,
}

#[derive(PartialEq, Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

const GRID_TOLERANCE: i32 = TILE_SIZE as i32 / 4; // Adjust this value based on your needs

impl Gengar {
    pub fn new(path: &str, pos_x: f32, pos_y: f32) -> Self {
        let image = image::open(path).unwrap();

        let pixels = Box::new(image.as_rgba8().unwrap().to_owned());
        Self {
            pixels,
            direction: None,
            is_moving: false,
            position: Vec2::new(pos_x, pos_y),
        }
    }

    fn snap_to_grid(&mut self) {
        self.position.x = (self.position.x / TILE_SIZE as f32).round() * TILE_SIZE as f32;
        self.position.y = (self.position.y / TILE_SIZE as f32).round() * TILE_SIZE as f32;
        self.direction = None;
    }

    fn has_reached_next_tile(&self) -> bool {
        // check dirction of movement and determine you are within the grid tolerance, if so return true

        if let Some(direction) = &self.direction {
            match direction {
                Direction::Up => todo!(),
                Direction::Left => todo!(),
                Direction::Right => todo!(),
                Direction::Down => todo!(),
            }
        }

        false
    }
}

// Pixels per second
const GENGAR_SPEED: f32 = 128.0;

impl Movement for Gengar {
    fn update(&mut self, input: &Input, delta_time: Duration) {}
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

        match input.y() {
            1 => {
                self.is_moving = true;
                self.direction = Some(Direction::Down);
            }
            -1 => {
                self.is_moving = true;
                self.direction = Some(Direction::Up);
            }
            _ => {}
        }
        match input.x() {
            1 => {
                self.is_moving = true;
                self.direction = Some(Direction::Right);
            }
            -1 => {
                self.is_moving = true;
                self.direction = Some(Direction::Left);
            }
            _ => {}
        }

        if input.y() == 0 && input.x() == 0 && self.has_reached_next_tile() {
            self.is_moving = false;
            self.direction = None;

            // let tile_boundary = (self.position.x / TILE_SIZE as f32).round() * TILE_SIZE as f32;
            // if (self.position.x - tile_boundary).abs() < 0.1 {
            //     self.position.x = tile_boundary;
            //     self.snap_to_grid();
            // }
            //
            // let tile_boundary = (self.position.y / TILE_SIZE as f32).round() * TILE_SIZE as f32;
            // if (self.position.y - tile_boundary).abs() < 0.1 {
            //     self.position.y = tile_boundary;
            //     self.snap_to_grid();
            // }
        }

        if !self.is_moving {
            return;
        }

        if let Some(direction) = &self.direction {
            let movement_vector = match direction {
                Direction::Up => Vec2::new(0.0, -1.0),
                Direction::Down => Vec2::new(0.0, 1.0),
                Direction::Left => Vec2::new(-1.0, 0.0),
                Direction::Right => Vec2::new(1.0, 0.0),
            };

            let movement_step = movement_vector.mul(GENGAR_SPEED * delta_time);
            self.position = self.position.add(movement_step);
        }
    }
}
