use std::{collections::{HashMap, BinaryHeap, VecDeque}, cmp::Ordering};

use bevy::prelude::*;

use crate::{map_generator::{Map, viewshed::Viewshed}, common::{resources::CharsetAsset, components::Position, Vec2Int, WIDTH, HEIGHT, states::GameState, TileType}, player::Player};

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
            if let Some(path) = calculate_path(position.0, player.0, &map) {
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

#[derive(Component, Debug, PartialEq)]
struct Path {
    waypoints: VecDeque<Vec2Int>,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    cost: i32,
    position: Vec2Int,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn calculate_path(start: Vec2Int, target: Vec2Int, map: &Map) -> Option<Path> {
    let mut heads = BinaryHeap::new();
    heads.push(Node {
        cost: 0,
        position: start,
    });

    let mut came_from: HashMap<Vec2Int, Option<Vec2Int>> = HashMap::new();
    came_from.insert(start, None);
    let mut cost_so_far = HashMap::new();
    cost_so_far.insert(start, 0);

    while let Some(head) = heads.pop() {
        if head.position == target {
            break;
        }
        for direction in Vec2Int::DIRECTIONS {
            let next = head.position + direction;
            let idx = map.xy_idx(next.x, next.y);
            if map.tiles[idx] == TileType::Wall {
                continue;
            }
            let new_cost = cost_so_far.get(&head.position).unwrap() + cost(head.position, next);
            if !cost_so_far.contains_key(&next) || new_cost < *cost_so_far.get(&next).unwrap() {
                cost_so_far.insert(next, new_cost);
                let priority = new_cost + next.distance(&target).round() as i32;
                heads.push(Node {
                    cost: priority,
                    position: next,
                });
                came_from.insert(next, Some(head.position));
            }
        }
    }

    let mut waypoints = Vec::new();
    let mut current = target;
    while let Some(previous) = came_from.get(&current) {
        if let Some(p) = previous {
            waypoints.push(current);
            current = *p;
        }
        else {
            break;
        }
    }

    if waypoints.is_empty() {
        return None;
    }
    Some(Path {
        waypoints: waypoints.iter().copied().rev().collect()
    })
}

fn cost(start: Vec2Int, end: Vec2Int) -> i32 {
    0
}

#[test]
fn test_pathfinding() {
    let map = Map::new();
    let start = map.rooms.iter().nth(0).unwrap().center();
    let target = map.rooms.iter().nth(1).unwrap().center();
    let path = calculate_path(Vec2Int::new(start.0, start.1), Vec2Int::new(target.0, target.1), &map);
    println!("start: {:?}, end: {:?}", start, target);
    println!("{:?}", path);
    assert!(path.is_some());
    let dead_end = Vec2Int::new(0, 0);
    let path = calculate_path(Vec2Int::new(start.0, start.1), dead_end, &map);
    assert_eq!(path, None);
}