use crate::actions::Actions;
use crate::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Speed(f32);

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_player))
            .add_system_set(SystemSet::on_update(GameState::Playing)
                .with_system(move_player)
            )
            .add_system_set(SystemSet::on_exit(GameState::Playing).with_system(drop_player));
    }
}

fn drop_player(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    commands.entity(player_query.single()).despawn_recursive();
}

fn spawn_player(mut commands: Commands, textures: Res<TextureAssets>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: textures.texture_bevy.clone(),
            transform: Transform::from_translation(Vec3::new(1., 0., 1.))
                .with_scale(Vec3::new(0.2, 0.2, 1.)),
            ..Default::default()
        })
        .insert(Player)
        .insert(Speed(150.));
}

fn move_player(
    mut player_query: Query<(&mut Transform, &Handle<Image>, &Speed), With<Player>>,
    time: Res<Time>,
    actions: Res<Actions>,
    images: Res<Assets<Image>>,
) {
    if actions.player_movement.is_none() {
        return;
    }

    let (mut player_transform, texture, speed) = player_query.single_mut();

    // Player movement
    let movement = Vec3::new(
        actions.player_movement.unwrap().x * speed.0 * time.delta_seconds(),
        actions.player_movement.unwrap().y * speed.0 * time.delta_seconds(),
        0.,
    );

    player_transform.translation += movement;

    // Clamp player transform to game area size
    let texture_size = images.get(texture).unwrap().texture_descriptor.size;

    let player_size = Vec2::new(
        texture_size.width as f32 * player_transform.scale.x.abs(),
        texture_size.height as f32 as f32 * player_transform.scale.y.abs(),
    );

    // TODO: Put these values somewhere
    let game_area = Vec2::new(300., 600.);

    let bounding_box = Vec2::new(game_area.x - player_size.x, game_area.y - player_size.y);

    player_transform.translation.x = player_transform
        .translation
        .x
        .clamp(-bounding_box.x / 2.0, bounding_box.x / 2.0);

    player_transform.translation.y = player_transform
        .translation
        .y
        .clamp(-bounding_box.y / 2.0, bounding_box.y / 2.0);
}
