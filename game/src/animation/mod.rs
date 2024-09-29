pub mod components;
pub mod systems;

use bevy::prelude::*;

use crate::GameState;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (systems::change_player_animation, systems::animate_sprite)
                .chain()
                .run_if(in_state(GameState::Running)),
        );
    }
}
