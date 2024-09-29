pub mod cli;
pub mod fruit;
pub mod main_menu;
pub mod player;
pub mod ui;

use crate::cli::CliArgs;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

/// Runs the game given the cli arguments parameters.
pub fn run(CliArgs { listen_address }: CliArgs) {
    tracing::info!("Game started {listen_address:?}");
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_state(AppState::MainMenu)
        .add_systems(Startup, spawn_camera)
        .add_plugins(GamePlugin)
        .add_plugins(main_menu::MainMenuPlugin)
        .run();
}

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(GameState::Paused)
            .add_plugins(player::PlayerPlugin)
            .add_plugins(ui::UiPlugin);
            .add_plugins(fruit::FruitPlugin)
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
