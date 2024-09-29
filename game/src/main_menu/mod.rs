pub mod components;
pub mod systems;

use bevy::prelude::*;

use crate::AppState;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), systems::spawn)
            .add_systems(OnExit(AppState::MainMenu), systems::despawn)
            .add_systems(
                Update,
                (
                    systems::interact_with_play_button,
                    systems::interact_with_quit_button,
                )
                    .run_if(in_state(AppState::MainMenu)),
            );
    }
}
