use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_spritesheet_animation::prelude::*;

use std::fmt;

use crate::ColliderBundle;

#[derive(Default, Component)]
pub struct Enemy;

#[derive(Debug, Default, Hash, PartialEq, Eq, Component)]
pub enum EnemyMovement {
    #[default]
    Idle,
    Hit,
}

impl fmt::Display for EnemyMovement {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        // Use the Debug implementation for Display
        write!(formatter, "{:?}", self)
    }
}

impl From<EnemyMovement> for String {
    fn from(movement: EnemyMovement) -> Self {
        movement.to_string() + "enemy"
    }
}

impl From<&EnemyMovement> for String {
    fn from(movement: &EnemyMovement) -> Self {
        movement.to_string() + "enemy"
    }
}

#[derive(Bundle)]
pub struct EnemyBundle {
    pub enemy: Enemy,
    pub name: Name,
    pub movement: EnemyMovement,
    pub sprite_bundle: Sprite3dBundle,
    pub sprite_sheet_animation: SpritesheetAnimation,
    pub collider_bundle: ColliderBundle,
    pub external_impulse: ExternalImpulse,
}
