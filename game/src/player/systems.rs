use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;
use std::collections::HashMap;

use leafwing_input_manager::prelude::*;

use crate::fruit::components::Fruit;
use crate::player::{
    components::{Movement, Player},
    PLAYER_HEIGHT, PLAYER_WIDTH,
};
use crate::{Action, Tilesets};

pub fn load_player_tilesets(
    asset_server: Res<AssetServer>,
    mut tileset: ResMut<Tilesets<Movement>>,
) {
    let idle_tileset = asset_server.load("Main Characters/Mask Dude/Idle (32x32).png");
    let run_tileset = asset_server.load("Main Characters/Mask Dude/Run (32x32).png");
    let jump_tileset = asset_server.load("Main Characters/Mask Dude/Jump (32x32).png");
    let fall_tileset = asset_server.load("Main Characters/Mask Dude/Fall (32x32).png");
    *tileset = Tilesets(HashMap::from([
        (Movement::Idle, idle_tileset),
        (Movement::Run, run_tileset),
        (Movement::Jump, jump_tileset),
        (Movement::Fall, fall_tileset),
    ]));
}

pub fn despawn(mut commands: Commands, enemy_entity_query: Query<Entity, With<Player>>) {
    if let Ok(player_entity) = enemy_entity_query.get_single() {
        commands.entity(player_entity).despawn();
    }
}

const MAX_SPEED: f32 = 150.0;
const ACCELERATION: f32 = 50.0;
pub fn move_player(
    mut player_query: Query<
        (
            &ActionState<Action>,
            &mut KinematicCharacterController,
            &mut Velocity,
        ),
        With<Player>,
    >,
    time: Res<Time>,
) {
    if let Ok((action, mut controller, mut velocity)) = player_query.get_single_mut() {
        if action.just_pressed(&Action::Jump) {
            velocity.linvel.y = MAX_SPEED;
        } else if action.just_pressed(&Action::Fall) {
            velocity.linvel.y = -MAX_SPEED;
        } else if action.pressed(&Action::Left) {
            velocity.linvel.x -= ACCELERATION;
        } else if action.pressed(&Action::Right) {
            velocity.linvel.x += ACCELERATION;
        }
        velocity.linvel.x = velocity.linvel.x.clamp(-MAX_SPEED, MAX_SPEED);
        let translation_change = velocity.linvel * time.delta_seconds();
        controller.translation = match controller.translation {
            Some(existing_translation) => Some(existing_translation + translation_change),
            None => Some(translation_change),
        };
    }
}

pub fn limit_player_movement(
    mut player_transform_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    const HALF_PLAYER_WIDTH: f32 = PLAYER_WIDTH / 2.0;
    const X_MIN: f32 = 0.0 + HALF_PLAYER_WIDTH;
    let x_max = window.width() - HALF_PLAYER_WIDTH;

    const HALF_PLAYER_HEIGHT: f32 = PLAYER_HEIGHT / 2.0;
    const Y_MIN: f32 = 0.0 + HALF_PLAYER_HEIGHT;
    let y_max = window.height() - HALF_PLAYER_HEIGHT;

    for mut transform in player_transform_query.iter_mut() {
        let mut translation = transform.translation;

        // Bound the enemy x position
        if translation.x < X_MIN {
            translation.x = X_MIN;
        } else if translation.x > x_max {
            translation.x = x_max;
        }
        // Bound the enemy y position
        if translation.y < Y_MIN {
            translation.y = Y_MIN;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        transform.translation = translation;
    }
}

pub fn collect_fruits(
    mut commands: Commands,
    character_controller_outputs: Query<
        &KinematicCharacterControllerOutput,
        (With<Player>, Changed<KinematicCharacterControllerOutput>),
    >,
    fruits: Query<Entity, With<Fruit>>,
) {
    if let Ok(output) = character_controller_outputs.get_single() {
        for collision in &output.collisions {
            if fruits.get(collision.entity).is_ok() {
                info!("Fruit collected");
                commands.entity(collision.entity).despawn()
            }
        }
    }
}
