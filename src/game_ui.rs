use crate::loading::FontAssets;
use crate::GameState;
use crate::score::Score;
use crate::util::despawn;
use bevy::prelude::*;

#[derive(Component)]
struct GameUI;

#[derive(Component)]
struct PlayButton;

#[derive(Component)]
struct ExitButton;

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_game_ui))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(update_game_ui))
            .add_system_set(SystemSet::on_exit(GameState::Playing).with_system(despawn::<GameUI>));
    }
}

fn spawn_game_ui(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                padding: UiRect::new(Val::Px(20.0), Val::Px(20.0), Val::Px(20.0), Val::Px(20.0)),
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::FlexEnd,
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            color: UiColor(Color::NONE),
            ..Default::default()
        })
        .insert(GameUI)
        .insert(Name::new("Ui"))
        .with_children(|parent| {
            // box around game area so nothing will clip
            // this is something hacky but if it works then it works ¯\_(ツ)_/¯
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    size: Size::new(Val::Px(70.0), Val::Px(750.0)),
                    position: UiRect::new(Val::Px(0.0), Val::Px(0.0), Val::Px(0.0), Val::Px(0.0)),
                    ..Default::default()
                },
                color: UiColor(Color::hex("00010f").unwrap()),
                ..Default::default()
            });

            parent.spawn_bundle(NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    size: Size::new(Val::Px(700.0), Val::Px(10.0)),
                    position: UiRect::new(Val::Px(0.0), Val::Px(0.0), Val::Px(0.0), Val::Px(0.0)),
                    ..Default::default()
                },
                color: UiColor(Color::hex("00010f").unwrap()),
                ..Default::default()
            });

            parent.spawn_bundle(NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    size: Size::new(Val::Px(500.0), Val::Px(750.0)),
                    position: UiRect::new(Val::Px(570.0), Val::Px(0.0), Val::Px(0.0), Val::Px(0.0)),
                    ..Default::default()
                },
                color: UiColor(Color::hex("00010f").unwrap()),
                ..Default::default()
            });

            parent.spawn_bundle(NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    size: Size::new(Val::Px(700.0), Val::Px(10.0)),
                    position: UiRect::new(Val::Px(0.0), Val::Px(0.0), Val::Px(0.0), Val::Px(710.0)),
                    ..Default::default()
                },
                color: UiColor(Color::hex("00010f").unwrap()),
                ..Default::default()
            });

            // score
            parent.spawn_bundle(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: "Score: 0".to_string(),
                        style: TextStyle {
                            font: font_assets.silk.clone(),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    }],
                    alignment: Default::default(),
                },
                ..Default::default()
            });
        });
}

fn update_game_ui(score: Res<Score>, mut text: Query<&mut Text, With<Text>>) {
    text.single_mut().sections[0].value = format!("Score: {}", score.get_score());
}
