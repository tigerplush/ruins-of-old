use bevy::prelude::*;

use super::Vec2Int;

#[derive(Component, Deref, DerefMut)]
pub struct Position(pub Vec2Int);
