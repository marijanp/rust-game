use bevy::prelude::*;

use super::components::{MainMenuButton, PauseMenu, QuitButton, ResumeButton};
use crate::{color, AppState, GameState};

pub fn spawn(mut commands: Commands, assets_server: Res<AssetServer>) {
    commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(10.0),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            PauseMenu {},
        ))
        .with_children(|parent| {
            parent.spawn(AudioPlayer::new(assets_server.load("ambience.mp3")));
            // Title
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Pause"),
                        TextLayout::default().with_justify(JustifyText::Center),
                        TextFont::default().with_font_size(64.),
                        TextColor::from(color::PRIMARY_CONTENT),
                    ));
                });

            // Resume Button
            parent
                .spawn((
                    Button,
                    BackgroundColor::from(color::PRIMARY),
                    Node {
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        width: Val::Px(200.0),
                        height: Val::Px(80.0),
                        ..default()
                    },
                    ResumeButton,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Resume"),
                        TextLayout::default().with_justify(JustifyText::Center),
                        TextFont::default().with_font_size(32.),
                        TextColor::from(color::PRIMARY_CONTENT),
                    ));
                });

            // MainMenu Button
            parent
                .spawn((
                    Button,
                    BackgroundColor::from(color::PRIMARY),
                    Node {
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        width: Val::Px(200.0),
                        height: Val::Px(80.0),
                        ..default()
                    },
                    MainMenuButton,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Main Menu"),
                        TextLayout::default().with_justify(JustifyText::Center),
                        TextFont::default().with_font_size(32.),
                        TextColor::from(color::PRIMARY_CONTENT),
                    ));
                });

            // Quit Button
            parent
                .spawn((
                    Button,
                    BackgroundColor::from(color::PRIMARY),
                    Node {
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        width: Val::Px(200.0),
                        height: Val::Px(80.0),
                        ..default()
                    },
                    QuitButton,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Quit"),
                        TextLayout::default().with_justify(JustifyText::Center),
                        TextFont::default().with_font_size(32.),
                        TextColor::from(color::PRIMARY_CONTENT),
                    ));
                });
        });
}
pub fn despawn(mut commands: Commands, pause_menu_query: Query<Entity, With<PauseMenu>>) {
    if let Ok(pause_menu_entity) = pause_menu_query.get_single() {
        commands.entity(pause_menu_entity).despawn_recursive();
    }
}

type ColorForInteraction<'a> = (&'a Interaction, &'a mut BackgroundColor);

pub fn interact_with_resume_button(
    mut button_query: Query<ColorForInteraction, (Changed<Interaction>, With<ResumeButton>)>,
    mut game_state_next: ResMut<NextState<GameState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match interaction {
            Interaction::None => {
                *background_color = color::PRIMARY.into();
            }
            Interaction::Pressed => {
                *background_color = color::PRIMARY.into();
                game_state_next.set(GameState::Running);
            }
            Interaction::Hovered => {
                *background_color = color::PRIMARY_HOVER.into();
            }
        }
    }
}

pub fn interact_with_quit_button(
    mut app_exit_event_writer: EventWriter<AppExit>,
    mut button_query: Query<ColorForInteraction, (Changed<Interaction>, With<QuitButton>)>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match interaction {
            Interaction::None => {
                *background_color = color::PRIMARY.into();
            }
            Interaction::Pressed => {
                *background_color = color::PRIMARY.into();
                app_exit_event_writer.send(AppExit::Success);
            }
            Interaction::Hovered => {
                *background_color = color::PRIMARY_HOVER.into();
            }
        }
    }
}

pub fn interact_with_main_menu_button(
    mut button_query: Query<ColorForInteraction, (Changed<Interaction>, With<MainMenuButton>)>,
    mut app_state_next: ResMut<NextState<AppState>>,
    mut game_state_next: ResMut<NextState<GameState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match interaction {
            Interaction::None => {
                *background_color = color::PRIMARY.into();
            }
            Interaction::Pressed => {
                *background_color = color::PRIMARY.into();
                app_state_next.set(AppState::MainMenu);
                game_state_next.set(GameState::Running);
            }
            Interaction::Hovered => {
                *background_color = color::PRIMARY_HOVER.into();
            }
        }
    }
}

pub fn toggle_pause_menu(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    game_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.any_just_pressed(vec![KeyCode::Escape]) {
        match game_state.get() {
            GameState::Running => {
                next_state.set(GameState::Paused);
            }
            GameState::Paused => {
                next_state.set(GameState::Running);
            }
        }
    }
}
