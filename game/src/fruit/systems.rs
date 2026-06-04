use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_spritesheet_animation::prelude::*;

use crate::fruit::components::{Fruit, FruitBundle};
use crate::ColliderBundle;

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut animations: ResMut<Assets<Animation>>,
) {
    let texture = asset_server.load("apple.png");

    let spritesheet = Spritesheet::new(&texture, 17, 1);
    let animation = animations.add(spritesheet.create_animation().add_row(0).build());

    for i in 5..15 {
        commands.spawn(FruitBundle {
            fruit: Fruit,
            name: Name::new("Fruit"),
            sprite: spritesheet
                .with_size_hint(32, 32)
                .sprite3d(&mut texture_atlas_layouts)
                .with_custom_size(Vec2::new(1., 1.)),
            transform: Transform::from_xyz(i as f32, 1., 2.5),
            sprite_sheet_animation: SpritesheetAnimation::new(animation.clone()),
            collider_bundle: ColliderBundle {
                rotation_constraints: LockedAxes::ROTATION_LOCKED,
                collider: Collider::ball(0.25),
                rigid_body: RigidBody::Dynamic,
                active_events: ActiveEvents::COLLISION_EVENTS,
                ..default()
            },
        });
    }
}

pub fn despawn(mut commands: Commands, fruits: Query<Entity, With<Fruit>>) {
    for fruit in fruits.iter() {
        commands.entity(fruit).despawn();
    }
}
