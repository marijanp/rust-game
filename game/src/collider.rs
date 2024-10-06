use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct ColliderBundle {
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub active_events: ActiveEvents,
    pub velocity: Velocity,
    pub rotation_constraints: LockedAxes,
}

impl From<&EntityInstance> for ColliderBundle {
    fn from(entity_instance: &EntityInstance) -> ColliderBundle {
        match entity_instance.identifier.as_ref() {
            "Cherry" => ColliderBundle {
                collider: Collider::ball(8.),
                rigid_body: RigidBody::Dynamic,
                active_events: ActiveEvents::COLLISION_EVENTS,
                rotation_constraints: LockedAxes::ROTATION_LOCKED_Z,
                ..default()
            },
            "Player" => ColliderBundle {
                collider: Collider::cuboid(9., 15.95),
                rigid_body: RigidBody::KinematicPositionBased,
                active_events: ActiveEvents::COLLISION_EVENTS,
                rotation_constraints: LockedAxes::ROTATION_LOCKED_Z,
                ..default()
            },
            entity => {
                tracing::debug!("Unknown entity '{entity}'");
                ColliderBundle::default()
            }
        }
    }
}

impl From<IntGridCell> for ColliderBundle {
    fn from(int_grid_cell: IntGridCell) -> ColliderBundle {
        if int_grid_cell.value == 1 {
            ColliderBundle {
                collider: Collider::cuboid(16. / 2., 16. / 2.),
                rigid_body: RigidBody::Fixed,
                ..default()
            }
        } else {
            panic!("Unsupported int grid cell value")
        }
    }
}
