use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::animation::components::{AnimationIndices, AnimationTimer};
use crate::collider::ColliderBundle;

#[derive(Default, Component)]
pub struct Fruit;

#[derive(Default, LdtkEntity, Bundle)]
pub struct FruitBundle {
    fruit: Fruit,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: LdtkSpriteSheetBundle,
    #[from_entity_instance]
    collider: ColliderBundle,
    #[with(animation_indices)]
    animation_indices: AnimationIndices,
    #[with(animation_timer)]
    animation_timer: AnimationTimer,
}

fn animation_indices(_: &EntityInstance) -> AnimationIndices {
    AnimationIndices { first: 0, last: 16 }
}
fn animation_timer(_: &EntityInstance) -> AnimationTimer {
    AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating))
}
