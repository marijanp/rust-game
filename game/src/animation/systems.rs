use bevy::prelude::*;
use bevy_rapier2d::prelude::{KinematicCharacterControllerOutput, Velocity};

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
    &'a KinematicCharacterControllerOutput,
);

const DELTA: f32 = 10.0;

pub fn change_player_animation(mut player: Query<AnimationRelated, With<Player>>) {
    if let Ok((
        velocity,
        textures,
        mut current_texture,
        mut sprite,
        mut animation_indices,
        character_controller,
    )) = player.get_single_mut()
    {
        //if (-DELTA..=DELTA).contains(&velocity.linvel.y) {
        if character_controller.grounded {
            if (-DELTA..=DELTA).contains(&velocity.linvel.x) {
                *current_texture = textures.get(&Movement::Idle).unwrap().clone();
                animation_indices.last = 10;
                info!("idle velocity: {velocity:?}");
            } else {
                info!("run velocity: {velocity:?}");
                animation_indices.last = 10;
                *current_texture = textures.get(&Movement::Run).unwrap().clone();
            }
        } else if !character_controller.grounded && velocity.linvel.y > DELTA {
            info!("jump velocity: {velocity:?}");
            *current_texture = textures.get(&Movement::Jump).unwrap().clone();
            animation_indices.last = 0;
        } else if !character_controller.grounded && velocity.linvel.y < -DELTA {
            info!("fall velocity: {velocity:?}");
            animation_indices.last = 0;
            *current_texture = textures.get(&Movement::Fall).unwrap().clone()
        }
        if velocity.linvel.x > DELTA {
            sprite.flip_x = false;
        } else if velocity.linvel.x < -DELTA {
            sprite.flip_x = true;
        }
    }
}
