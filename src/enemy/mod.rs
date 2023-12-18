use bevy::prelude::*;

use crate::{map_generator::{Map, viewshed::Viewshed}, common::{resources::CharsetAsset, components::Position, Vec2Int, WIDTH, HEIGHT, states::GameState}, player::Player};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::PlayerTurn), spawn_enemies)
            .add_systems(Update, handle_enemies.run_if(in_state(GameState::EnemyTurn)));
    }
}

#[derive(Component)]
struct Enemy;

fn spawn_enemies(
    map: Res<Map>,
    atlas: Res<CharsetAsset>,
    mut commands: Commands,
) {
    for (id, room) in map.rooms.iter().skip(1).enumerate() {
        let (x, y) = room.center();

        commands
            .spawn(SpriteSheetBundle {
                texture_atlas: atlas.atlas.clone(),
                sprite: TextureAtlasSprite {
                    custom_size: Some(Vec2::new(1.0, 1.0)),
                    index: 'g' as usize,
                    color: Color::rgb(1.0, 0.0, 0.0),
                    ..Default::default()
                },
                transform: Transform::from_scale(Vec3::new(WIDTH, HEIGHT, 1.0)),
                ..Default::default()
            })
            .insert(Name::from(format!("Enemy {}", id)))
            .insert(Position(Vec2Int::new(x, y)))
            .insert(Viewshed {range: 8.0})
            .insert(Enemy);
    }
}

fn handle_enemies(
    map: Res<Map>,
    mut state: ResMut<NextState<GameState>>,
    enemies: Query<(&Name, &Viewshed, &Position), With<Enemy>>,
    players: Query<&Position, With<Player>>
) {
    let Ok(player) = players.get_single() else {
        return;
    };
    for (name, viewshed, position) in &enemies {
        if map.is_visible(position.0, player.0) && position.0.distance(&player.0) < viewshed.range {
            debug!("{} considers their own existence", name);
        }
    }

    state.set(GameState::PlayerTurn);
}