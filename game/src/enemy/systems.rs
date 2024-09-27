use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand;

use crate::enemy::{components::Enemy, ENEMY_HEIGHT, ENEMY_SPEED, ENEMY_WIDTH};

pub fn spawn(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..10 {
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(
                    rand::random::<f32>() * window.width(),
                    rand::random::<f32>() * window.height(),
                    0.0,
                ),
                texture: asset_server.load("Enemies/bee.png"),
                ..default()
            },
            Enemy {
                direction: Vec2::new(rand::random::<f32>(), rand::random::<f32>()),
            },
        ));
    }
}

pub fn despawn(mut commands: Commands, enemy_entity_query: Query<Entity, With<Enemy>>) {
    for enemy_entity in enemy_entity_query.iter() {
        commands.entity(enemy_entity).despawn();
    }
}

pub fn limit_enemy_movement(
    mut enemy_transform_query: Query<&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    const HALF_ENEMY_WIDTH: f32 = ENEMY_WIDTH / 2.0;
    const X_MIN: f32 = 0.0 + HALF_ENEMY_WIDTH;
    let x_max = window.width() - HALF_ENEMY_WIDTH;

    const HALF_ENEMY_HEIGHT: f32 = ENEMY_HEIGHT / 2.0;
    const Y_MIN: f32 = 0.0 + HALF_ENEMY_HEIGHT;
    let y_max = window.height() - HALF_ENEMY_HEIGHT;

    for mut transform in enemy_transform_query.iter_mut() {
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

pub fn move_enemy(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}
