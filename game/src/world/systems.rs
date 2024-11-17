use bevy::prelude::*;
use bevy::render::mesh::PrimitiveTopology;
use bevy_rapier3d::prelude::Collider as RapierCollider;
use bevy_rapier3d::prelude::*;
use blenvy::*;

use crate::world::components::Collider;

pub fn spawn(mut commands: Commands) {
    commands.spawn((
        BlueprintInfo::from_path("levels/World.glb"),
        SpawnBlueprint,
        HideUntilReady,
        GameWorldTag,
    ));
}

pub fn despawn(mut commands: Commands, gameworlds: Query<Entity, With<GameWorldTag>>) {
    for gameworld in gameworlds.iter() {
        commands.entity(gameworld).despawn_recursive();
    }
}

// replaces all physics stand-ins with the actual rapier types
#[allow(clippy::type_complexity)]
pub fn physics_replace_proxies(
    meshes: Res<Assets<Mesh>>,
    mesh_handles: Query<&Handle<Mesh>>,
    mut proxy_colliders: Query<
        (Entity, &Collider, &Name, &mut Visibility),
        (Without<RapierCollider>, Added<Collider>),
    >,
    // needed for tri meshes
    children: Query<&Children>,

    mut commands: Commands,
) {
    for proxy_colider in proxy_colliders.iter_mut() {
        let (entity, collider_proxy, name, mut visibility) = proxy_colider;
        // we hide the collider meshes: perhaps they should be removed altogether once processed ?
        if name.ends_with("_collider") || name.ends_with("_sensor") {
            *visibility = Visibility::Hidden;
        }

        let mut rapier_collider: RapierCollider;
        match collider_proxy {
            Collider::Ball(radius) => {
                rapier_collider = RapierCollider::ball(*radius);
                commands.entity(entity).insert(rapier_collider);
            }
            Collider::Cuboid(size) => {
                rapier_collider = RapierCollider::cuboid(size.x, size.y, size.z);
                commands.entity(entity).insert(rapier_collider);
            }
            Collider::Capsule(a, b, radius) => {
                rapier_collider = RapierCollider::capsule(*a, *b, *radius);
                commands.entity(entity).insert(rapier_collider);
            }
            Collider::Mesh => {
                for (_, collider_mesh) in
                    Mesh::search_in_children(entity, &children, &meshes, &mesh_handles)
                {
                    rapier_collider = RapierCollider::from_bevy_mesh(
                        collider_mesh,
                        &ComputedColliderShape::TriMesh,
                    )
                    .unwrap();
                    commands.entity(entity).insert(rapier_collider);
                }
            }
        }
    }
}

pub(crate) trait MeshExt {
    fn search_in_children<'a>(
        parent: Entity,
        children: &'a Query<&Children>,
        meshes: &'a Assets<Mesh>,
        mesh_handles: &'a Query<&Handle<Mesh>>,
    ) -> Vec<(Entity, &'a Mesh)>;
}

impl MeshExt for Mesh {
    fn search_in_children<'a>(
        parent: Entity,
        children_query: &'a Query<&Children>,
        meshes: &'a Assets<Mesh>,
        mesh_handles: &'a Query<&Handle<Mesh>>,
    ) -> Vec<(Entity, &'a Mesh)> {
        if let Ok(children) = children_query.get(parent) {
            let mut result: Vec<_> = children
                .iter()
                .filter_map(|entity| mesh_handles.get(*entity).ok().map(|mesh| (*entity, mesh)))
                .map(|(entity, mesh_handle)| {
                    (
                        entity,
                        meshes
                            .get(mesh_handle)
                            .expect("Failed to get mesh from handle"),
                    )
                })
                .map(|(entity, mesh)| {
                    assert_eq!(mesh.primitive_topology(), PrimitiveTopology::TriangleList);
                    (entity, mesh)
                })
                .collect();
            let mut inner_result = children
                .iter()
                .flat_map(|entity| {
                    Self::search_in_children(*entity, children_query, meshes, mesh_handles)
                })
                .collect();
            result.append(&mut inner_result);
            result
        } else {
            Vec::new()
        }
    }
}
