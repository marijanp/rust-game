use bevy::prelude::*;

#[derive(Hash, PartialEq, Eq)]
pub enum Movement {
    Idle,
    Run,
    Jump,
    Fall,
}

#[derive(Component)]
pub struct Player;
