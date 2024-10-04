use bevy::prelude::*;
use bevy_rapier2d::prelude::{KinematicCharacterControllerOutput, Velocity};

use super::components::{AnimationIndices, AnimationTimer};
use crate::player::components::Player;

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
    &'a mut Handle<Image>,
    &'a mut Sprite,
    &'a mut AnimationIndices,
    &'a KinematicCharacterControllerOutput,
);

const DELTA: f32 = 10.0;

pub fn change_player_animation(
    mut player: Query<AnimationRelated, With<Player>>,
    asset_server: Res<AssetServer>,
) {
    if let Ok((
        velocity,
        mut current_texture,
        mut sprite,
        mut animation_indices,
        character_controller,
    )) = player.get_single_mut()
    {
        if character_controller.grounded {
            if (-DELTA..=DELTA).contains(&velocity.linvel.x) {
                *current_texture = asset_server.load("Main Characters/Mask Dude/Idle (32x32).png");
                animation_indices.last = 10;
            } else {
                animation_indices.last = 10;
                *current_texture = asset_server.load("Main Characters/Mask Dude/Run (32x32).png");
            }
        } else if !character_controller.grounded && velocity.linvel.y > DELTA {
            *current_texture = asset_server.load("Main Characters/Mask Dude/Jump (32x32).png");
            animation_indices.last = 0;
        } else if !character_controller.grounded && velocity.linvel.y < -DELTA {
            animation_indices.last = 0;
            *current_texture = asset_server.load("Main Characters/Mask Dude/Fall (32x32).png");
        }
        if velocity.linvel.x > DELTA {
            sprite.flip_x = false;
        } else if velocity.linvel.x < -DELTA {
            sprite.flip_x = true;
        }
    }
}
