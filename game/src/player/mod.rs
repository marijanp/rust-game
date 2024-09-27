pub mod components;
pub mod systems;

use crate::{AppState, GameState};
use bevy::prelude::*;

const PLAYER_SPEED: f32 = 300.0;
const PLAYER_WIDTH: f32 = 128.0;
const PLAYER_HEIGHT: f32 = 256.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), systems::spawn_player)
            .add_systems(
                Update,
                (systems::move_player, systems::limit_player_movement)
                    .chain()
                    .run_if(in_state(GameState::Running)),
            )
            .add_systems(OnExit(AppState::InGame), systems::despawn);
    }
}
