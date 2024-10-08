use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Ground;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct GroundBundle {
    ground: Ground,
}
