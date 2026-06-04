use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_spritesheet_animation::prelude::*;

use crate::collider::ColliderBundle;
use crate::enemy::components::{Enemy, EnemyBundle, EnemyMovement};

#[derive(Resource)]
pub struct EnemyAnimations {
    idle: Handle<Animation>,
    hit: Handle<Animation>,
}

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut animations: ResMut<Assets<Animation>>,
) {
    let mut animation_resource = None;

    for (texture, transform, dim, columns, frames) in [
        //("john.png", Transform::from_xyz(10., 1., 3.), 64, 3, 3),
        //("kai.png", Transform::from_xyz(8., 1., 4.), 128, 4, 4),
        ("brat.png", Transform::from_xyz(12., 1., 4.), 32, 7, 4),
        ("brat.png", Transform::from_xyz(8., 1., 4.), 32, 7, 4),
    ] {
        let texture = asset_server.load(texture);
        let spritesheet = Spritesheet::new(&texture, columns, 1);
        let enemy_animations = animation_resource.get_or_insert_with(|| EnemyAnimations {
            idle: animations.add(
                spritesheet
                    .create_animation()
                    .add_partial_row(0, 0..frames)
                    .build(),
            ),
            hit: animations.add(
                spritesheet
                    .create_animation()
                    .add_horizontal_strip(4, 0, 3)
                    .set_repetitions(AnimationRepeat::Times(1))
                    .set_duration(AnimationDuration::PerRepetition(200))
                    .build(),
            ),
        });

        commands.spawn(EnemyBundle {
            enemy: Enemy,
            name: Name::new("Enemy"),
            movement: EnemyMovement::Idle,
            sprite: spritesheet
                .with_size_hint(dim, dim)
                .sprite3d(&mut texture_atlas_layouts)
                .with_custom_size(Vec2::new(2., 2.))
                .with_flip(true, false),
            transform,
            sprite_sheet_animation: SpritesheetAnimation::new(enemy_animations.idle.clone()),
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

    if let Some(animation_resource) = animation_resource {
        commands.insert_resource(animation_resource);
    }
}

pub fn despawn(mut commands: Commands, enemy_query: Query<Entity, With<Enemy>>) {
    for enemy in enemy_query.iter() {
        commands.entity(enemy).despawn();
    }
}

pub fn update_animation(
    animations: Res<EnemyAnimations>,
    mut enemy_query: Query<(&EnemyMovement, &mut SpritesheetAnimation), With<Enemy>>,
) {
    for (movement, mut animation) in enemy_query.iter_mut() {
        let next_animation = match movement {
            EnemyMovement::Idle => &animations.idle,
            EnemyMovement::Hit => &animations.hit,
        };
        if animation.animation != *next_animation {
            animation.switch(next_animation.clone());
        }
    }
}
