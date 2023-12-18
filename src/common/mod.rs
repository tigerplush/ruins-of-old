use bevy::prelude::*;

pub mod components;
pub mod rect;
pub mod resources;
pub mod states;
pub mod vec2int;

pub use rect::*;
pub use vec2int::*;

pub const WIDTH: f32 = 16.0;
pub const HEIGHT: f32 = 16.0;

#[derive(Clone, Copy, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub trait ToWorld {
    fn to_world(&self) -> Vec3;
}

impl ToWorld for (i32, i32) {
    fn to_world(&self) -> Vec3 {
        Vec3::new(self.0 as f32 * WIDTH, self.1 as f32 * HEIGHT, 1.0)
    }
}