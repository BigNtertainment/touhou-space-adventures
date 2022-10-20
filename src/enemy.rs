use crate::game_area::GameArea;
use crate::GameState;
use bevy::prelude::*;
use bevy_rapier2d::prelude::Collider;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Playing).with_system(update_enemy_movement),
        );
    }
}

#[derive(Component)]
pub struct Enemy;

#[derive(Bundle)]
pub struct EnemyBundle {
    #[bundle]
    pub sprite_bundle: SpriteBundle,
    pub enemy: Enemy,
    pub collider: Collider,
}

fn update_enemy_movement(
    mut commands: Commands,
    mut enemies: Query<(Entity, &mut Transform, &EnemyMovement)>,
    time: Res<Time>,
    game_area: Res<GameArea>,
) {
    for (entity, mut transform, movement) in enemies.iter_mut() {
        match movement {
            EnemyMovement::Vertical { speed } => {
                transform.translation.y -= speed * time.delta_seconds();
            },
            EnemyMovement::Sin { speed, amplitude } => {
                transform.translation.y -= speed * time.delta_seconds();
                transform.translation.x += (transform.translation.y / 50.).sin() * amplitude / 15.;
            }
        }

        if transform.translation.y < -game_area.physical_pos().y - game_area.height / 2. {
            commands.entity(entity).despawn_recursive();
        }
    }
}

#[derive(Component, Copy, Clone, Debug)]
pub enum EnemyMovement {
    Vertical {
        speed: f32,
    },
    Sin {
        speed: f32,
        amplitude: f32,
    }
}

impl Default for EnemyMovement {
    fn default() -> Self {
        Self::Vertical { speed: 150. }
    }
}