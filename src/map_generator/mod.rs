use bevy::prelude::*;

use crate::common::{resources::CharsetAsset, states::GameState, TileType, HEIGHT, WIDTH, Vec2Int, ToWorld};

pub use self::map::Map;
use self::viewshed::{check_player_viewshed, render_player_viewshed};

mod map;
pub mod viewshed;

pub struct MapGeneratorPlugin;

impl Plugin for MapGeneratorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::LoadAssets), generate_map)
            .add_systems(Update, (
                check_player_viewshed,
                render_player_viewshed,
            ));
    }
}

#[derive(Component)]
pub struct Tile(Vec2Int);

fn generate_map(atlas: Res<CharsetAsset>, mut commands: Commands) {
    let map = Map::new();

    commands
        .spawn((
            Name::from("Tilemap"),
            TransformBundle::default(),
            InheritedVisibility::default(),
        ))
        .with_children(|parent| {
            for (idx, tile) in map.tiles.clone().iter().enumerate() {
                let char = match tile {
                    TileType::Floor => '.',
                    TileType::Wall => '#',
                };
                let (x, y) = map.idx_xy(idx);
                parent.spawn(SpriteSheetBundle {
                    texture_atlas: atlas.atlas.clone(),
                    sprite: TextureAtlasSprite {
                        custom_size: Some(Vec2::new(1.0, 1.0)),
                        index: char as usize,
                        color: Color::rgba(0.0, 0.0, 0.0, 0.0),
                        ..Default::default()
                    },
                    transform: Transform::from_scale(Vec3::new(WIDTH, HEIGHT, 1.0))
                        .with_translation((x, y).to_world()),
                    ..Default::default()
                })
                .insert(Tile(Vec2Int::new(x, y)));
            }
        });
    commands.insert_resource(map);
}
