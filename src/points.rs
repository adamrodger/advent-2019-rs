pub use crate::compass::{Bearing, Direction};

use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub struct Point2D {
    pub x: i32,
    pub y: i32,
}

impl Point2D {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self::new(0, 0)
    }

    pub fn manhattan_distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    pub fn move_bearing(&self, bearing: &Bearing) -> Self {
        match bearing {
            Bearing::North => Self {
                x: self.x,
                y: self.y - 1,
            },
            Bearing::South => Self {
                x: self.x,
                y: self.y + 1,
            },
            Bearing::East => Self {
                x: self.x + 1,
                y: self.y,
            },
            Bearing::West => Self {
                x: self.x - 1,
                y: self.y,
            },
        }
    }
}

impl Add for Point2D {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Point2D {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl Sub for Point2D {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl SubAssign for Point2D {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }
}
