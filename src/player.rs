use crate::input::Input;
use crate::{Movement, Render, GRID_SIZE};
use image::RgbaImage;
use std::time::Duration;

pub struct Gengar {
    pixels: Box<RgbaImage>,
    /// Utility value used to determine correct movement when two movement keys are pressed at once
    direction: Direction,
    is_moving: bool,
    pos_x: f32,
    pos_y: f32,
}

#[derive(PartialEq, Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

const GRID_TOLERANCE: i32 = GRID_SIZE as i32 / 4; // Adjust this value based on your needs

impl Gengar {
    pub fn new(path: &str, pos_x: f32, pos_y: f32) -> Self {
        let image = image::open(path).unwrap();
        let pixels = Box::new(image.as_rgba8().unwrap().to_owned());
        Self {
            pixels,
            pos_x,
            pos_y,
            is_moving: false,
            direction: Direction::Right,
        }
    }

    fn is_on_grid(&self) -> bool {
        self.is_on_vertical_grid() && self.is_on_horizontal_grid()
    }

    fn is_on_vertical_grid(&self) -> bool {
        dbg!(self.pos_x().round());
        (self.pos_x.round() as i32 % GRID_SIZE as i32).abs() < GRID_TOLERANCE
    }

    fn is_on_horizontal_grid(&self) -> bool {
        (self.pos_y.round() as i32 % GRID_SIZE as i32).abs() < GRID_TOLERANCE
    }

    fn has_reached_tile(&self, move_dist: f32) -> bool {
        match self.direction {
            Direction::Left => todo!(),
            Direction::Right => {
                dbg!(self.pos_x);
                dbg!(self.pos_x.floor() + move_dist);
                self.pos_x >= (self.pos_x.floor() + move_dist)
            }
            Direction::Up => todo!(),
            Direction::Down => todo!(),
        }
    }

    fn snap_to_grid(&mut self, move_dist: f32) {
        let grid_x = (self.pos_x / GRID_SIZE as f32).round() * GRID_SIZE as f32;
        let grid_y = (self.pos_y / GRID_SIZE as f32).round() * GRID_SIZE as f32;

        let distance_to_grid_x = (self.pos_x - grid_x).abs();
        let distance_to_grid_y = (self.pos_y - grid_y).abs();

        if distance_to_grid_x <= move_dist {
            self.pos_x = grid_x;
        }
        if distance_to_grid_y <= move_dist {
            self.pos_y = grid_y;
        }
    }

    // fn set_first_direction(&mut self, input: &Input) {
    //     if self.direction == Direction::None {
    //         if input.x() == 0 {
    //             if input.y() == -1 {
    //                 self.direction = Direction::Up;
    //             }
    //             if input.y() == 1 {
    //                 self.direction = Direction::Down;
    //             }
    //         }
    //         if input.y() == 0 {
    //             if input.x() == -1 {
    //                 self.direction = Direction::Left;
    //             }
    //             if input.x() == 1 {
    //                 self.direction = Direction::Right;
    //             }
    //         }
    //     }
    // }
}

// Pixels per second
const GENGAR_SPEED: f32 = 64.0;

impl Movement for Gengar {
    fn update(&mut self, input: &Input, delta_time: Duration) {}
}

impl Render for Gengar {
    fn pixels(&self) -> Option<&RgbaImage> {
        Some(self.pixels.as_ref())
    }

    fn pos_x(&self) -> f32 {
        self.pos_x
    }

    fn pos_y(&self) -> f32 {
        self.pos_y
    }

    fn update(&mut self, input: &Input, delta_time: Duration) {
        let delta_time = delta_time.as_secs_f32();
        let move_dist = movement_distance(delta_time);

        if !self.is_moving {
            match input.y() {
                1 => {
                    self.is_moving = true;
                    self.direction = Direction::Down;
                }
                -1 => {
                    self.is_moving = true;
                    self.direction = Direction::Up;
                }
                _ => {}
            }
            match input.x() {
                1 => {
                    self.is_moving = true;
                    self.direction = Direction::Right;
                }
                -1 => {
                    self.is_moving = true;
                    self.direction = Direction::Left;
                }
                _ => {}
            }
        }

        if input.y() == 0 && input.x() == 0 {
            self.is_moving = false;
        }

        if self.is_moving {
            match self.direction {
                Direction::Left => self.pos_x -= move_dist,
                Direction::Right => self.pos_x += move_dist,
                Direction::Up => self.pos_y -= move_dist,
                Direction::Down => self.pos_y += move_dist,
            }
        }
    }
}

fn movement_distance(delta_time: f32) -> f32 {
    GENGAR_SPEED * delta_time
}
