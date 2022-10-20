use crate::game_area::GameArea;
use crate::GameState;
use bevy::prelude::*;

use crate::loading::TextureAssets;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_enemies))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(update_vertical_enemy_movement));
    }
}

#[derive(Component)]
pub struct Enemy;

#[derive(Bundle)]
pub struct EnemyBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
    enemy: Enemy,
}

fn spawn_enemies(mut commands: Commands, textures: Res<TextureAssets>, game_area: Res<GameArea>) {
    // TODO: Think how to implement logic that spawns waves or somtehing
    for i in -1..2 {
        commands
            .spawn_bundle(EnemyBundle {
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
                enemy: Enemy,
            })
            .insert(VerticalEnemyMovement { speed: 150. });
    }
}

#[derive(Component)]
pub struct VerticalEnemyMovement {
    speed: f32,
}

impl VerticalEnemyMovement {
    fn update(&self, transform: &mut Transform, delta: f32) {
        transform.translation.y -= delta * self.speed;
    }
}

fn update_vertical_enemy_movement(
    mut commands: Commands,
    mut enemies: Query<(Entity, &mut Transform, &VerticalEnemyMovement)>,
    time: Res<Time>,
    game_area: Res<GameArea>,
) {
    for (entity, mut transform, movement) in enemies.iter_mut() {
        movement.update(transform.as_mut(), time.delta_seconds());

        if transform.translation.y < -game_area.physical_pos().y - game_area.height / 2. {
            commands.entity(entity).despawn_recursive();
        }
    }
}
