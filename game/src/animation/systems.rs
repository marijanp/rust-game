use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;

use super::components::{AnimationIndices, AnimationTimer};
use crate::player::components::{Movement, Player};
use crate::Textures;

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
) {
    for (indices, mut timer, mut atlas) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = if atlas.index >= indices.last {
                indices.first
            } else {
                atlas.index + 1
            };
        }
    }
}

type AnimationRelated<'a> = (
    &'a Velocity,
    &'a Textures<Movement>,
    &'a mut Handle<Image>,
    &'a mut Sprite,
    &'a mut AnimationIndices,
);

pub fn change_player_animation(mut player: Query<AnimationRelated, With<Player>>) {
    if let Ok((velocity, textures, mut current_texture, mut sprite, mut animation_indices)) =
        player.get_single_mut()
    {
        if velocity.linvel.x == 0. && velocity.linvel.y == 0. {
            *current_texture = textures.get(&Movement::Idle).unwrap().clone();
            animation_indices.last = 10;
        } else {
            if velocity.linvel.y > 0. {
                *current_texture = textures.get(&Movement::Jump).unwrap().clone();
                animation_indices.last = 0;
            } else if velocity.linvel.y < 0. {
                animation_indices.last = 0;
                *current_texture = textures.get(&Movement::Fall).unwrap().clone()
            } else {
                animation_indices.last = 10;
                *current_texture = textures.get(&Movement::Run).unwrap().clone();
            }
            if velocity.linvel.x > 0. {
                sprite.flip_x = false;
            }
            if velocity.linvel.x < 0. {
                sprite.flip_x = true;
            }
        }
    }
}
