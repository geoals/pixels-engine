use crate::vec2::Vec2;
use crate::TILE_SIZE;

pub trait PositionExt {
    fn tile_coordinate(&self) -> (i64, i64);
}

impl PositionExt for Vec2 {
    fn tile_coordinate(&self) -> (i64, i64) {
        let x = (self.x / TILE_SIZE as f32).floor() as i64;
        let y = (self.y / TILE_SIZE as f32).floor() as i64;
        (x, y)
    }
}

#[derive(PartialEq, Debug, Default, Copy, Clone)]
pub enum Direction {
    Left,
    Right,
    Up,
    #[default]
    Down,
}

impl Direction {
    pub fn to_vector(&self) -> Vec2 {
        match self {
            Direction::Up => Vec2::new(0.0, -1.0),
            Direction::Down => Vec2::new(0.0, 1.0),
            Direction::Left => Vec2::new(-1.0, 0.0),
            Direction::Right => Vec2::new(1.0, 0.0),
        }
    }

    pub fn from_vector(vector: (i32, i32)) -> Option<Self> {
        match vector {
            (0, 1) => Some(Direction::Down),
            (0, -1) => Some(Direction::Up),
            (1, 0) => Some(Direction::Right),
            (-1, 0) => Some(Direction::Left),
            _ => None,
        }
    }

    pub fn axis(&self) -> Axis {
        match self {
            Direction::Up | Direction::Down => Axis::Vertical,
            Direction::Left | Direction::Right => Axis::Horizontal,
        }
    }

    pub fn x(&self) -> i32 {
        match self {
            Direction::Left => -1,
            Direction::Right => 1,
            _ => 0,
        }
    }

    pub fn y(&self) -> i32 {
        match self {
            Direction::Up => -1,
            Direction::Down => 1,
            _ => 0,
        }
    }

    pub fn from_str(string: &str) -> Result<Self, String> {
        match string {
            "Up" => Ok(Direction::Up),
            "Down" => Ok(Direction::Down),
            "Left" => Ok(Direction::Left),
            "Right" => Ok(Direction::Right),
            _ => Err(format!("Invalid direction: {}", string)),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum Axis {
    Horizontal,
    Vertical,
}
