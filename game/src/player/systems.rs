use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_spritesheet_animation::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::collider::ColliderBundle;
use crate::fruit::components::Fruit;
use crate::player::components::{Player, PlayerBundle};
use crate::Input;

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut library: ResMut<AnimationLibrary>,
) {
    let texture = asset_server.load("player.png");

    let spritesheet = Spritesheet::new(4, 1);
    let clip = Clip::from_frames(spritesheet.row(0));
    let clip_id = library.register_clip(clip);
    let animation = Animation::from_clip(clip_id);
    let animation_id = library.register_animation(animation);

    let layout = texture_atlas_layouts.add(spritesheet.atlas_layout(128, 128));

    commands.spawn(PlayerBundle {
        player: Player,
        name: Name::new("Player"),
        sprite_bundle: Sprite3dBuilder::from_image(texture)
            .with_atlas(layout)
            .with_transform(Transform::from_xyz(1., 4., 1.))
            .with_custom_size(Vec2::new(1., 1.))
            .build(),
        sprite_sheet_animation: SpritesheetAnimation::from_id(animation_id),
        collider_bundle: ColliderBundle {
            collider: Collider::round_cylinder(0.4, 0.1, 0.1),
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
    });
}

pub fn despawn(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    if let Ok(player) = player_query.get_single() {
        commands.entity(player).despawn();
    }
}

// http://www.mathforgameprogrammers.com/gdc2016/GDC2016_Pittman_Kyle_BuildingABetterJump.pdf
const GRAVITY: f32 = -9.81;
const V: f32 = 3.;

type CharacterController<'a> = (
    &'a ActionState<Input>,
    &'a mut KinematicCharacterController,
    Option<&'a KinematicCharacterControllerOutput>,
);

pub fn move_player(
    mut player_query: Query<CharacterController, With<Player>>,
    time: Res<Time>,
    mut grounded_timer: Local<f32>,
) {
    if let Ok((action, mut controller, output)) = player_query.get_single_mut() {
        let mut translation = Vec3::ZERO;

        // if we are grounded
        if output.map_or(false, |output| output.grounded) {
            translation.y = 0.;
            *grounded_timer = 0.8;
        }

        if *grounded_timer > 0. {
            *grounded_timer -= time.delta_seconds();
            if action.just_pressed(&Input::Jump) {
                translation.y = 20.;
            }
        } else {
            translation.y += GRAVITY * time.delta_seconds() * controller.custom_mass.unwrap_or(1.);
        }

        if action.pressed(&Input::Left) {
            translation.x = -V;
        } else if action.pressed(&Input::Right) {
            translation.x = V;
        }

        if action.just_released(&Input::Left) || action.just_released(&Input::Right) {
            translation.x = 0.;
        }

        if action.pressed(&Input::Up) {
            translation.z = -V;
        } else if action.pressed(&Input::Down) {
            translation.z = V;
        }

        if action.just_released(&Input::Up) || action.just_released(&Input::Down) {
            translation.z = 0.;
        }

        let translation_change = translation * time.delta_seconds();

        controller.translation = match controller.translation {
            Some(existing_translation) => {
                //info!("change: {translation_change}");
                //info!("existing: {existing_translation}");
                Some(existing_translation + translation_change)
            }
            None => {
                //info!("change: {translation_change}");
                Some(translation_change)
            }
        };
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
                info!("Fruit collected");
                commands.entity(collision.entity).despawn()
            }
        }
    }
}
