use bevy::prelude::*;

use crate::fruit::components::Fruit;

pub fn despawn(mut commands: Commands, fruits: Query<Entity, With<Fruit>>) {
    for fruit in fruits.iter() {
        commands.entity(fruit).despawn();
    }
}
