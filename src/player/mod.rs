use bevy::prelude::*;

use crate::{
    common::{
        components::Position, resources::CharsetAsset, states::GameState, Vec2Int, HEIGHT, WIDTH,
    },
    map_generator::{Map, viewshed::Viewshed},
    MainCamera,
};

use self::input::move_player;

mod input;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Setup), spawn_player)
            .add_systems(PostUpdate, render_camera)
            .add_systems(Update, move_player.run_if(in_state(GameState::PlayerTurn)));
    }
}

fn spawn_player(map: Res<Map>, atlas: Res<CharsetAsset>, mut commands: Commands) {
    let (x, y) = map.rooms.first().unwrap().center();
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: atlas.atlas.clone(),
            sprite: TextureAtlasSprite {
                custom_size: Some(Vec2::new(1.0, 1.0)),
                index: '@' as usize,
                ..Default::default()
            },
            transform: Transform::from_scale(Vec3::new(WIDTH, HEIGHT, 1.0)),
            ..Default::default()
        })
        .insert(Player)
        .insert(Name::from("Player"))
        .insert(Position(Vec2Int::new(x, y)))
        .insert(Viewshed {range: 8.0});
}

fn render_camera(
    players: Query<&Transform, (With<Player>, Without<MainCamera>)>,
    mut cameras: Query<&mut Transform, With<MainCamera>>,
) {
    let Ok(player) = players.get_single() else {
        return;
    };

    for mut camera in &mut cameras {
        camera.translation = player.translation;
    }
}
