use std::ops::Add;

use bevy::prelude::*;

use super::{HEIGHT, WIDTH};

#[derive(Clone, Copy)]
pub struct Vec2Int {
    pub x: i32,
    pub y: i32,
}

impl Vec2Int {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub const ZERO: Self = Self::new(0, 0);
    pub const LEFT: Self = Self::new(-1, 0);
    pub const RIGHT: Self = Self::new(1, 0);
    pub const UP: Self = Self::new(0, 1);
    pub const DOWN: Self = Self::new(0, -1);

    pub const DIRECTIONS: [Self; 4] = [Self::LEFT, Self::RIGHT, Self::UP, Self::DOWN];

    pub fn to_world(&self) -> Vec3 {
        Vec3::new(self.x as f32 * WIDTH, self.y as f32 * HEIGHT, 1.0)
    }

    pub fn distance(&self, other: &Self) -> f32 {
        let x = other.x - self.x;
        let y = other.y - self.y;
        let squared = x.pow(2) + y.pow(2);
        (squared as f32).sqrt()
    }
}

impl Add for Vec2Int {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<&Self> for Vec2Int {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
