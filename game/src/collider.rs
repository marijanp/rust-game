use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Clone, Debug, Default, Bundle)]
pub struct ColliderBundle {
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub active_events: ActiveEvents,
    pub rotation_constraints: LockedAxes,
}
