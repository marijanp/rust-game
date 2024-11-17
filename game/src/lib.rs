pub mod cli;
pub mod collider;
pub mod color;
pub mod enemy;
pub mod fruit;
pub mod main_menu;
pub mod player;
pub mod ui;
pub mod world;

use crate::cli::CliArgs;
use crate::collider::ColliderBundle;
use crate::player::components::Player;

use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::window::PrimaryWindow;
#[cfg(feature = "dev")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use bevy_spritesheet_animation::prelude::SpritesheetAnimationPlugin;
use blenvy::*;
use leafwing_input_manager::prelude::*;

// App

#[derive(States, Clone, Debug, Copy, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
    GameOver,
}

/// Runs the game given the cli arguments parameter
pub fn run(CliArgs { listen_address }: CliArgs) {
    info!("Game started {listen_address:?}");
    App::new()
        // default_nearest disables linear texture filtering, we need this because of pixel art
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_state(AppState::MainMenu)
        .add_plugins(main_menu::MainMenuPlugin)
        .add_plugins(GamePlugin)
        .run();
}

// Game

#[derive(States, Clone, Debug, Copy, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    Running,
    Paused,
}

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(GameState::Paused)
            .add_systems(Startup, spawn_camera)
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugins(BlenvyPlugin::default())
            .add_plugins(InputManagerPlugin::<Input>::default())
            .add_plugins(SpritesheetAnimationPlugin)
            .add_plugins(ui::UiPlugin)
            .add_plugins(world::WorldPlugin)
            .add_plugins(player::PlayerPlugin)
            .add_plugins(fruit::FruitPlugin)
            .add_plugins(enemy::EnemyPlugin)
            .add_systems(Update, (touch_system, move_camera));
        #[cfg(feature = "dev")]
        app.add_plugins(RapierDebugRenderPlugin::default())
            .add_plugins(WorldInspectorPlugin::new());
    }
}

// Input manager

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum Input {
    Up,
    Down,
    Left,
    Right,
    Jump,
    Fall,
    LightPunch,
    Hook,
    Uppercut,
}

impl Input {
    pub fn player_one() -> InputMap<Input> {
        InputMap::new([
            (Input::Up, KeyCode::KeyW),
            (Input::Up, KeyCode::ArrowUp),
            (Input::Down, KeyCode::KeyS),
            (Input::Down, KeyCode::ArrowDown),
            (Input::Left, KeyCode::KeyA),
            (Input::Left, KeyCode::ArrowLeft),
            (Input::Right, KeyCode::KeyD),
            (Input::Right, KeyCode::ArrowRight),
            (Input::Jump, KeyCode::Space),
            (Input::LightPunch, KeyCode::KeyJ),
            (Input::Hook, KeyCode::KeyH),
            (Input::Uppercut, KeyCode::KeyU),
        ])
    }
}

// Camera

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    let width = window.width();
    let height = window.height();
    let physical_width = window.physical_width();
    let physical_height = window.physical_height();
    info!("logical window dimensions: {width}x{height}");
    info!("physical window dimensions: {physical_width}x{physical_height}");
    let common_transform =
        Transform::from_xyz(0., 8., 10.).looking_at(Vec3::new(0.0, 4., 0.0), Vec3::Y);
    // light
    commands.spawn(DirectionalLightBundle {
        transform: common_transform,
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::WindowSize(32.),
            ..default()
        }
        .into(),
        transform: common_transform,
        ..default()
    });
}

fn move_camera(
    mut camera: Query<&mut Transform, (With<Camera3d>, Without<Player>)>,
    player: Query<&Transform, (With<Player>, Without<Camera3d>)>,
    time: Res<Time>,
) {
    if let (Ok(mut camera), Ok(player)) = (camera.get_single_mut(), player.get_single()) {
        let direction = Vec3::new(
            player.translation.x,
            camera.translation.y,
            camera.translation.z,
        );
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
    mut action_state_query: Query<&mut ActionState<Input>>,
) {
    let window = window_query.get_single().unwrap();
    if let Ok(mut action_state) = action_state_query.get_single_mut() {
        if touches
            .iter()
            .any(|touch| touch.position().x < window.width() / 2.)
        {
            action_state.press(&Input::Left);
        } else if touches
            .iter()
            .any(|touch| touch.position().x >= window.width() / 2.)
        {
            action_state.press(&Input::Right);
        } else if touches
            .iter()
            .any(|touch| touch.position().y < window.height() / 2.)
        {
            action_state.press(&Input::Up);
        } else if touches
            .iter()
            .any(|touch| touch.position().y >= window.height() / 2.)
        {
            action_state.press(&Input::Down);
        }
    };
}
