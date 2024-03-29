use bevy::{prelude::*, log::LogPlugin};

#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use common::{resources::CharsetAsset, states::GameState};
use enemy::EnemyPlugin;
use map_generator::MapGeneratorPlugin;
use player::PlayerPlugin;
use system::render;

mod common;
mod enemy;
mod map_generator;
mod player;
mod system;

#[derive(Component)]
struct MainCamera;

fn setup(
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut commands: Commands,
) {
    // Setup the sprite sheet
    let texture_handle = asset_server.load("terminal8x8_transparent.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(8.0, 8.0), 16, 16, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    // add sprite atlas as resource
    commands.insert_resource(CharsetAsset {
        atlas: texture_atlas_handle.clone(),
    });

    // Add a 2D Camera
    let mut cam = Camera2dBundle::default();
    cam.transform.scale = Vec3::new(0.5, 0.5, 1.0);
    commands.spawn((MainCamera, cam));
}

fn switch_to_setup_state(mut state: ResMut<NextState<GameState>>) {
    state.set(GameState::Setup);
}

fn switch_to_normal_play(mut state: ResMut<NextState<GameState>>) {
    state.set(GameState::PlayerTurn);
}

fn main() {
    let mut app = App::new();

    app.add_state::<GameState>()
        .add_plugins(DefaultPlugins.set(
        // This sets image filtering to nearest
        // This is done to prevent textures with low resolution (e.g. pixel art) from being blurred
        // by linear filtering.
        ImagePlugin::default_nearest(),
    ).disable::<LogPlugin>());


    #[cfg(not(feature = "debug"))]
    app.add_plugins(LogPlugin::default());

    #[cfg(feature = "debug")]
    app.add_plugins(WorldInspectorPlugin::new())
        .add_plugins(LogPlugin {
            level: bevy::log::Level::DEBUG,
            ..default()
        });
    app.add_plugins((
            PlayerPlugin,
            MapGeneratorPlugin,
            EnemyPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            switch_to_setup_state.run_if(in_state(GameState::LoadAssets)),
        )
        .add_systems(
            Update,
            switch_to_normal_play.run_if(in_state(GameState::Setup)),
        )
        .add_systems(Update, render)
        .run();
}
