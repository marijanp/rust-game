pub mod components;
pub mod systems;

use bevy::prelude::*;

use crate::{AppState, GameState};

pub struct EnemyPlugin;

pub const ENEMY_HEIGHT: f32 = 128.0;
pub const ENEMY_WIDTH: f32 = 128.0;
pub const ENEMY_SPEED: f32 = 100.0;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), systems::spawn)
            .add_systems(
                Update,
                (systems::move_enemy, systems::limit_enemy_movement)
                    .chain()
                    .run_if(in_state(GameState::Running)),
            )
            .add_systems(OnExit(AppState::InGame), systems::despawn);
    }
}
