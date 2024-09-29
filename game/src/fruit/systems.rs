use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;
use rand;

use crate::animation::components::{AnimationIndices, AnimationTimer};
use crate::fruit::{components::Fruit, FRUIT_HEIGHT, FRUIT_WIDTH};

pub fn spawn(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let window = window_query.get_single().unwrap();
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(FRUIT_HEIGHT as u32, FRUIT_WIDTH as u32),
        17,
        1,
        None,
        None,
    );
    for _ in 0..10 {
        commands.spawn((
            Fruit,
            SpriteBundle {
                transform: Transform::from_xyz(
                    rand::random::<f32>() * window.width(),
                    rand::random::<f32>() * window.height(),
                    0.0,
                ),
                texture: asset_server.load("Items/Fruits/Apple.png"),
                ..default()
            },
            TextureAtlas {
                layout: texture_atlas_layouts.add(layout.clone()),
                index: 0,
            },
            // Animation
            AnimationIndices { first: 0, last: 16 },
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            // Physics
            RigidBody::Dynamic,
            Velocity::linear(rand::random::<Vec2>()),
            Collider::ball(8.),
            LockedAxes::ROTATION_LOCKED_Z,
        ));
    }
}

pub fn despawn(mut commands: Commands, fruits: Query<Entity, With<Fruit>>) {
    for fruit in fruits.iter() {
        commands.entity(fruit).despawn();
    }
}
