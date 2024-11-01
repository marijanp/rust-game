use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;


use crate::ColliderBundle;

#[derive(Default, Component)]
pub struct Enemy;

#[derive(Bundle)]
pub struct EnemyBundle {
    pub enemy: Enemy,
    pub name: Name,
    pub sprite_bundle: Sprite3dBundle,
    pub sprite_sheet_animation: SpritesheetAnimation,
    pub collider_bundle: ColliderBundle,
}
