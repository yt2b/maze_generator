use rand::seq::SliceRandom;
use std::ops::{Add, Div, Mul};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vector2 {
    pub y: i32,
    pub x: i32,
}

impl Vector2 {
    pub fn new(y: i32, x: i32) -> Self {
        Self { y, x }
    }
}

impl Add for Vector2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Mul<i32> for Vector2 {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Div<i32> for Vector2 {
    type Output = Self;

    fn div(self, rhs: i32) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

pub fn get_random_dirs() -> Vec<Vector2> {
    let mut dirs = vec![UP, DOWN, LEFT, RIGHT];
    dirs.shuffle(&mut rand::thread_rng());
    dirs
}

pub const UP: Vector2 = Vector2 { y: -1, x: 0 };
pub const DOWN: Vector2 = Vector2 { y: 1, x: 0 };
pub const LEFT: Vector2 = Vector2 { y: 0, x: -1 };
pub const RIGHT: Vector2 = Vector2 { y: 0, x: 1 };

#[cfg(test)]
mod tests {
    use crate::vector2::{Vector2, DOWN, RIGHT, UP};

    #[test]
    fn test_vector2() {
        assert_eq!(UP + RIGHT, Vector2::new(-1, 1));
        assert_eq!(UP * 2, Vector2::new(-2, 0));
        assert_eq!((DOWN + DOWN) / 2, Vector2::new(1, 0));
    }
}
