use bevy::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Ground;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Wall;

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub enum Collider {
    Ball(f32),
    Cuboid(Vec3),
    Capsule(Vec3, Vec3, f32),
    #[default]
    Mesh,
}
