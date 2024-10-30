use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::world::components::{Ground, Wall};

const WALL_HEIGHT: f32 = 1.;
const WALL_WIDTH: f32 = 20.;
const WALL_THICKNESS: f32 = 0.01;

const GROUND_LENGTH: f32 = 20.;
const GROUND_WIDTH: f32 = 5.;

// http://www.mathforgameprogrammers.com/gdc2016/GDC2016_Pittman_Kyle_BuildingABetterJump.pdf
pub fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Ground,
        Name::new("Ground"),
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(GROUND_LENGTH, GROUND_WIDTH)),
            transform: Transform::from_xyz(GROUND_LENGTH / 2., 0., GROUND_WIDTH / 2.),
            material: materials.add(Color::srgb(0., 1., 0.)),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(GROUND_LENGTH / 2., 0.01, GROUND_WIDTH / 2.),
    ));

    commands.spawn((
        Wall,
        Name::new("Level Start Wall"),
        PbrBundle {
            mesh: meshes.add(Cuboid::new(WALL_THICKNESS, WALL_HEIGHT, GROUND_WIDTH)),
            transform: Transform::from_xyz(0., WALL_HEIGHT / 2., GROUND_WIDTH / 2.),
            material: materials.add(Color::srgba(1., 1., 1., 1.)),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(WALL_THICKNESS / 2., WALL_HEIGHT / 2., GROUND_WIDTH / 2.),
    ));

    commands.spawn((
        Wall,
        Name::new("Back Wall"),
        PbrBundle {
            mesh: meshes.add(Cuboid::new(WALL_WIDTH, WALL_HEIGHT, WALL_THICKNESS)),
            transform: Transform::from_xyz(WALL_WIDTH / 2., WALL_HEIGHT / 2., 0.),
            material: materials.add(Color::srgba(1., 0., 0., 0.5)),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(WALL_WIDTH / 2., WALL_HEIGHT / 2., WALL_THICKNESS / 2.),
    ));

    commands.spawn((
        Wall,
        Name::new("Front Wall"),
        PbrBundle {
            mesh: meshes.add(Cuboid::new(WALL_WIDTH, WALL_HEIGHT, WALL_THICKNESS)),
            transform: Transform::from_xyz(WALL_WIDTH / 2., WALL_HEIGHT / 2., GROUND_WIDTH),
            material: materials.add(Color::srgba(0., 0., 1., 0.1)),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(WALL_WIDTH / 2., WALL_HEIGHT / 2., WALL_THICKNESS / 2.),
    ));
}

pub fn despawn(
    mut commands: Commands,
    ground_entity_query: Query<Entity, With<Ground>>,
    walls_entity_query: Query<Entity, With<Wall>>,
) {
    if let Ok(ground_entity) = ground_entity_query.get_single() {
        commands.entity(ground_entity).despawn();
    }
    for wall in walls_entity_query.iter() {
        commands.entity(wall).despawn();
    }
}

pub fn draw_cursor(
    camera_query: Query<(&Camera, &GlobalTransform)>,
    ground_query: Query<&GlobalTransform, With<Ground>>,
    windows: Query<&Window>,
    mut gizmos: Gizmos,
) {
    let (camera, camera_transform) = camera_query.single();
    let ground = ground_query.single();

    let Some(cursor_position) = windows.single().cursor_position() else {
        return;
    };

    // Calculate a ray pointing from the camera into the world based on the cursor's position.
    let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };

    // Calculate if and where the ray is hitting the ground plane.
    let Some(distance) =
        ray.intersect_plane(ground.translation(), InfinitePlane3d::new(ground.up()))
    else {
        return;
    };
    let point = ray.get_point(distance);

    // Draw a circle just above the ground plane at that position.
    gizmos.circle(point + ground.up() * 0.1, ground.up(), 0.2, Color::WHITE);
}
