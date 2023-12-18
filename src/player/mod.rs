use bevy::prelude::*;

use crate::{common::{states::GameState, resources::CharsetAsset, WIDTH, HEIGHT, components::Position, Vec2Int}, MainCamera};

use self::input::move_player;

mod input;

pub struct PlayerPlugin;

#[derive(Component)]
struct Player;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnExit(GameState::StartScreen), spawn_player)
            .add_systems(PostUpdate, render_camera)
            .add_systems(Update, move_player.run_if(in_state(GameState::PlayerTurn)));
    }
}

fn spawn_player(
    atlas: Res<CharsetAsset>,
    mut commands: Commands,
) {
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
        .insert(Position(Vec2Int::new(21, 16)));
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