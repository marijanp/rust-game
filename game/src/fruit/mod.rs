pub mod components;
pub mod systems;

use bevy::prelude::*;

use crate::AppState;

pub struct FruitPlugin;

pub const FRUIT_HEIGHT: f32 = 32.0;
pub const FRUIT_WIDTH: f32 = 32.0;

impl Plugin for FruitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), systems::spawn)
            .add_systems(OnExit(AppState::InGame), systems::despawn);
    }
}
