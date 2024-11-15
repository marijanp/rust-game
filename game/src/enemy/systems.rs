use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_spritesheet_animation::prelude::*;

use crate::collider::ColliderBundle;
use crate::enemy::components::{Enemy, EnemyBundle};

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut library: ResMut<AnimationLibrary>,
) {
    for (texture, transform, dim, columns, frames) in [
        ("john.png", Transform::from_xyz(10., 0.1, 3.), 64, 3, 3),
        ("kai.png", Transform::from_xyz(8., 0.1, 4.), 128, 4, 4),
        ("brat.png", Transform::from_xyz(12., 0.1, 4.), 32, 7, 4),
    ] {
        let texture = asset_server.load(texture);

        let spritesheet = Spritesheet::new(columns, 1);
        let clip = Clip::from_frames(spritesheet.row_partial(0, 0..frames));
        let clip_id = library.register_clip(clip);
        let animation = Animation::from_clip(clip_id);
        let animation_id = library.register_animation(animation);

        let layout = texture_atlas_layouts.add(spritesheet.atlas_layout(dim, dim));

        commands.spawn(EnemyBundle {
            enemy: Enemy,
            name: Name::new("Enemy"),
            sprite_bundle: Sprite3dBuilder::from_image(texture)
                .with_atlas(layout)
                .with_transform(transform)
                .with_custom_size(Vec2::new(1., 1.))
                .with_flip(true, false)
                .build(),
            sprite_sheet_animation: SpritesheetAnimation::from_id(animation_id),
            collider_bundle: ColliderBundle {
                collider: Collider::round_cylinder(0.4, 0.1, 0.1),
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
    if let Ok(enemy) = enemy_query.get_single() {
        commands.entity(enemy).despawn();
    }
}
