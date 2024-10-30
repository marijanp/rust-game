pub mod components;
pub mod systems;

use crate::AppState;

use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), systems::spawn)
            .add_systems(
                Update,
                systems::draw_cursor.run_if(in_state(AppState::InGame)),
            )
            .add_systems(OnExit(AppState::InGame), systems::despawn);
    }
}
