pub mod components;
pub mod systems;

use crate::{AppState, GameState};

use bevy::prelude::*;
use blenvy::GltfBlueprintsSet;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<components::Collider>()
            .add_systems(
                OnEnter(GameState::SpawningLevelColliders),
                systems::spawn.run_if(in_state(AppState::InGame)),
            )
            .add_systems(
                Update,
                systems::physics_replace_proxies
                    .after(GltfBlueprintsSet::AfterSpawn)
                    .run_if(in_state(GameState::SpawningLevelColliders))
                    .run_if(in_state(AppState::InGame)),
            )
            .add_systems(OnExit(AppState::InGame), systems::despawn);
    }
}
