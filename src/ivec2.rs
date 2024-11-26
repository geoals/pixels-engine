use std::ops::{Add, AddAssign, Mul, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct IVec2 {
    pub x: i64,
    pub y: i64,
}

impl IVec2 {
    pub const ZERO: Self = Self { x: 0, y: 0 };

    pub fn new(x: i64, y: i64) -> Self {
        IVec2 { x, y }
    }
}

impl Add for IVec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        IVec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Mul<i64> for IVec2 {
    type Output = Self;

    fn mul(self, scalar: i64) -> Self {
        IVec2 {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl AddAssign for IVec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for IVec2 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        IVec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
