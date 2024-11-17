use bevy::prelude::*;
use bevy_rapier3d::geometry::Collider as RapierCollider;
use bevy_rapier3d::prelude::*;

#[derive(Clone, Debug, Default, Bundle)]
pub struct ColliderBundle {
    // A collider and mass_properties are required for forces and impulses to be applied correctly
    pub collider: RapierCollider,
    pub mass_properties: ColliderMassProperties,
    pub rigid_body: RigidBody,
    pub active_events: ActiveEvents,
    pub rotation_constraints: LockedAxes,
}
