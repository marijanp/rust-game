use bevy::prelude::*;

use crate::main_menu::components::{MainMenu, PlayButton, QuitButton};
use crate::{color, AppState, GameState};

pub fn spawn(mut commands: Commands, assets_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(10.0),
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            },
            MainMenu {},
        ))
        .with_children(|parent| {
            // Title
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(ImageBundle {
                        image: assets_server
                            .load("Players/128x256/Green/alienGreen_stand.png")
                            .into(),
                        style: Style {
                            width: Val::Px(128.0),
                            height: Val::Px(256.0),
                            ..default()
                        },
                        ..default()
                    });
                    parent.spawn(TextBundle {
                        text: Text {
                            justify: JustifyText::Center,
                            sections: vec![TextSection {
                                value: "Awesome Game".to_string(),
                                style: TextStyle {
                                    font_size: 64.0,
                                    color: color::PRIMARY_CONTENT,
                                    ..default()
                                },
                            }],
                            ..default()
                        },
                        ..default()
                    });
                });

            // Play Button
            parent
                .spawn((
                    ButtonBundle {
                        background_color: color::PRIMARY.into(),
                        style: Style {
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            width: Val::Px(200.0),
                            height: Val::Px(80.0),
                            ..default()
                        },
                        ..default()
                    },
                    PlayButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            justify: JustifyText::Center,
                            sections: vec![TextSection {
                                value: "Play".to_string(),
                                style: TextStyle {
                                    font_size: 32.0,
                                    color: color::PRIMARY_CONTENT,
                                    ..default()
                                },
                            }],
                            ..default()
                        },
                        ..default()
                    });
                });

            // Quit Button
            parent
                .spawn((
                    ButtonBundle {
                        background_color: color::PRIMARY.into(),
                        style: Style {
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            width: Val::Px(200.0),
                            height: Val::Px(80.0),
                            ..default()
                        },
                        ..default()
                    },
                    QuitButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            justify: JustifyText::Center,
                            sections: vec![TextSection {
                                value: "Quit".to_string(),
                                style: TextStyle {
                                    font_size: 32.0,
                                    color: color::PRIMARY_CONTENT,
                                    ..default()
                                },
                            }],

                            ..default()
                        },
                        ..default()
                    });
                });
        });
}
pub fn despawn(mut commands: Commands, main_menu_query: Query<Entity, With<MainMenu>>) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
}

type ColorForInteraction<'a> = (&'a Interaction, &'a mut BackgroundColor);

pub fn interact_with_play_button(
    mut button_query: Query<ColorForInteraction, (Changed<Interaction>, With<PlayButton>)>,
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
                app_state_next.set(AppState::InGame);
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
