pub mod animation;
pub mod cli;
pub mod collider;
pub mod color;
pub mod fruit;
pub mod main_menu;
pub mod player;
pub mod ui;
pub mod world;

use crate::cli::CliArgs;
use crate::collider::ColliderBundle;
use crate::fruit::components::FruitBundle;
use crate::player::components::{Player, PlayerBundle};
use crate::world::components::GroundBundle;

use bevy::core_pipeline::bloom::BloomSettings;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;
use std::collections::HashMap;

/// Runs the game given the cli arguments parameters.
pub fn run(CliArgs { listen_address }: CliArgs) {
    tracing::info!("Game started {listen_address:?}");
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_state(AppState::MainMenu)
        .add_plugins(main_menu::MainMenuPlugin)
        .add_plugins(GamePlugin)
        .add_systems(Startup, spawn_camera)
        .run();
}

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(GameState::Paused)
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(16.0))
            .add_plugins(InputManagerPlugin::<Action>::default())
            .add_plugins(LdtkPlugin)
            .insert_resource(LevelSelection::index(0))
            .register_ldtk_entity::<FruitBundle>("Cherry")
            .register_ldtk_entity::<PlayerBundle>("Player")
            .register_ldtk_int_cell::<GroundBundle>(1)
            .add_plugins(world::WorldPlugin)
            .add_plugins(animation::AnimationPlugin)
            .add_plugins(player::PlayerPlugin)
            .add_plugins(fruit::FruitPlugin)
            .add_plugins(ui::UiPlugin)
            .insert_resource(Tilesets::<player::components::Movement>::default())
            .add_systems(Update, (touch_system, update_camera));
        #[cfg(debug_assertions)]
        app.add_plugins(RapierDebugRenderPlugin::default());
    }
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum Action {
    Left,
    Right,
    Jump,
    Fall,
}

impl Action {
    pub fn player_one() -> InputMap<Action> {
        InputMap::new([
            (Action::Left, KeyCode::KeyA),
            (Action::Left, KeyCode::ArrowLeft),
            (Action::Right, KeyCode::KeyD),
            (Action::Right, KeyCode::ArrowRight),
            (Action::Jump, KeyCode::KeyW),
            (Action::Jump, KeyCode::Space),
            (Action::Jump, KeyCode::ArrowUp),
            (Action::Fall, KeyCode::KeyS),
            (Action::Fall, KeyCode::ArrowDown),
        ])
    }
}

#[derive(States, Clone, Debug, Copy, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
    GameOver,
}

#[derive(States, Clone, Debug, Copy, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    Running,
    Paused,
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    let width = window.width();
    let height = window.height();
    let physical_width = window.physical_width();
    let physical_height = window.physical_height();
    info!("logical: {width}x{height}");
    info!("physical: {physical_width}x{physical_height}");
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true, // HDR is required for the bloom effect
                ..default()
            },
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        },
        BloomSettings::NATURAL,
    ));
}

fn update_camera(
    mut camera: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player: Query<&Transform, (With<Player>, Without<Camera2d>)>,
    time: Res<Time>,
) {
    if let (Ok(mut camera), Ok(player)) = (camera.get_single_mut(), player.get_single()) {
        let Vec3 { x, y, .. } = player.translation;
        let direction = Vec3::new(x, y, camera.translation.z);
        // Applies a smooth effect to camera movement using interpolation between
        // the camera position and the player position on the x and y axes.
        // Here we use the in-game time, to get the elapsed time (in seconds)
        // since the previous update. This avoids jittery movement when tracking
        // the player.
        camera.translation = camera
            .translation
            .lerp(direction, time.delta_seconds() * 2.);
    }
}

fn touch_system(
    touches: Res<Touches>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut action_state_query: Query<&mut ActionState<Action>>,
    player_query: Query<&Transform, With<Player>>,
) {
    let window = window_query.get_single().unwrap();
    if let Ok(player) = player_query.get_single() {
        if let Ok(mut action_state) = action_state_query.get_single_mut() {
            if touches
                .iter()
                .any(|touch| touch.position().x < player.translation.x)
            {
                action_state.press(&Action::Left);
            } else if touches
                .iter()
                .any(|touch| touch.position().x >= player.translation.x)
            {
                action_state.press(&Action::Right);
            }
            if touches.iter().count() + touches.iter_just_pressed().count() >= 2 {
                if touches
                    .iter_just_pressed()
                    .any(|touch| touch.position().y < window.height() / 2.)
                {
                    action_state.press(&Action::Jump);
                } else if touches
                    .iter_just_pressed()
                    .any(|touch| touch.position().y >= window.height() / 2.)
                {
                    action_state.press(&Action::Fall);
                }
            }
        }
    };
}

#[derive(Default, Deref, Resource)]
pub struct Tilesets<T>(pub HashMap<T, Handle<Image>>);
