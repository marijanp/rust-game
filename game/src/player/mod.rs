pub mod components;
pub mod systems;

use crate::{AppState, GameState};
use bevy::prelude::*;

pub const PLAYER_WIDTH: f32 = 32.0;
pub const PLAYER_HEIGHT: f32 = 32.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
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
            (
                systems::move_player,
                systems::player_animation_event,
                systems::update_player_animation,
                systems::update_facing_direction,
                systems::flip_player_sprite,
            )
                .chain()
                .run_if(in_state(GameState::Running))
                .run_if(in_state(AppState::InGame)),
        )
        .add_systems(
            Update,
            (
                systems::collect_fruits.run_if(in_state(GameState::Running)),
                (systems::update_enemies_in_reach, systems::punch)
                    .chain()
                    .run_if(in_state(GameState::Running)),
            ),
        )
        .add_systems(OnExit(AppState::InGame), systems::despawn);
    }
}
