use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_spritesheet_animation::prelude::*;

use leafwing_input_manager::InputManagerBundle;

use crate::{ColliderBundle, Input};

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

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub name: Name,
    pub sprite_bundle: Sprite3dBundle,
    pub sprite_sheet_animation: SpritesheetAnimation,
    pub collider_bundle: ColliderBundle,
    pub input_manager: InputManagerBundle<Input>,
    pub character_controller: KinematicCharacterController,
}
