use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::KinematicCharacterController;
use leafwing_input_manager::InputManagerBundle;

use crate::animation::components::{AnimationIndices, AnimationTimer};
use crate::{Action, ColliderBundle};

#[derive(Default, Hash, PartialEq, Eq)]
pub enum Movement {
    #[default]
    Idle,
    Run,
    Jump,
    Fall,
}

#[derive(Default, Component)]
pub struct Player;

#[derive(Default, LdtkEntity, Bundle)]
pub struct PlayerBundle {
    fruit: Player,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: LdtkSpriteSheetBundle,
    #[from_entity_instance]
    collider: ColliderBundle,
    #[with(animation_indices)]
    animation_indices: AnimationIndices,
    #[with(animation_timer)]
    animation_timer: AnimationTimer,
    #[with(input_manager)]
    input_manager: InputManagerBundle<Action>,
    #[with(character_controller)]
    textures: KinematicCharacterController,
}

fn animation_indices(_: &EntityInstance) -> AnimationIndices {
    AnimationIndices { first: 0, last: 10 }
}

fn animation_timer(_: &EntityInstance) -> AnimationTimer {
    AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating))
}

fn input_manager(_: &EntityInstance) -> InputManagerBundle<Action> {
    InputManagerBundle {
        input_map: Action::player_one(),
        ..Default::default()
    }
}

fn character_controller(_: &EntityInstance) -> KinematicCharacterController {
    KinematicCharacterController {
        apply_impulse_to_dynamic_bodies: true,
        ..default()
    }
}
