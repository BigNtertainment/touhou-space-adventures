use crate::loading::FontAssets;
use crate::GameState;
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
                padding: UiRect::new(Val::Px(20.0),Val::Px(20.0),Val::Px(20.0),Val::Px(20.0)),
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
