use bevy::prelude::*;

use crate::main_menu::components::{MainMenu, PlayButton, QuitButton};
use crate::{color, AppState, GameState};

pub fn spawn(mut commands: Commands) {
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
            MainMenu,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Awesome Game"),
                TextFont {
                    font_size: 64.0,
                    ..default()
                },
                TextColor(color::PRIMARY_CONTENT),
                TextLayout::new_with_justify(Justify::Center),
            ));

            spawn_menu_button(parent, "Play", PlayButton);
            spawn_menu_button(parent, "Quit", QuitButton);
        });
}

fn spawn_menu_button<T: Component>(parent: &mut ChildSpawnerCommands, label: &str, marker: T) {
    parent
        .spawn((
            Button,
            Node {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Px(200.0),
                height: Val::Px(80.0),
                ..default()
            },
            BackgroundColor(color::PRIMARY),
            marker,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(label),
                TextFont {
                    font_size: 32.0,
                    ..default()
                },
                TextColor(color::PRIMARY_CONTENT),
                TextLayout::new_with_justify(Justify::Center),
            ));
        });
}

pub fn despawn(mut commands: Commands, main_menu_query: Query<Entity, With<MainMenu>>) {
    if let Ok(main_menu_entity) = main_menu_query.single() {
        commands.entity(main_menu_entity).despawn();
    }
}

type ColorForInteraction<'a> = (&'a Interaction, &'a mut BackgroundColor);

pub fn interact_with_play_button(
    mut button_query: Query<ColorForInteraction, (Changed<Interaction>, With<PlayButton>)>,
    mut app_state_next: ResMut<NextState<AppState>>,
    mut game_state_next: ResMut<NextState<GameState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.single_mut() {
        match interaction {
            Interaction::None => {
                *background_color = color::PRIMARY.into();
            }
            Interaction::Pressed => {
                *background_color = color::PRIMARY.into();
                app_state_next.set(AppState::InGame);
                game_state_next.set(GameState::SpawningLevelColliders);
            }
            Interaction::Hovered => {
                *background_color = color::PRIMARY_HOVER.into();
            }
        }
    }
}

pub fn interact_with_quit_button(
    mut app_exit_event_writer: MessageWriter<AppExit>,
    mut button_query: Query<ColorForInteraction, (Changed<Interaction>, With<QuitButton>)>,
) {
    if let Ok((interaction, mut background_color)) = button_query.single_mut() {
        match interaction {
            Interaction::None => {
                *background_color = color::PRIMARY.into();
            }
            Interaction::Pressed => {
                *background_color = color::PRIMARY.into();
                app_exit_event_writer.write(AppExit::Success);
            }
            Interaction::Hovered => {
                *background_color = color::PRIMARY_HOVER.into();
            }
        }
    }
}
