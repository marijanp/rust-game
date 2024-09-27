use bevy::prelude::*;

pub enum Posture {
    Front,
    Stand,
    Duck,
    Jump,
    Hit,
    Walk,
    Swim,
    Climb,
}

#[derive(Component)]
pub struct Player {
    pub posture: Posture,
}
