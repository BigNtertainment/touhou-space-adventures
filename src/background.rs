use crate::{game_area::GameArea, GameState};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(create_background));
    }
}

fn create_background(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    game_area: Res<GameArea>,
) {
    // TODO: Add background somehow
    // commands.spawn_bundle(MaterialMesh2dBundle {
    //     mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
    //     transform: Transform::from_translation(game_area.physical_pos().extend(0.))
    //         .with_scale(Vec3::new(game_area.width, game_area.height, 0.)),
    //     material: materials.add(ColorMaterial::from(Color::DARK_GRAY)),
    //     ..default()
    // });
}
