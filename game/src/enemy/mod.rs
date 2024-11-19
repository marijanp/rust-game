pub mod components;
pub mod systems;

use crate::{AppState, GameState};
use bevy::prelude::*;
use blenvy::GltfBlueprintsSet;

pub const PLAYER_WIDTH: f32 = 32.0;
pub const PLAYER_HEIGHT: f32 = 32.0;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::InGame),
            systems::spawn.after(GltfBlueprintsSet::AfterSpawn),
        )
        .add_systems(
            Update,
            systems::update_animation.run_if(in_state(GameState::Running)),
        )
        .add_systems(OnExit(AppState::InGame), systems::despawn);
    }
}
