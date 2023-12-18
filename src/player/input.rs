use bevy::prelude::*;

use crate::{
    common::{components::Position, TileType, Vec2Int},
    map_generator::Map,
};

use super::Player;

pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    map: Res<Map>,
    mut players: Query<&mut Position, With<Player>>,
) {
    for mut position in &mut players {
        let mut direction = Vec2Int::ZERO;
        if keyboard_input.pressed(KeyCode::A) {
            direction = Vec2Int::LEFT;
        } else if keyboard_input.pressed(KeyCode::D) {
            direction = Vec2Int::RIGHT;
        } else if keyboard_input.pressed(KeyCode::W) {
            direction = Vec2Int::UP;
        } else if keyboard_input.pressed(KeyCode::S) {
            direction = Vec2Int::DOWN;
        }
        let new_pos: Vec2Int = direction + position.0;
        let idx = map.xy_idx(new_pos.x, new_pos.y);
        if map.tiles[idx] == TileType::Floor {
            position.0 = new_pos;
        }
    }
}
