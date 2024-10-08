use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use std::collections::{HashMap, HashSet};

use crate::world::components::Ground;

// http://www.mathforgameprogrammers.com/gdc2016/GDC2016_Pittman_Kyle_BuildingABetterJump.pdf
pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("tile-based-game.ldtk"),
        ..Default::default()
    });
}

pub fn despawn(
    mut commands: Commands,
    enemy_entity_query: Query<Entity, With<Handle<LdtkProject>>>,
) {
    if let Ok(world_entity) = enemy_entity_query.get_single() {
        commands.entity(world_entity).despawn_recursive();
    }
}

/// Spawns colliders for the ground of a level.
///
/// One can simply insert a ColliderBundle into the GroundBundle,
/// but this spawns a new collider for EVERY ground tile.
/// This approach leads to bad performance.
///
/// Instead, by marking the ground cells and spawning the collisions later,
/// we can minimize the amount of colliding entities.
///
/// The gound_query obtains all the grid coordinates that are marked with Ground entities.
/// These grid coordinates are assigned to the respective level, which is obtained with the
/// parent_query.
///
/// For each level obtained with the level_query we
///  1. Combine all connected ground cells per row to plates
///  2. Combine all plates per row into rectangles
///  3. Spawn the collider for each rectangle
pub fn add_ground_collider(
    mut commands: Commands,
    ground_query: Query<(&GridCoords, &Parent), Added<Ground>>,
    parent_query: Query<&Parent, Without<Ground>>,
    level_query: Query<(Entity, &LevelIid)>,
    ldtk_projects: Query<&Handle<LdtkProject>>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
) {
    /// Represents a (potentially wide) ground that is 1 cell tall
    /// Used to spawn ground colliders
    #[derive(Clone, Eq, PartialEq, Debug, Default, Hash)]
    struct Plate {
        left: i32,
        right: i32,
    }

    /// A simple rectangle type representing ground of any size
    #[derive(Debug)]
    struct Rect {
        left: i32,
        right: i32,
        top: i32,
        bottom: i32,
    }

    // Stores the grid coordinates belonging to Ground entities for the respective level
    let mut ground_coordinates_for_level: HashMap<Entity, HashSet<GridCoords>> = HashMap::new();

    // For every grid coordinate belonging to a `Ground` entity,
    // we obtain the level of type `Entity` (ground.parent = layer, ground.parent.parent = level),
    // and assign the grid coordinate to the respective level it belongs to.
    ground_query.iter().for_each(|(&grid_coordinates, parent)| {
        // An intgrid tile's direct parent will be a layer entity, not the level entity
        // To get the level entity, you need the tile's grandparent, we obtain using parent_query
        if let Ok(level) = parent_query.get(parent.get()) {
            ground_coordinates_for_level
                .entry(level.get())
                .or_default()
                .insert(grid_coordinates);
        }
    });

    if !ground_query.is_empty() {
        level_query.iter().for_each(|(level_entity, level_iid)| {
            if let Some(ground_coordinates) = ground_coordinates_for_level.get(&level_entity) {
                let ldtk_project = ldtk_project_assets
                    .get(ldtk_projects.single())
                    .expect("Project should be loaded if level has spawned");

                let level = ldtk_project
                    .as_standalone()
                    .get_loaded_level_by_iid(&level_iid.to_string())
                    .expect("Spawned level should exist in LDtk project");

                // Get the levels width and height
                let LayerInstance {
                    c_wid: columns,
                    c_hei: rows,
                    grid_size,
                    ..
                } = level.layer_instances()[0];

                // combine ground grid coordinates into flat "plates" in each individual row
                let mut plate_stack: Vec<Vec<Plate>> = Vec::new();

                for row in 0..rows {
                    let mut row_plates: Vec<Plate> = Vec::new();
                    let mut plate_start = None;

                    // + 1 to the width such the algorithm "terminates" after processing plates that touch the right edge
                    for col in 0..columns + 1 {
                        match (
                            plate_start,
                            ground_coordinates.contains(&GridCoords { x: col, y: row }),
                        ) {
                            (Some(s), false) => {
                                row_plates.push(Plate {
                                    left: s,
                                    right: col - 1,
                                });
                                plate_start = None;
                            }
                            (None, true) => plate_start = Some(col),
                            _ => (),
                        }
                    }
                    plate_stack.push(row_plates);
                }

                // combine "plates" into rectangles across multiple rows
                let mut rect_builder: HashMap<Plate, Rect> = HashMap::new();
                let mut prev_row_plates: Vec<Plate> = vec![];
                let mut ground_rects: Vec<Rect> = vec![];

                // an extra empty row so the algorithm "finishes" the rects that touch the top edge
                plate_stack.push(Vec::new());

                for (row_index, current_row_plates) in plate_stack.into_iter().enumerate() {
                    for prev_row_plate in &prev_row_plates {
                        if !current_row_plates.contains(prev_row_plate) {
                            // remove the finished rect so that the same plate in the future starts a new rect
                            if let Some(rect) = rect_builder.remove(prev_row_plate) {
                                ground_rects.push(rect);
                            }
                        }
                    }
                    for current_row_plate in &current_row_plates {
                        rect_builder
                            .entry(current_row_plate.clone())
                            .and_modify(|rect| rect.top += 1)
                            .or_insert(Rect {
                                bottom: row_index as i32,
                                top: row_index as i32,
                                left: current_row_plate.left,
                                right: current_row_plate.right,
                            });
                    }
                    prev_row_plates = current_row_plates;
                }

                commands.entity(level_entity).with_children(|level| {
                    // Spawn colliders for every rectangle..
                    // Making the collider a child of the level serves two purposes:
                    // 1. Adjusts the transforms to be relative to the level for free
                    // 2. the colliders will be despawned automatically when levels unload
                    for ground_rect in ground_rects {
                        let width = ((ground_rect.right - ground_rect.left + 1) * grid_size) as f32;
                        let height =
                            ((ground_rect.top - ground_rect.bottom + 1) * grid_size) as f32;

                        level.spawn((
                            Collider::cuboid(width / 2., height / 2.),
                            RigidBody::Fixed,
                            Friction::new(1.0),
                            Transform::from_xyz(
                                ((ground_rect.left + ground_rect.right + 1) * grid_size) as f32
                                    / 2.,
                                ((ground_rect.bottom + ground_rect.top + 1) * grid_size) as f32
                                    / 2.,
                                0.,
                            ),
                            GlobalTransform::default(),
                        ));
                    }
                });
            }
        });
    }
}
