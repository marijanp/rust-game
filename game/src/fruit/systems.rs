use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_spritesheet_animation::prelude::*;

use crate::fruit::components::{Fruit, FruitBundle};
use crate::ColliderBundle;

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut library: ResMut<AnimationLibrary>,
) {
    let texture = asset_server.load("Items/Fruits/Apple.png");

    let spritesheet = Spritesheet::new(17, 1);
    let clip = Clip::from_frames(spritesheet.row(0));
    let clip_id = library.register_clip(clip);
    let animation = Animation::from_clip(clip_id);
    let animation_id = library.register_animation(animation);

    let layout = texture_atlas_layouts.add(spritesheet.atlas_layout(32, 32));

    for i in 5..15 {
        commands.spawn(FruitBundle {
            fruit: Fruit,
            name: Name::new("Fruit"),
            sprite_bundle: Sprite3dBuilder::from_image(texture.clone())
                .with_atlas(layout.clone())
                .with_transform(Transform::from_xyz(i as f32, 1., 2.5))
                .with_custom_size(Vec2::new(1., 1.))
                .build(),
            sprite_sheet_animation: SpritesheetAnimation::from_id(animation_id),
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
