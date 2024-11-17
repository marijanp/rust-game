use bevy::prelude::*;

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub enum Collider {
    Ball(f32),
    Cuboid(Vec3),
    Capsule(Vec3, Vec3, f32),
    #[default]
    Mesh,
}
