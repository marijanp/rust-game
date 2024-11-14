use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_spritesheet_animation::prelude::*;
use leafwing_input_manager::InputManagerBundle;

use std::fmt;

use crate::{ColliderBundle, Input};

#[derive(Debug, Default, Hash, PartialEq, Eq, Component)]
pub enum Movement {
    #[default]
    Idle,
    Walk,
    Jump,
    Fall,
    Jab,
    Hook,
    Uppercut,
}

impl fmt::Display for Movement {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        // Use the Debug implementation for Display
        write!(formatter, "{:?}", self)
    }
}

impl From<Movement> for String {
    fn from(movement: Movement) -> Self {
        movement.to_string()
    }
}

#[derive(Default, Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub name: Name,
    pub movement: Movement,
    pub sprite_bundle: Sprite3dBundle,
    pub sprite_sheet_animation: SpritesheetAnimation,
    pub collider_bundle: ColliderBundle,
    pub input_manager: InputManagerBundle<Input>,
    pub character_controller: KinematicCharacterController,
}
