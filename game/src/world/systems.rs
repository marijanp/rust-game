use bevy::prelude::*;
use bevy::mesh::PrimitiveTopology;
use bevy_rapier3d::prelude::Collider as RapierCollider;
use bevy_rapier3d::prelude::*;

use crate::world::components::{Collider, GameWorldTag};
use crate::GameState;

pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("levels/World.glb"))),
        GameWorldTag,
    ));
}

pub fn not_spawning_level_colliders(game_state: Res<State<GameState>>) -> bool {
    *game_state.get() != GameState::SpawningLevelColliders
}

pub fn despawn(mut commands: Commands, gameworlds: Query<Entity, With<GameWorldTag>>) {
    for gameworld in gameworlds.iter() {
        commands.entity(gameworld).despawn();
    }
}

// replaces all physics stand-ins with the actual rapier types
#[allow(clippy::type_complexity)]
pub fn physics_replace_proxies(
    meshes: Res<Assets<Mesh>>,
    mesh_handles: Query<&Mesh3d>,
    mut proxy_colliders: Query<
        (Entity, &Collider, &Name, &mut Visibility),
        Without<RapierCollider>,
    >,
    // needed for tri meshes
    children: Query<&Children>,
    mut commands: Commands,
    game_state: Res<State<GameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    let mut converted = false;

    for proxy_colider in proxy_colliders.iter_mut() {
        let (entity, collider_proxy, name, mut visibility) = proxy_colider;
        // we hide the collider meshes: perhaps they should be removed altogether once processed ?
        if is_occluding_world_mesh(name.as_str()) {
            *visibility = Visibility::Hidden;
        }

        match collider_proxy {
            Collider::Ball(radius) => {
                commands
                    .entity(entity)
                    .insert((RapierCollider::ball(*radius), RigidBody::Fixed));
                converted = true;
            }
            Collider::Cuboid(size) => {
                commands
                    .entity(entity)
                    .insert((RapierCollider::cuboid(size.x, size.y, size.z), RigidBody::Fixed));
                converted = true;
            }
            Collider::Capsule(a, b, radius) => {
                commands
                    .entity(entity)
                    .insert((RapierCollider::capsule(*a, *b, *radius), RigidBody::Fixed));
                converted = true;
            }
            Collider::Mesh => {
                for (_, collider_mesh) in
                    Mesh::search_in_self_and_children(entity, &children, &meshes, &mesh_handles)
                {
                    let rapier_collider = RapierCollider::from_bevy_mesh(
                        collider_mesh,
                        &ComputedColliderShape::TriMesh(TriMeshFlags::default()),
                    )
                    .unwrap();
                    commands.entity(entity).insert((rapier_collider, RigidBody::Fixed));
                    converted = true;
                }
            }
        }
        if name.ends_with("_sensor") {
            commands.entity(entity).insert(Sensor);
        }
    }
    if converted && *game_state.get() == GameState::SpawningLevelColliders {
        next_game_state.set(GameState::Running);
    }
}

pub fn convert_named_collider_meshes(
    meshes: Res<Assets<Mesh>>,
    mesh_handles: Query<&Mesh3d>,
    mut named_colliders: Query<
        (Entity, &Name, Option<&mut Visibility>),
        (Without<RapierCollider>, Without<Collider>),
    >,
    children: Query<&Children>,
    mut commands: Commands,
    game_state: Res<State<GameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    let mut converted = false;

    for (entity, name, visibility) in named_colliders.iter_mut() {
        if !is_collider_proxy_name(name.as_str()) {
            continue;
        }

        if let Some(mut visibility) = visibility {
            *visibility = Visibility::Hidden;
        }

        let Some((_, collider_mesh)) =
            Mesh::search_in_self_and_children(entity, &children, &meshes, &mesh_handles)
                .into_iter()
                .next()
        else {
            continue;
        };

        let Some(rapier_collider) = RapierCollider::from_bevy_mesh(
            collider_mesh,
            &ComputedColliderShape::TriMesh(TriMeshFlags::default()),
        ) else {
            continue;
        };

        let mut entity_commands = commands.entity(entity);
        entity_commands.insert((rapier_collider, RigidBody::Fixed));
        if name.ends_with("_sensor") {
            entity_commands.insert(Sensor);
        }
        converted = true;
    }

    if converted && *game_state.get() == GameState::SpawningLevelColliders {
        next_game_state.set(GameState::Running);
    }
}

pub fn convert_world_scene_meshes(
    gameworlds: Query<Entity, With<GameWorldTag>>,
    meshes: Res<Assets<Mesh>>,
    mesh_handles: Query<&Mesh3d>,
    mut world_mesh_metadata: Query<(Option<&Name>, Option<&mut Visibility>), With<Mesh3d>>,
    existing_colliders: Query<(), With<RapierCollider>>,
    children: Query<&Children>,
    mut commands: Commands,
    game_state: Res<State<GameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    let mut converted = false;

    for gameworld in gameworlds.iter() {
        for (entity, mesh) in Mesh::search_in_self_and_children(
            gameworld,
            &children,
            &meshes,
            &mesh_handles,
        ) {
            if existing_colliders.get(entity).is_ok() {
                continue;
            }

            let Some(rapier_collider) = RapierCollider::from_bevy_mesh(
                mesh,
                &ComputedColliderShape::TriMesh(TriMeshFlags::default()),
            ) else {
                continue;
            };

            commands
                .entity(entity)
                .insert((rapier_collider, RigidBody::Fixed));

            if let Ok((name, Some(mut visibility))) = world_mesh_metadata.get_mut(entity) {
                if name.is_some_and(|name| is_occluding_world_mesh(name.as_str())) {
                    *visibility = Visibility::Hidden;
                }
            }

            converted = true;
        }
    }

    if converted && *game_state.get() == GameState::SpawningLevelColliders {
        next_game_state.set(GameState::Running);
    }
}

pub fn finish_spawning_without_collider_proxies(
    gameworlds: Query<&Children, With<GameWorldTag>>,
    proxy_colliders: Query<Entity, With<Collider>>,
    named_colliders: Query<(&Name, Option<&RapierCollider>), Without<Collider>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    let has_unprocessed_named_collider = named_colliders
        .iter()
        .any(|(name, rapier_collider)| {
            is_collider_proxy_name(name.as_str()) && rapier_collider.is_none()
        });

    if !gameworlds.is_empty() && proxy_colliders.is_empty() && !has_unprocessed_named_collider {
        next_game_state.set(GameState::Running);
    }
}

fn is_collider_proxy_name(name: &str) -> bool {
    name.ends_with("_collider") || name.ends_with("_sensor")
}

fn is_occluding_world_mesh(name: &str) -> bool {
    name.contains("Wall") || is_collider_proxy_name(name)
}

pub(crate) trait MeshExt {
    fn search_in_self_and_children<'a>(
        entity: Entity,
        children: &'a Query<&Children>,
        meshes: &'a Assets<Mesh>,
        mesh_handles: &'a Query<&Mesh3d>,
    ) -> Vec<(Entity, &'a Mesh)>;

    fn search_in_children<'a>(
        parent: Entity,
        children: &'a Query<&Children>,
        meshes: &'a Assets<Mesh>,
        mesh_handles: &'a Query<&Mesh3d>,
    ) -> Vec<(Entity, &'a Mesh)>;
}

impl MeshExt for Mesh {
    fn search_in_self_and_children<'a>(
        entity: Entity,
        children_query: &'a Query<&Children>,
        meshes: &'a Assets<Mesh>,
        mesh_handles: &'a Query<&Mesh3d>,
    ) -> Vec<(Entity, &'a Mesh)> {
        let mut result = mesh_handles
            .get(entity)
            .ok()
            .and_then(|mesh_handle| meshes.get(&mesh_handle.0).map(|mesh| (entity, mesh)))
            .into_iter()
            .collect::<Vec<_>>();
        result.append(&mut Self::search_in_children(
            entity,
            children_query,
            meshes,
            mesh_handles,
        ));
        result
    }

    fn search_in_children<'a>(
        parent: Entity,
        children_query: &'a Query<&Children>,
        meshes: &'a Assets<Mesh>,
        mesh_handles: &'a Query<&Mesh3d>,
    ) -> Vec<(Entity, &'a Mesh)> {
        if let Ok(children) = children_query.get(parent) {
            let mut result: Vec<_> = children
                .iter()
                .filter_map(|entity| mesh_handles.get(entity).ok().map(|mesh| (entity, mesh)))
                .map(|(entity, mesh_handle)| {
                    (
                        entity,
                        meshes
                            .get(&mesh_handle.0)
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
                    Self::search_in_children(entity, children_query, meshes, mesh_handles)
                })
                .collect();
            result.append(&mut inner_result);
            result
        } else {
            Vec::new()
        }
    }
}
