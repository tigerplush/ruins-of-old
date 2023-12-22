
use bevy::prelude::*;

use crate::{map_generator::{Map, viewshed::Viewshed}, common::{resources::CharsetAsset, components::Position, Vec2Int, WIDTH, HEIGHT, states::GameState, TileType}, player::Player};

use self::pathfinding::Path;

mod pathfinding;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Setup), spawn_enemies)
            .add_systems(Update, transition_to_plan_state.run_if(in_state(GameState::EnemyTurn)))
            .add_systems(OnEnter(GameState::PlanEnemyTurn), plan_enemy_actions)
            .add_systems(Update, transition_to_act_state.run_if(in_state(GameState::PlanEnemyTurn)))
            .add_systems(OnEnter(GameState::ActEnemyTurn), (
                act_enemy_actions,
                enemy_wander
            ))
            .add_systems(Update, transition_to_player_state.run_if(in_state(GameState::ActEnemyTurn)));
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

fn plan_enemy_actions(
    map: Res<Map>,
    enemies: Query<(&Viewshed, &Position, Entity), With<Enemy>>,
    players: Query<&Position, With<Player>>,
    mut commands: Commands,
) {
    let Ok(player) = players.get_single() else {
        return;
    };
    for (viewshed, position, entity) in &enemies {
        if map.is_visible(position.0, player.0) && position.0.distance(&player.0) < viewshed.range {
            if let Some(path) = Path::calculate(position.0, player.0, &map) {
                commands.entity(entity).insert(path);
            }
        }
    }
}

fn transition_to_plan_state(
    mut state: ResMut<NextState<GameState>>,
) {
    state.set(GameState::PlanEnemyTurn);
}

fn transition_to_act_state(
    mut state: ResMut<NextState<GameState>>,) {
    state.set(GameState::ActEnemyTurn);
}

fn transition_to_player_state(
    mut state: ResMut<NextState<GameState>>,) {
    state.set(GameState::PlayerTurn);
}

fn act_enemy_actions(
    mut enemies: Query<(&mut Position, &mut Path, Entity), With<Enemy>>,
    mut commands: Commands,
) {
    for (mut pos, mut path, entity) in &mut enemies {
        if let Some(point) = path.waypoints.pop_front() {
            pos.0 = point;
        }
        else {
            commands.entity(entity).remove::<Path>();
        }
    }
}

fn enemy_wander(
    map: Res<Map>,
    mut enemies: Query<&mut Position, (With<Enemy>, Without<Path>)>,
) {
    for mut enemy in &mut enemies {
        let next_direction = enemy.0 + Vec2Int::random_direction();
        let idx = map.xy_idx(next_direction.x, next_direction.y);
        if map.tiles[idx] != TileType::Wall {
            enemy.0 = next_direction;
        }
    }
}
