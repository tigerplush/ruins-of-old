use bevy::prelude::*;

use super::{WIDTH, HEIGHT, Vec2Int};

#[derive(Component, Deref, DerefMut)]
pub struct Position(pub Vec2Int);
