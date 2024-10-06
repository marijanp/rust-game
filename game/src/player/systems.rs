use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::collections::HashMap;

use leafwing_input_manager::prelude::*;

use crate::fruit::components::Fruit;
use crate::player::components::{Movement, Player};
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

// http://www.mathforgameprogrammers.com/gdc2016/GDC2016_Pittman_Kyle_BuildingABetterJump.pdf
const METER: f32 = 16.;
const V_X: f32 = 10. * METER;
const HEIGHT: f32 = 5. * METER;
const DISTANCE_AT_HEIGHT: f32 = 2.5 * METER;

const V_0: f32 = (2. * HEIGHT * V_X) / DISTANCE_AT_HEIGHT;
const GRAVITY: f32 = (-2. * HEIGHT * (V_X * V_X)) / (DISTANCE_AT_HEIGHT * DISTANCE_AT_HEIGHT);

pub fn move_player(
    mut player_query: Query<
        (
            &ActionState<Action>,
            &mut KinematicCharacterController,
            &Velocity,
        ),
        With<Player>,
    >,
    time: Res<Time>,
) {
    if let Ok((action, mut controller, velocity)) = player_query.get_single_mut() {
        let mut velocity = velocity.linvel;

        if action.just_pressed(&Action::Jump) {
            velocity.y = V_0;
        } else {
            velocity.y += GRAVITY * time.delta_seconds();
        }

        if action.pressed(&Action::Left) {
            velocity.x = -V_X;
        } else if action.pressed(&Action::Right) {
            velocity.x = V_X;
        }

        if action.just_released(&Action::Left) || action.just_released(&Action::Right) {
            velocity.x = 0.;
        }

        let translation_change = velocity * time.delta_seconds();
        controller.translation = match controller.translation {
            Some(existing_translation) => Some(existing_translation + translation_change),
            None => Some(translation_change),
        };
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
