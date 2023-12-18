use bevy::prelude::*;

use super::TileType;

#[derive(Resource)]
pub struct CharsetAsset {
    pub atlas: Handle<TextureAtlas>,
}

#[derive(Resource)]
pub struct Map {
    pub tiles: Vec<TileType>,
}