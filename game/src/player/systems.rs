use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_spritesheet_animation::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::collider::ColliderBundle;
use crate::enemy::components::{Enemy, EnemyMovement};
use crate::fruit::components::Fruit;
use crate::player::components::{
    EnemiesInReach, FacingDirection, Movement, Player, PlayerBundle, PlayerReachSensor,
};
use crate::Input;

#[derive(Resource)]
pub struct PlayerAnimations {
    idle: Handle<Animation>,
    jump: Handle<Animation>,
    fall: Handle<Animation>,
    walk: Handle<Animation>,
    jab: Handle<Animation>,
    hook: Handle<Animation>,
    uppercut: Handle<Animation>,
}

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut animations: ResMut<Assets<Animation>>,
) {
    let texture = asset_server.load("bouncer.png");

    let spritesheet = Spritesheet::new(&texture, 61, 1);
    let player_animations = PlayerAnimations {
        idle: animations.add(
            spritesheet
                .create_animation()
                .add_horizontal_strip(0, 0, 12)
                .build(),
        ),
        jump: animations.add(
            spritesheet
                .create_animation()
                .add_horizontal_strip(24, 0, 2)
                .set_repetitions(AnimationRepeat::Times(1))
                .build(),
        ),
        fall: animations.add(
            spritesheet
                .create_animation()
                .add_horizontal_strip(26, 0, 2)
                .set_repetitions(AnimationRepeat::Times(1))
                .build(),
        ),
        walk: animations.add(
            spritesheet
                .create_animation()
                .add_horizontal_strip(28, 0, 16)
                .build(),
        ),
        jab: animations.add(
            spritesheet
                .create_animation()
                .add_horizontal_strip(44, 0, 3)
                .set_repetitions(AnimationRepeat::Times(1))
                .set_duration(AnimationDuration::PerRepetition(200))
                .build(),
        ),
        hook: animations.add(
            spritesheet
                .create_animation()
                .add_horizontal_strip(47, 0, 5)
                .set_repetitions(AnimationRepeat::Times(1))
                .set_duration(AnimationDuration::PerRepetition(200))
                .build(),
        ),
        uppercut: animations.add(
            spritesheet
                .create_animation()
                .add_horizontal_strip(55, 0, 6)
                .set_repetitions(AnimationRepeat::Times(1))
                .set_duration(AnimationDuration::PerRepetition(200))
                .build(),
        ),
    };

    let idle_animation = player_animations.idle.clone();
    commands.insert_resource(player_animations);

    commands
        .spawn(PlayerBundle {
            player: Player,
            name: Name::new("Player"),
            movement: Movement::Idle,
            sprite: spritesheet
                .with_size_hint(32, 32)
                .sprite3d(&mut texture_atlas_layouts)
                .with_custom_size(Vec2::new(2., 2.)),
            transform: Transform::from_xyz(1., 4., 1.),
            sprite_sheet_animation: SpritesheetAnimation::new(idle_animation),
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
            input_map: Input::player_one(),
            velocity: Velocity::default(),
            enemies_in_reach: EnemiesInReach::default(),
            facing_direction: FacingDirection::Right,
        })
        .with_children(|children| {
            children.spawn((
                Collider::cuboid(0.5, 0.5, 0.5),
                Transform::from_xyz(0.2, 0., 0.),
                Sensor,
                PlayerReachSensor,
                ActiveEvents::COLLISION_EVENTS,
            ));
        });
}

pub fn despawn(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    if let Ok(player) = player_query.single() {
        commands.entity(player).despawn();
    }
}

pub fn update_facing_direction(
    mut player_query: Query<(&Velocity, &mut FacingDirection), With<Player>>,
) {
    for (velocity, mut facing_direction) in player_query.iter_mut() {
        if velocity.linvel.x > DELTA {
            facing_direction.set_if_neq(FacingDirection::Right);
        } else if velocity.linvel.x < -DELTA {
            facing_direction.set_if_neq(FacingDirection::Left);
        }
    }
}

const DELTA: f32 = 0.5;

pub fn update_player_animation(
    animations: Res<PlayerAnimations>,
    mut player_query: Query<(&Movement, &mut SpritesheetAnimation), With<Player>>,
) {
    for (movement, mut animation) in player_query.iter_mut() {
        let next_animation = match movement {
            Movement::Idle => &animations.idle,
            Movement::Walk => &animations.walk,
            Movement::Jump => &animations.jump,
            Movement::Fall => &animations.fall,
            Movement::Jab => &animations.jab,
            Movement::Hook => &animations.hook,
            Movement::Uppercut => &animations.uppercut,
        };
        if animation.animation != *next_animation {
            animation.switch(next_animation.clone());
        }
    }
}

pub fn flip_player_sprite(
    mut player_reach_sensor_query: Query<&mut Transform, With<PlayerReachSensor>>,
    mut player_query: Query<(&FacingDirection, &mut Sprite3d), With<Player>>,
) {
    for (facing_direction, mut sprite) in player_query.iter_mut() {
        match facing_direction {
            FacingDirection::Left => sprite.flip_x = true,
            FacingDirection::Right => sprite.flip_x = false,
        }

        if let Ok(mut reach_sensor_transform) = player_reach_sensor_query.single_mut() {
            match facing_direction {
                FacingDirection::Left => {
                    reach_sensor_transform.translation =
                        -1. * reach_sensor_transform.translation.abs()
                }
                FacingDirection::Right => {
                    reach_sensor_transform.translation = reach_sensor_transform.translation.abs()
                }
            }
        }
    }
}

pub fn player_animation_event(
    mut events: MessageReader<AnimationEvent>,
    mut player_query: Query<&mut Movement, With<Player>>,
    mut enemy_query: Query<&mut EnemyMovement, With<Enemy>>,
) {
    for event in events.read() {
        match event {
            AnimationEvent::AnimationEnd { entity, .. } => {
                if let Ok(mut movement) = player_query.get_mut(*entity) {
                    movement.set_if_neq(Movement::Idle);
                }
                if let Ok(mut movement) = enemy_query.get_mut(*entity) {
                    movement.set_if_neq(EnemyMovement::Idle);
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
    if let Ok((action, velocity, mut movement, mut controller, output)) = player_query.single_mut()
    {
        let mut velocity = velocity.linvel;

        let is_grounded = output.map_or(false, |output| output.grounded);
        if is_grounded {
            velocity.y = 0.;
            *grounded_timer = 0.8;
        } else {
            velocity.y += GRAVITY * time.delta_secs() * controller.custom_mass.unwrap_or(1.);
        }

        if *grounded_timer > 0. {
            *grounded_timer -= time.delta_secs();
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
            let mut translation_change = velocity * time.delta_secs();
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

pub fn update_enemies_in_reach(
    mut collision_events: MessageReader<CollisionEvent>,
    player_reach_sensor_query: Query<&ChildOf, With<PlayerReachSensor>>,
    mut player_query: Query<&mut EnemiesInReach, With<Player>>,
) {
    for event in collision_events.read() {
        match event {
            CollisionEvent::Started(entity1, entity2, _) => {
                if let Ok(player_entity) = player_reach_sensor_query
                    .get(*entity1)
                    .or_else(|_| player_reach_sensor_query.get(*entity2))
                {
                    if let Ok(mut enemies_in_reach) = player_query.get_mut(player_entity.parent()) {
                        // Identify the enemy entity (the other collider)
                        let enemy = if player_reach_sensor_query.get(*entity1).is_ok() {
                            *entity2
                        } else {
                            *entity1
                        };
                        enemies_in_reach.0.insert(enemy);
                    }
                }
            }
            CollisionEvent::Stopped(entity1, entity2, _) => {
                if let Ok(player_entity) = player_reach_sensor_query
                    .get(*entity1)
                    .or_else(|_| player_reach_sensor_query.get(*entity2))
                {
                    if let Ok(mut enemies_in_reach) = player_query.get_mut(player_entity.parent()) {
                        // Identify the enemy entity (the other collider)
                        let enemy = if player_reach_sensor_query.get(*entity1).is_ok() {
                            *entity2
                        } else {
                            *entity1
                        };
                        enemies_in_reach.0.remove(&enemy);
                    }
                }
            }
        }
    }
}

pub fn punch(
    player_query: Query<(&ActionState<Input>, &FacingDirection, &EnemiesInReach), With<Player>>,
    mut enemy_impulses: Query<(&mut ExternalImpulse, &mut EnemyMovement), With<Enemy>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    if let Ok((input, facing_direction, enemies_in_reach)) = player_query.single() {
        for enemy in &enemies_in_reach.0 {
            if input.just_pressed(&Input::Hook) {
                if let Ok((mut ext_impulse, mut movement)) = enemy_impulses.get_mut(*enemy) {
                    match facing_direction {
                        FacingDirection::Left => ext_impulse.impulse = Vec3::new(0., 0., 0.2),
                        FacingDirection::Right => ext_impulse.impulse = Vec3::new(0., 0., -0.2),
                    }
                    movement.set_if_neq(EnemyMovement::Hit);
                    commands.spawn((
                        AudioPlayer::<AudioSource>(asset_server.load("punch.mp3")),
                        PlaybackSettings::DESPAWN,
                    ));
                }
            } else if input.just_pressed(&Input::LightPunch) {
                if let Ok((mut ext_impulse, mut movement)) = enemy_impulses.get_mut(*enemy) {
                    match facing_direction {
                        FacingDirection::Left => ext_impulse.impulse = Vec3::new(-0.1, 0., 0.),
                        FacingDirection::Right => ext_impulse.impulse = Vec3::new(0.1, 0., 0.),
                    }
                    movement.set_if_neq(EnemyMovement::Hit);
                    commands.spawn((
                        AudioPlayer::<AudioSource>(asset_server.load("punch.mp3")),
                        PlaybackSettings::DESPAWN,
                    ));
                }
            } else if input.just_pressed(&Input::Uppercut) {
                if let Ok((mut ext_impulse, mut movement)) = enemy_impulses.get_mut(*enemy) {
                    ext_impulse.impulse = Vec3::new(0., 0.1, 0.);
                    movement.set_if_neq(EnemyMovement::Hit);
                    commands.spawn((
                        AudioPlayer::<AudioSource>(asset_server.load("punch.mp3")),
                        PlaybackSettings::DESPAWN,
                    ));
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
    if let Ok(output) = character_controller_outputs.single() {
        for collision in &output.collisions {
            if fruits.get(collision.entity).is_ok() {
                commands.entity(collision.entity).despawn()
            }
        }
    }
}
