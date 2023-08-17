use image::RgbaImage;
use crate::{GRID_SIZE, Render};
use crate::input::Input;

pub struct Gengar {
    pixels: Box<RgbaImage>,
    /// Utility value used to determine correct movement when two movement keys are pressed at once
    first_direction: Direction,
    current_direction: Direction,
    pos_x: i32,
    pos_y: i32,
}

#[derive(PartialEq, Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
    None,
}

impl Gengar {
    pub fn new(path: &str, pos_x: i32, pos_y: i32) -> Self {
        let image = image::open(path).unwrap();
        let pixels = Box::new(image.as_rgba8().unwrap().to_owned());
        Self { pixels, pos_x, pos_y, first_direction: Direction::None, current_direction: Direction::None }
    }

    fn is_on_grid(&self) -> bool {
        self.is_on_horizontal_grid() && self.is_on_vertical_grid()
    }

    fn is_on_vertical_grid(&self) -> bool {
        self.pos_x % GRID_SIZE as i32 == 0
    }

    fn is_on_horizontal_grid(&self) -> bool {
        self.pos_y % GRID_SIZE as i32 == 0
    }

    fn set_first_direction(&mut self, input: &Input) {
        if self.first_direction == Direction::None {
            if input.x() == 0 {
                if input.y() == -1 {
                    self.first_direction = Direction::Up;
                }
                if input.y() == 1 {
                    self.first_direction = Direction::Down;
                }
            }
            if input.y() == 0 {
                if input.x() == -1 {
                    self.first_direction = Direction::Left;
                }
                if input.x() == 1 {
                    self.first_direction = Direction::Right;
                }
            }
        }
    }
}

/// Pixels per frame TODO time based instead of frame based
const GENGAR_SPEED: i32 = 4;

impl Render for Gengar {
    fn update(&mut self, input: &Input) {
        self.set_first_direction(input);

        if input.y() == 1 && self.is_on_vertical_grid() {
            self.current_direction = Direction::Down;
        }
        if input.y() == -1 && self.is_on_vertical_grid() {
            self.current_direction = Direction::Up;
        }
        if input.x() == 1 && !(self.first_direction == Direction::Right && input.y() != 0) && self.is_on_horizontal_grid() {
            self.current_direction = Direction::Right;
        }
        if input.x() == -1 && !(self.first_direction == Direction::Left && input.y() != 0) && self.is_on_horizontal_grid() {
            self.current_direction = Direction::Left;
        }

        if !input.is_movement_keys_pressed() && self.is_on_grid() {
            self.current_direction = Direction::None;
            self.first_direction = Direction::None;
        }

        match self.current_direction {
            Direction::Left => self.pos_x -= GENGAR_SPEED,
            Direction::Right => self.pos_x += GENGAR_SPEED,
            Direction::Up => self.pos_y -= GENGAR_SPEED,
            Direction::Down => self.pos_y += GENGAR_SPEED,
            Direction::None => (),
        }
    }

    fn pixels(&self) -> Option<&RgbaImage> {
        Some(self.pixels.as_ref())
    }

    fn pos_x(&self) -> i32 {
        self.pos_x
    }

    fn pos_y(&self) -> i32 {
        self.pos_y
    }
}
