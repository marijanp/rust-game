pub mod components;
pub mod systems;

use crate::player;
use crate::AppState;

use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), (systems::spawn).chain())
            .add_systems(
                OnEnter(AppState::InGame),
                player::systems::load_player_tilesets,
            )
            .add_systems(
                Update,
                systems::add_ground_collider.run_if(in_state(AppState::InGame)),
            )
            .add_systems(OnExit(AppState::InGame), systems::despawn);
    }
}
