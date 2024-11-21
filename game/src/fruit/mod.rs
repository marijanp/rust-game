pub mod components;
pub mod systems;

use bevy::prelude::*;

use crate::{AppState, GameState};

pub struct FruitPlugin;

pub const FRUIT_HEIGHT: f32 = 32.0;
pub const FRUIT_WIDTH: f32 = 32.0;

impl Plugin for FruitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnTransition {
                exited: GameState::SpawningLevelColliders,
                entered: GameState::Running,
            },
            systems::spawn.run_if(in_state(AppState::InGame)),
        )
        .add_systems(OnExit(AppState::InGame), systems::despawn);
    }
}
