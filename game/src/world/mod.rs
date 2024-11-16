pub mod components;
pub mod systems;

use crate::AppState;

use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<components::Collider>()
            .add_systems(
                OnEnter(AppState::InGame), systems::spawn)
            .add_systems(Update, systems::physics_replace_proxies.run_if(in_state(AppState::InGame)))
            .add_systems(OnExit(AppState::InGame), systems::despawn);
    }
}
