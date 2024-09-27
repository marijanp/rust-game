mod components;
mod systems;

use bevy::prelude::*;

use crate::{AppState, GameState};

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Paused),
            systems::spawn.run_if(in_state(AppState::InGame)),
        )
        .add_systems(OnExit(GameState::Paused), systems::despawn)
        .add_systems(
            Update,
            systems::toggle_pause_menu.run_if(in_state(AppState::InGame)),
        )
        .add_systems(
            Update,
            (
                systems::interact_with_main_menu_button,
                systems::interact_with_resume_button,
                systems::interact_with_quit_button,
            )
                .run_if(in_state(AppState::InGame))
                .run_if(in_state(GameState::Paused)),
        );
    }
}
