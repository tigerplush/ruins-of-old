use bevy::prelude::*;

#[derive(Resource)]
pub struct CharsetAsset {
    pub atlas: Handle<TextureAtlas>,
}
