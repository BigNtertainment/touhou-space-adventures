use crate::actions::Actions;
use crate::enemy::Enemy;
use crate::game_area::{GameArea, GameAreaBound};
use crate::game_area::{GameArea, GameAreaBound, GameAreaBoundLabel};
use crate::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy_rapier2d::prelude::Collider;

pub struct PlayerPlugin;

#[derive(Component, Default)]
pub struct Player;

#[derive(Component, Default)]
pub struct Speed(f32);

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_player))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(move_player)
                    .before(GameAreaBoundLabel),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(move_player)
                    .with_system(die),
            )
            .add_system_set(SystemSet::on_exit(GameState::Playing).with_system(drop_player));
    }
}

#[derive(Bundle)]
struct PlayerBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
    player: Player,
    speed: Speed,
    bound: GameAreaBound,
    collider: Collider,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            sprite_bundle: SpriteBundle {
                transform: Transform::from_scale(Vec3::new(0.2, 0.2, 1.)),
                ..default()
            },
            player: Player::default(),
            speed: Speed(150.),
            bound: GameAreaBound::default(),
            collider: Collider::cuboid(30., 30.),
        }
    }
}

fn drop_player(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    commands.entity(player_query.single()).despawn_recursive();
}

fn spawn_player(mut commands: Commands, textures: Res<TextureAssets>, game_area: Res<GameArea>) {
    commands.spawn_bundle(PlayerBundle {
        sprite_bundle: SpriteBundle {
            transform: Transform::from_scale(Vec3::new(0.2, 0.2, 1.)).with_translation(
                (game_area.physical_pos() - Vec2::new(0., 0.25 * game_area.height)).extend(1.),
            ),
            texture: textures.player_texture.clone(),
            ..default()
        },
        ..default()
    });
}

fn move_player(
    mut player_query: Query<(&mut Transform, &Speed), With<Player>>,
    time: Res<Time>,
    actions: Res<Actions>,
) {
    if actions.player_movement.is_none() {
        return;
    }

    let (mut player_transform, speed) = player_query.single_mut();

    // Player movement
    let movement = Vec3::new(
        actions.player_movement.unwrap().x * speed.0 * time.delta_seconds(),
        actions.player_movement.unwrap().y * speed.0 * time.delta_seconds(),
        0.,
    );

    player_transform.translation += movement;
}

fn die(
    player_query: Query<&Transform, With<Player>>,
    enemies_query: Query<&Transform, With<Enemy>>,
) {
    let player_transform = player_query.single();

    for enemy_transform in &enemies_query {
        if collide(
            player_transform.translation,
            Vec2::splat(30.),
            enemy_transform.translation,
            Vec2::splat(30.),
        )
        .is_some()
        {
            println!("Player is dead");
            break;
        }
    }
}
