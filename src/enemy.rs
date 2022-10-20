use crate::character::MoveSpeed;
use crate::game_area::GameArea;
use crate::GameState;
use bevy::prelude::*;

use crate::loading::TextureAssets;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_enemies))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(move_enemy));
    }
}

#[derive(Component)]
pub struct Enemy;

#[derive(Bundle)]
pub struct EnemyBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
    enemy: Enemy,
    move_speed: MoveSpeed,
}

fn spawn_enemies(mut commands: Commands, textures: Res<TextureAssets>, game_area: Res<GameArea>) {
    // TODO: Think how to implement logic that spawns waves or somtehing
    for i in -1..2 {
        commands.spawn_bundle(EnemyBundle {
            sprite_bundle: SpriteBundle {
                texture: textures.texture_bevy.clone(),
                transform: Transform::from_translation(Vec3::new(
                    game_area.physical_pos().x + 100. * i as f32,
                    300.,
                    1.,
                ))
                .with_scale(Vec3::new(0.2, 0.2, 1.)),
                ..default()
            },
            move_speed: MoveSpeed(150.),
            enemy: Enemy,
        });
    }
}

fn move_enemy(mut enemy_query: Query<(&mut Transform, &MoveSpeed), With<Enemy>>, time: Res<Time>) {
    for (mut enemy_transform, move_speed) in &mut enemy_query {
        enemy_transform.translation.y -= move_speed.0 * time.delta_seconds();
    }

    // TODO: Delete enemies when they go out of the screen view
}
