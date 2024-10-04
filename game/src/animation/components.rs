use bevy::prelude::*;

#[derive(Default, Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Default, Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);
