use crate::loading::{FontAssets, TextureAssets};
use crate::score::Score;
use crate::GameState;
use bevy::prelude::*;

#[derive(Component)]
struct GameOverUI;

#[derive(Component)]
struct PlayButton;

#[derive(Component)]
struct ExitButton;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ButtonColors>()
            .add_system_set(SystemSet::on_enter(GameState::GameOver).with_system(setup_end))
            .add_system_set(
                SystemSet::on_update(GameState::GameOver)
                    .with_system(click_play_button)
                    .with_system(click_exit_button),
            )
            .add_system_set(SystemSet::on_exit(GameState::GameOver).with_system(cleanup_end));
    }
}

struct ButtonColors {
    normal: UiColor,
    hovered: UiColor,
}

impl Default for ButtonColors {
    fn default() -> Self {
        ButtonColors {
            normal: Color::rgb(0.15, 0.15, 0.15).into(),
            hovered: Color::rgb(0.25, 0.25, 0.25).into(),
        }
    }
}

fn setup_end(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    button_colors: Res<ButtonColors>,
    score: Res<Score>,
    texture_assets: Res<TextureAssets>,
) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            color: UiColor(Color::hex("00010f").unwrap()),
            ..Default::default()
        })
        .insert(GameOverUI)
        .insert(Name::new("Ui"))
        .with_children(|parent| {
            // the background
            parent.spawn_bundle(ImageBundle {
                style: Style {
                    ..Default::default()
                },
                image: UiImage(texture_assets.game_over_bg.clone()),
                ..Default::default()
            });

            // title text
            parent.spawn_bundle(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: format!(
                            "You lost with score of: {}",
                            score.get_score()
                        ),
                        style: TextStyle {
                            font: font_assets.silk.clone(),
                            font_size: 96.0,
                            color: Color::rgb(1.0, 1.0, 1.0),
                        },
                    }],
                    alignment: Default::default(),
                },
                ..Default::default()
            });

            // play button
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(350.0), Val::Px(50.0)),
                        margin: UiRect::all(Val::Auto),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    color: button_colors.normal,
                    ..Default::default()
                })
                .insert(PlayButton)
                .insert(Name::new("PlayButton"))
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text {
                            sections: vec![TextSection {
                                value: "Play Again".to_string(),
                                style: TextStyle {
                                    font: font_assets.silk_bold.clone(),
                                    font_size: 40.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                            }],
                            alignment: Default::default(),
                        },
                        ..Default::default()
                    });
                });

            // exit button
            // #[cfg(not(target_arch = "wasm32"))]
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(350.0), Val::Px(50.0)),
                        margin: UiRect::all(Val::Auto),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    color: button_colors.normal,
                    ..Default::default()
                })
                .insert(ExitButton)
                .insert(Name::new("ExitButton"))
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text {
                            sections: vec![TextSection {
                                value: "Back to Menu".to_string(),
                                style: TextStyle {
                                    font: font_assets.silk_bold.clone(),
                                    font_size: 40.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                            }],
                            alignment: Default::default(),
                        },
                        ..Default::default()
                    });
                });
        });
}

fn click_play_button(
    button_colors: Res<ButtonColors>,
    mut state: ResMut<State<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<PlayButton>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                state.set(GameState::Playing).unwrap();
            }
            Interaction::Hovered => {
                *color = button_colors.hovered;
            }
            Interaction::None => {
                *color = button_colors.normal;
            }
        }
    }
}

fn click_exit_button(
    button_colors: Res<ButtonColors>,
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<ExitButton>),
    >,
    mut state: ResMut<State<GameState>>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                state.set(GameState::Menu).unwrap();
            }
            Interaction::Hovered => {
                *color = button_colors.hovered;
            }
            Interaction::None => {
                *color = button_colors.normal;
            }
        }
    }
}

fn cleanup_end(mut commands: Commands, ui: Query<Entity, With<GameOverUI>>) {
    commands.entity(ui.single()).despawn_recursive();
}
