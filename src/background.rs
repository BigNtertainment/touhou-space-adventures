use crate::{game_area::GameArea, loading::TextureAssets, GameState, util::despawn};
use bevy::prelude::*;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(create_background))
            .add_system_set(SystemSet::on_exit(GameState::Playing).with_system(despawn::<Background>));
    }
}

#[derive(Component)]
struct Background;

fn create_background(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    game_area: Res<GameArea>,
) {
    commands.spawn_bundle(SpriteBundle {
        texture: textures.game_area.clone(),
        transform: Transform::from_translation(game_area.physical_pos().extend(0.)),
        ..default()
    }).insert(Background);
}
