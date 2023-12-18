use bevy::prelude::*;

use crate::{common::components::Position, player::Player};

use super::{Tile, Map};

#[derive(Component)]
pub struct Viewshed {
    pub range: f32,
}

#[derive(Component)]
pub struct Visited;

#[derive(Component)]
pub struct InRange;

pub fn check_player_viewshed(
    map: ResMut<Map>,
    players: Query<(&Position, &Viewshed), With<Player>>,
    tiles: Query<(Entity, &Tile)>,
    mut commands: Commands,
) {
    let Ok((player, viewshed)) = players.get_single() else {
        return;
    };

    for (entity, tile) in &tiles {
        if tile.0.distance(player) < viewshed.range && map.is_visible(player.0, tile.0) {
            commands.entity(entity).insert((Visited, InRange));
        }
        else {
            commands.entity(entity).remove::<InRange>();
        }
    }
}

pub fn render_player_viewshed(
    mut in_range: Query<&mut TextureAtlasSprite, With<InRange>>,
    mut visited: Query<&mut TextureAtlasSprite, (With<Visited>, Without<InRange>)>,
) {
    for mut sprite in &mut in_range {
        sprite.color = Color::WHITE;
    }
    for mut sprite in &mut visited {
        sprite.color = Color::GRAY;
    }
}