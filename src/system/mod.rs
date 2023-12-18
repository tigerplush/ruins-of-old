use bevy::prelude::*;

use crate::common::components::Position;

pub fn render(mut renderables: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in &mut renderables {
        transform.translation = position.to_world();
    }
}
