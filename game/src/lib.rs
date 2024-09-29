pub mod animation;
pub mod cli;
pub mod color;
pub mod fruit;
pub mod main_menu;
pub mod player;
pub mod ui;

use crate::cli::CliArgs;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
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
            .add_plugins(RapierDebugRenderPlugin::default())
            .add_plugins(InputManagerPlugin::<Action>::default())
            .add_plugins(animation::AnimationPlugin)
            .add_plugins(player::PlayerPlugin)
            .add_plugins(fruit::FruitPlugin)
            .add_plugins(ui::UiPlugin)
            .add_systems(OnEnter(AppState::InGame), spawn_ground);
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
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

#[derive(Component, Deref)]
pub struct Textures<T>(pub HashMap<T, Handle<Image>>);

fn spawn_ground(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    /* Create the ground. */
    commands.spawn((
        Collider::cuboid(window.width(), 10.),
        TransformBundle::from(Transform::from_xyz(0., 0., 0.)),
    ));

    /* Create the bouncing ball. */
    commands.spawn((
        RigidBody::Dynamic,
        Collider::ball(50.0),
        Restitution::coefficient(0.9),
        TransformBundle::from(Transform::from_xyz(
            window.width() / 2.,
            window.height(),
            0.,
        )),
    ));
}
