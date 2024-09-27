use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::player::{
    components::{Player, Posture},
    PLAYER_HEIGHT, PLAYER_SPEED, PLAYER_WIDTH,
};

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(PLAYER_WIDTH / 2.0, PLAYER_HEIGHT / 2.0, 0.0),
            texture: asset_server.load("Players/128x256/Green/alienGreen_stand.png"),
            ..default()
        },
        Player {
            posture: Posture::Stand,
        },
    ));
}

pub fn despawn(mut commands: Commands, enemy_entity_query: Query<Entity, With<Player>>) {
    if let Ok(player_entity) = enemy_entity_query.get_single() {
        commands.entity(player_entity).despawn();
    }
}

pub fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyA) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyS) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
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
