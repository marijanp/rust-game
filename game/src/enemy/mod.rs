pub mod components;
pub mod systems;

use crate::{AppState, GameState};
use bevy::prelude::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnTransition {
                exited: GameState::SpawningLevelColliders,
                entered: GameState::Running,
            },
            systems::spawn.run_if(in_state(AppState::InGame)),
        )
        .add_systems(
            Update,
            systems::update_animation.run_if(in_state(GameState::Running)),
        )
        .add_systems(OnExit(AppState::InGame), systems::despawn);
    }
}
