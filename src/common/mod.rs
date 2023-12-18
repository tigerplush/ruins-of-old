
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