pub mod components;
pub mod systems;

use crate::{AppState, GameState};

use bevy::prelude::*;

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
                (
                    systems::physics_replace_proxies,
                    systems::convert_named_collider_meshes,
                    systems::convert_world_scene_meshes,
                    systems::finish_spawning_without_collider_proxies,
                )
                    .chain()
                    .run_if(in_state(GameState::SpawningLevelColliders))
                    .run_if(in_state(AppState::InGame)),
            )
            .add_systems(
                Update,
                (
                    systems::physics_replace_proxies,
                    systems::convert_named_collider_meshes,
                    systems::convert_world_scene_meshes,
                )
                    .chain()
                    .run_if(in_state(AppState::InGame))
                    .run_if(systems::not_spawning_level_colliders),
            )
            .add_systems(OnExit(AppState::InGame), systems::despawn);
    }
}
