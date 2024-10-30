pub mod components;
pub mod systems;

use crate::{AppState, GameState};
use bevy::prelude::*;

pub const PLAYER_WIDTH: f32 = 32.0;
pub const PLAYER_HEIGHT: f32 = 32.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), systems::spawn)
            .add_systems(
                Update,
                (systems::move_player)
                    .chain()
                    .run_if(in_state(GameState::Running)),
            )
            .add_systems(
                Update,
                systems::collect_fruits.run_if(in_state(GameState::Running)),
            )
            .add_systems(OnExit(AppState::InGame), systems::despawn);
    }
}
