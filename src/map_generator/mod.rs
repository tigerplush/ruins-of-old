use bevy::prelude::*;

use crate::common::{resources::{Map, CharsetAsset}, TileType, states::GameState, WIDTH, HEIGHT, Rect};

pub struct MapGeneratorPlugin;

impl Plugin for MapGeneratorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(GameState::StartScreen), generate_map);
    }
}

fn generate_map(
    atlas: Res<CharsetAsset>,
    mut commands: Commands,
) {
    let tiles = generate_rooms();

    commands.spawn((
            Name::from("Tilemap"),
            TransformBundle::default(),
            InheritedVisibility::default(),
        ))
        .with_children(|parent| {
            for (idx, tile) in tiles.clone().iter().enumerate() {
                let char = match tile {
                    TileType::Floor => '.',
                    TileType::Wall => '#',
                };
                parent.spawn(SpriteSheetBundle {
                    texture_atlas: atlas.atlas.clone(),
                    sprite: TextureAtlasSprite {
                        custom_size: Some(Vec2::new(1.0, 1.0)),
                        index: char as usize,
                        ..Default::default()
                    },
                    transform: Transform::from_scale(Vec3::new(WIDTH, HEIGHT, 1.0)).with_translation(idx_xy(idx)),
                    ..Default::default()
                });
            }
        });
    commands.insert_resource(Map {
        tiles,
    });
}

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * 80) + x as usize
}

fn idx_xy(idx: usize) -> Vec3 {
    let x = idx % 80;
    let y = idx / 80;
    Vec3::new(x as f32 * WIDTH, y as f32 * HEIGHT, 1.0)
}

fn generate_empty_tiles() -> Vec<TileType> {
    let mut tiles = vec![TileType::Floor; 80*50];

    for x in 0..80 {
        tiles[xy_idx(x, 0)] = TileType::Wall;
        tiles[xy_idx(x, 49)] = TileType::Wall;
    }
    for y in 0..50 {
        tiles[xy_idx(0, y)] = TileType::Wall;
        tiles[xy_idx(79, y)] = TileType::Wall;
    }
    tiles
}

fn generate_rooms() -> Vec<TileType> {
    let mut tiles = vec![TileType::Wall; 80*50];

    let room1 = Rect::new(20, 15, 10, 15);
    let room2 = Rect::new(35, 15, 10, 15);
    apply_room_to_map(&room1, &mut tiles);
    apply_room_to_map(&room2, &mut tiles);
    tiles
}

fn apply_room_to_map(room: &Rect, map: &mut [TileType]) {
    for y in room.y + 1..= room.y2 {
        for x in room.x + 1..= room.x2 {
            map[xy_idx(x, y)] = TileType::Floor;
        }
    }
}