pub mod animation;
pub mod cli;
pub mod collider;
pub mod color;
pub mod fruit;
pub mod main_menu;
pub mod player;
pub mod ui;

use crate::cli::CliArgs;
use crate::collider::ColliderBundle;
use crate::fruit::components::FruitBundle;

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
            .register_ldtk_int_cell::<GroundBundle>(1)
            .add_plugins(animation::AnimationPlugin)
            .add_plugins(player::PlayerPlugin)
            .add_plugins(fruit::FruitPlugin)
            .add_plugins(ui::UiPlugin)
            .add_systems(OnEnter(AppState::InGame), spawn_ground);
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
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

#[derive(Component, Deref)]
pub struct Textures<T>(pub HashMap<T, Handle<Image>>);

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Ground;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct GroundBundle {
    ground: Ground,
    #[from_int_grid_cell]
    collider: ColliderBundle,
}

fn spawn_ground(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("tile-based-game.ldtk"),
        ..Default::default()
    });
}
