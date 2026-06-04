use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;

use crate::collider::ColliderBundle;

#[derive(Default, Component)]
pub struct Fruit;

#[derive(Bundle)]
pub struct FruitBundle {
    pub fruit: Fruit,
    pub name: Name,
    pub sprite: Sprite3d,
    pub transform: Transform,
    pub sprite_sheet_animation: SpritesheetAnimation,
    pub collider_bundle: ColliderBundle,
}
