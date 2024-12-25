use bevy::prelude::*;
use bevy_rapier2d::prelude::{KinematicCharacterControllerOutput, Velocity};

use super::components::{AnimationIndices, AnimationTimer};
use crate::player::components::{Movement, Player};
use crate::Tilesets;

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite)>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = if atlas.index >= indices.last {
                    indices.first
                } else {
                    atlas.index + 1
                };
            }
        }
    }
}

type AnimationRelated<'a> = (
    &'a Velocity,
    &'a mut Sprite,
    &'a mut AnimationIndices,
    &'a KinematicCharacterControllerOutput,
);

const DELTA: f32 = 10.0;

pub fn change_player_animation(
    mut player: Query<AnimationRelated, With<Player>>,
    tileset: Res<Tilesets<Movement>>,
) {
    if let Ok((velocity, mut sprite, mut animation_indices, character_controller)) =
        player.get_single_mut()
    {
        if character_controller.grounded {
            if (-DELTA..=DELTA).contains(&velocity.linvel.x) {
                sprite.image = tileset.get(&Movement::Idle).unwrap().clone();
                animation_indices.last = 10;
            } else {
                animation_indices.last = 10;
                sprite.image = tileset.get(&Movement::Run).unwrap().clone();
            }
        } else if !character_controller.grounded && velocity.linvel.y > DELTA {
            sprite.image = tileset.get(&Movement::Jump).unwrap().clone();
            animation_indices.last = 0;
        } else if !character_controller.grounded && velocity.linvel.y < -DELTA {
            animation_indices.last = 0;
            sprite.image = tileset.get(&Movement::Fall).unwrap().clone();
        }
        if velocity.linvel.x > DELTA {
            sprite.flip_x = false;
        } else if velocity.linvel.x < -DELTA {
            sprite.flip_x = true;
        }
    }
}
