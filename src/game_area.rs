use bevy::prelude::*;

use crate::{GameState, HEIGHT, WIDTH};

pub struct GameAreaPlugin;

impl Plugin for GameAreaPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(register_game_area).add_system_set(
            SystemSet::on_update(GameState::Playing).with_system(bound_to_game_area),
        );
    }
}

#[derive(Component, Default)]
pub struct GameAreaBound;

pub struct GameArea {
    pub width: f32,
    pub height: f32,
    /// Position relative to the window
    position: Vec2,
}

impl GameArea {
    pub fn physical_pos(&self) -> Vec2 {
        self.position * Vec2::new(WIDTH, HEIGHT)
    }
}

fn register_game_area(mut commands: Commands) {
    commands.insert_resource(GameArea {
        width: 300.,
        height: 600.,
        position: Vec2::new(-0.25, 0.),
    })
}

pub fn bound_to_game_area(
    mut bound_entities: Query<(&mut Transform, Option<&Handle<Image>>), With<GameAreaBound>>,
    images: Res<Assets<Image>>,
    game_area: Res<GameArea>,
) {
    for (mut transform, texture) in bound_entities.iter_mut() {
        let texture_size = if let Some(texture) = texture {
            let texture_size = images.get(texture).unwrap().texture_descriptor.size;

            Vec2::new(texture_size.width as f32, texture_size.height as f32)
        } else {
            Vec2::splat(1.)
        };

        let size = Vec2::new(
            texture_size.x * transform.scale.x.abs(),
            texture_size.y * transform.scale.y.abs(),
        );

        let bounding_box_origin = game_area.physical_pos();
        let bounding_box_size = Vec2::new(game_area.width - size.x, game_area.height - size.y);

        let half_bounding_box_size = bounding_box_size / 2.;

        let absolute_position = transform.translation.truncate() - bounding_box_origin;

        let bound_pos = Vec2::new(
            absolute_position
                .x
                .clamp(-half_bounding_box_size.x, half_bounding_box_size.x),
            absolute_position
                .y
                .clamp(-half_bounding_box_size.y, half_bounding_box_size.y),
        ) + bounding_box_origin;

        transform.translation = bound_pos.extend(0.);
    }
}
