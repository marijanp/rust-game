use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_spritesheet_animation::prelude::*;

use crate::collider::ColliderBundle;
use crate::enemy::components::{Enemy, EnemyBundle, EnemyMovement};

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut library: ResMut<AnimationLibrary>,
) {
    for (texture, transform, dim, columns, frames) in [
        //("john.png", Transform::from_xyz(10., 1., 3.), 64, 3, 3),
        //("kai.png", Transform::from_xyz(8., 1., 4.), 128, 4, 4),
        ("brat.png", Transform::from_xyz(12., 1., 4.), 32, 7, 4),
        ("brat.png", Transform::from_xyz(8., 1., 4.), 32, 7, 4),
    ] {
        let texture = asset_server.load(texture);
        let spritesheet = Spritesheet::new(columns, 1);

        // Idle
        let idle_animation_id = library
            .animation_with_name(EnemyMovement::Idle)
            .unwrap_or_else(|| {
                let clip = Clip::from_frames(spritesheet.row_partial(0, 0..frames));
                let clip_id = library.register_clip(clip);
                let animation = Animation::from_clip(clip_id);
                let animation_id = library.register_animation(animation);
                library
                    .name_animation(animation_id, EnemyMovement::Idle)
                    .unwrap();
                animation_id
            });

        // Hit
        if library.animation_with_name(EnemyMovement::Hit).is_none() {
            let jump_clip = Clip::from_frames(spritesheet.horizontal_strip(4, 0, 3));
            let jump_clip_id = library.register_clip(jump_clip);
            let mut animation = Animation::from_clip(jump_clip_id);
            animation
                .set_repetitions(AnimationRepeat::Times(1))
                .set_duration(AnimationDuration::PerRepetition(200));
            let animation_id = library.register_animation(animation);
            library
                .name_animation(animation_id, EnemyMovement::Hit)
                .unwrap();
        }

        let layout = texture_atlas_layouts.add(spritesheet.atlas_layout(dim, dim));

        commands.spawn(EnemyBundle {
            enemy: Enemy,
            name: Name::new("Enemy"),
            movement: EnemyMovement::Idle,
            sprite_bundle: Sprite3dBuilder::from_image(texture)
                .with_atlas(layout)
                .with_transform(transform)
                .with_custom_size(Vec2::new(2., 2.))
                .with_flip(true, false)
                .build(),
            sprite_sheet_animation: SpritesheetAnimation::from_id(idle_animation_id),
            collider_bundle: ColliderBundle {
                collider: Collider::round_cylinder(0.9, 0.05, 0.1),
                rigid_body: RigidBody::Dynamic,
                active_events: ActiveEvents::COLLISION_EVENTS,
                rotation_constraints: LockedAxes::ROTATION_LOCKED,
                ..default()
            },
            external_impulse: ExternalImpulse::default(),
        });
    }
}

pub fn despawn(mut commands: Commands, enemy_query: Query<Entity, With<Enemy>>) {
    for enemy in enemy_query.iter() {
        commands.entity(enemy).despawn_recursive();
    }
}

pub fn update_animation(
    library: Res<AnimationLibrary>,
    mut enemy_query: Query<(&EnemyMovement, &mut SpritesheetAnimation), With<Enemy>>,
) {
    for (movement, mut animation) in enemy_query.iter_mut() {
        if let Some(animation_id) = library.animation_with_name(movement) {
            if animation.animation_id != animation_id {
                animation.switch(animation_id);
            }
        }
    }
}
