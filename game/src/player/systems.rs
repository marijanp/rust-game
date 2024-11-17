use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_spritesheet_animation::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::collider::ColliderBundle;
use crate::enemy::components::Enemy;
use crate::fruit::components::Fruit;
use crate::player::components::{Movement, Player, PlayerBundle};
use crate::Input;

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut library: ResMut<AnimationLibrary>,
) {
    let texture = asset_server.load("bouncer.png");

    let spritesheet = Spritesheet::new(61, 1);

    // Idle
    let idle_animation_id = library
        .animation_with_name(Movement::Idle)
        .unwrap_or_else(|| {
            let clip = Clip::from_frames(spritesheet.horizontal_strip(0, 0, 12));
            let clip_id = library.register_clip(clip);
            let animation = Animation::from_clip(clip_id);
            let animation_id = library.register_animation(animation);
            library
                .name_animation(animation_id, Movement::Idle)
                .unwrap();
            animation_id
        });

    // Jump
    if library.animation_with_name(Movement::Jump).is_none() {
        let jump_clip = Clip::from_frames(spritesheet.horizontal_strip(24, 0, 2));
        let jump_clip_id = library.register_clip(jump_clip);
        let mut animation = Animation::from_clip(jump_clip_id);
        animation.set_repetitions(AnimationRepeat::Times(1));
        let animation_id = library.register_animation(animation);
        library
            .name_animation(animation_id, Movement::Jump)
            .unwrap();
    }

    // Fall
    if library.animation_with_name(Movement::Fall).is_none() {
        let fall_clip = Clip::from_frames(spritesheet.horizontal_strip(26, 0, 2));
        let fall_clip_id = library.register_clip(fall_clip);
        let mut animation = Animation::from_clip(fall_clip_id);
        animation.set_repetitions(AnimationRepeat::Times(1));
        let animation_id = library.register_animation(animation);
        library
            .name_animation(animation_id, Movement::Fall)
            .unwrap();
    }

    // Run
    if library.animation_with_name(Movement::Walk).is_none() {
        let walk_clip = Clip::from_frames(spritesheet.horizontal_strip(28, 0, 16));
        let walk_clip_id = library.register_clip(walk_clip);
        let animation = Animation::from_clip(walk_clip_id);
        let animation_id = library.register_animation(animation);
        library
            .name_animation(animation_id, Movement::Walk)
            .unwrap();
    }

    // Jab
    if library.animation_with_name(Movement::Jab).is_none() {
        let jab_clip = Clip::from_frames(spritesheet.horizontal_strip(44, 0, 3));
        let jab_clip_id = library.register_clip(jab_clip);
        let mut jab_animation = Animation::from_clip(jab_clip_id);
        jab_animation
            .set_repetitions(AnimationRepeat::Times(1))
            .set_duration(AnimationDuration::PerRepetition(200));
        let jab_animation_id = library.register_animation(jab_animation);
        library
            .name_animation(jab_animation_id, Movement::Jab)
            .unwrap();
    }

    // Hook
    if library.animation_with_name(Movement::Hook).is_none() {
        let hook_clip = Clip::from_frames(spritesheet.horizontal_strip(47, 0, 5));
        let hook_clip_id = library.register_clip(hook_clip);
        let mut hook_animation = Animation::from_clip(hook_clip_id);
        hook_animation
            .set_repetitions(AnimationRepeat::Times(1))
            .set_duration(AnimationDuration::PerRepetition(200));
        let hook_animation_id = library.register_animation(hook_animation);
        library
            .name_animation(hook_animation_id, Movement::Hook)
            .unwrap();
    }

    // Uppercut
    if library.animation_with_name(Movement::Uppercut).is_none() {
        let uppercut_clip = Clip::from_frames(spritesheet.horizontal_strip(55, 0, 6));
        let uppercut_clip_id = library.register_clip(uppercut_clip);
        let mut animation = Animation::from_clip(uppercut_clip_id);
        animation
            .set_repetitions(AnimationRepeat::Times(1))
            .set_duration(AnimationDuration::PerRepetition(200));
        let animation_id = library.register_animation(animation);
        library
            .name_animation(animation_id, Movement::Uppercut)
            .unwrap();
    }

    let layout = texture_atlas_layouts.add(spritesheet.atlas_layout(32, 32));

    commands
        .spawn(PlayerBundle {
            player: Player,
            name: Name::new("Player"),
            movement: Movement::Idle,
            sprite_bundle: Sprite3dBuilder::from_image(texture)
                .with_atlas(layout)
                .with_transform(Transform::from_xyz(1., 4., 1.))
                .with_custom_size(Vec2::new(2., 2.))
                .build(),
            sprite_sheet_animation: SpritesheetAnimation::from_id(idle_animation_id),
            collider_bundle: ColliderBundle {
                collider: Collider::round_cylinder(0.9, 0.05, 0.1),
                rigid_body: RigidBody::KinematicPositionBased,
                active_events: ActiveEvents::COLLISION_EVENTS,
                ..default()
            },
            character_controller: KinematicCharacterController {
                custom_mass: Some(10.0),
                up: Vec3::Y,
                offset: CharacterLength::Absolute(0.01),
                slide: true,
                autostep: Some(CharacterAutostep {
                    max_height: CharacterLength::Relative(0.3),
                    min_width: CharacterLength::Relative(0.5),
                    include_dynamic_bodies: false,
                }),
                // Don’t allow climbing slopes larger than 45 degrees.
                max_slope_climb_angle: 45.0_f32.to_radians(),
                // Automatically slide down on slopes smaller than 30 degrees.
                min_slope_slide_angle: 30.0_f32.to_radians(),
                apply_impulse_to_dynamic_bodies: true,
                snap_to_ground: Some(CharacterLength::Absolute(0.5)),
                ..default()
            },
            input_manager: InputManagerBundle {
                input_map: Input::player_one(),
                ..default()
            },
            velocity: Velocity::default(),
        })
        .with_children(|children| {
            children.spawn((
                Collider::cuboid(0.2, 0.2, 0.2),
                TransformBundle::from(Transform::from_xyz(0.2, 0., 0.)),
                Sensor,
            ));
        });
}

pub fn despawn(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    if let Ok(player) = player_query.get_single() {
        commands.entity(player).despawn_recursive();
    }
}

const DELTA: f32 = 0.5;

pub fn flip_player_sprite(
    library: Res<AnimationLibrary>,
    mut query: Query<
        (
            &Velocity,
            &Movement,
            &mut Sprite3d,
            &mut SpritesheetAnimation,
        ),
        With<Player>,
    >,
) {
    for (velocity, movement, mut sprite, mut animation) in query.iter_mut() {
        if let Some(animation_id) = library.animation_with_name(movement.to_string()) {
            if animation.animation_id != animation_id {
                animation.switch(animation_id);
            }
        }
        if velocity.linvel.x > DELTA {
            sprite.flip_x = false;
        } else if velocity.linvel.x < -DELTA {
            sprite.flip_x = true;
        }
    }
}

pub fn player_animation_event(
    mut events: EventReader<AnimationEvent>,
    library: Res<AnimationLibrary>,
    mut player_query: Query<&mut Movement, With<Player>>,
) {
    for event in events.read() {
        match event {
            AnimationEvent::ClipEnd { animation_id, .. } => {
                if let Some(jab_animation_id) = library.animation_with_name(Movement::Jab) {
                    if *animation_id == jab_animation_id {
                        if let Ok(mut movement) = player_query.get_single_mut() {
                            if *movement == Movement::Jab {
                                *movement = Movement::Idle
                            }
                        }
                    }
                }
                if let Some(hook_animation_id) = library.animation_with_name(Movement::Hook) {
                    if *animation_id == hook_animation_id {
                        if let Ok(mut movement) = player_query.get_single_mut() {
                            if *movement == Movement::Hook {
                                *movement = Movement::Idle
                            }
                        }
                    }
                }
                if let Some(uppercut_animation_id) = library.animation_with_name(Movement::Uppercut)
                {
                    if *animation_id == uppercut_animation_id {
                        if let Ok(mut movement) = player_query.get_single_mut() {
                            if *movement == Movement::Uppercut {
                                *movement = Movement::Idle
                            }
                        }
                    }
                }
            }
            _event => (),
        }
    }
}

type CharacterController<'a> = (
    &'a ActionState<Input>,
    &'a Velocity,
    &'a mut Movement,
    &'a mut KinematicCharacterController,
    Option<&'a KinematicCharacterControllerOutput>,
);

// http://www.mathforgameprogrammers.com/gdc2016/GDC2016_Pittman_Kyle_BuildingABetterJump.pdf
const V: f32 = 3.;

const HEIGHT: f32 = 8.;
const DISTANCE_AT_HEIGHT: f32 = 5.;

const V_0: f32 = (2. * HEIGHT * V) / DISTANCE_AT_HEIGHT;
const GRAVITY: f32 = (-2. * HEIGHT * (V * V)) / (DISTANCE_AT_HEIGHT * DISTANCE_AT_HEIGHT);

pub fn move_player(
    mut player_query: Query<CharacterController, With<Player>>,
    time: Res<Time>,
    mut grounded_timer: Local<f32>,
) {
    if let Ok((action, velocity, mut movement, mut controller, output)) =
        player_query.get_single_mut()
    {
        let mut velocity = velocity.linvel;

        let is_grounded = output.map_or(false, |output| output.grounded);
        if is_grounded {
            velocity.y = 0.;
            *grounded_timer = 0.8;
        } else {
            velocity.y += GRAVITY * time.delta_seconds() * controller.custom_mass.unwrap_or(1.);
        }

        if *grounded_timer > 0. {
            *grounded_timer -= time.delta_seconds();
            if action.just_pressed(&Input::Jump) {
                velocity.y = V_0;
            }
        }

        // Horizontal movement
        if action.pressed(&Input::Left) {
            velocity.x = -V;
        } else if action.pressed(&Input::Right) {
            velocity.x = V;
        }

        if !action.pressed(&Input::Left) && !action.pressed(&Input::Right) {
            velocity.x = 0.;
        }

        // Vertical movement
        if action.pressed(&Input::Up) {
            velocity.z = -V;
        } else if action.pressed(&Input::Down) {
            velocity.z = V;
        }

        if !action.pressed(&Input::Up) && !action.pressed(&Input::Down) {
            velocity.z = 0.;
        }

        let is_moving = !((-DELTA..=DELTA).contains(&velocity.x)
            && (-DELTA..=DELTA).contains(&velocity.z)
            && (-DELTA..=DELTA).contains(&velocity.y));

        if *movement != Movement::Jab
            && *movement != Movement::Hook
            && *movement != Movement::Uppercut
        {
            if action.just_pressed(&Input::LightPunch) {
                *movement = Movement::Jab;
            } else if action.just_pressed(&Input::Hook) {
                *movement = Movement::Hook;
            } else if action.just_pressed(&Input::Uppercut) {
                *movement = Movement::Uppercut;
            } else if !is_grounded {
                if velocity.y > DELTA {
                    *movement = Movement::Jump;
                } else if velocity.y < -DELTA {
                    // *movement = Movement::Fall;
                }
            } else if !is_moving {
                *movement = Movement::Idle;
            } else {
                *movement = Movement::Walk;
            }
        }

        if is_moving {
            let mut translation_change = velocity * time.delta_seconds();
            controller.translation = match controller.translation {
                Some(existing_translation) => {
                    translation_change += existing_translation;
                    debug!("existing {existing_translation} updated: {translation_change}");
                    Some(translation_change)
                }
                None => {
                    debug!("{translation_change}");
                    Some(translation_change)
                }
            };
        }
    }
}

pub fn punch(
    character_controller_outputs: Query<
        (
            &ActionState<Input>,
            &KinematicCharacterControllerOutput,
            &Velocity,
        ),
        (With<Player>, Changed<KinematicCharacterControllerOutput>),
    >,
    mut enemy_impulses: Query<&mut ExternalImpulse, With<Enemy>>,
) {
    if let Ok((input, output, player_velocity)) = character_controller_outputs.get_single() {
        if input.just_pressed(&Input::Hook) {
            for collision in &output.collisions {
                if let Ok(mut ext_impulse) = enemy_impulses.get_mut(collision.entity) {
                    if player_velocity.linvel.x > DELTA {
                        ext_impulse.impulse = Vec3::new(0., 0., -0.2);
                    } else if player_velocity.linvel.x < -DELTA {
                        ext_impulse.impulse = Vec3::new(0., 0., 0.2);
                    }
                }
            }
        } else if input.just_pressed(&Input::LightPunch) {
            for collision in &output.collisions {
                if let Ok(mut ext_impulse) = enemy_impulses.get_mut(collision.entity) {
                    if player_velocity.linvel.x > DELTA {
                        ext_impulse.impulse = Vec3::new(0.2, 0., 0.);
                    } else if player_velocity.linvel.x < -DELTA {
                        ext_impulse.impulse = Vec3::new(-0.2, 0., 0.);
                    }
                }
            }
        } else if input.just_pressed(&Input::Uppercut) {
            for collision in &output.collisions {
                if let Ok(mut ext_impulse) = enemy_impulses.get_mut(collision.entity) {
                    ext_impulse.impulse = Vec3::new(0., 0.1, 0.);
                }
            }
        }
    }
}

pub fn collect_fruits(
    mut commands: Commands,
    character_controller_outputs: Query<
        &KinematicCharacterControllerOutput,
        (With<Player>, Changed<KinematicCharacterControllerOutput>),
    >,
    fruits: Query<Entity, With<Fruit>>,
) {
    if let Ok(output) = character_controller_outputs.get_single() {
        for collision in &output.collisions {
            if fruits.get(collision.entity).is_ok() {
                commands.entity(collision.entity).despawn()
            }
        }
    }
}
