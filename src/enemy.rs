use crate::game_area::GameArea;
use crate::GameState;
use bevy::prelude::*;
use bevy_rapier2d::prelude::Collider;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Playing).with_system(update_vertical_enemy_movement),
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

// TODO: Just make `EnemyMovement` a component
#[derive(Component, Copy, Clone, Debug)]
pub struct VerticalEnemyMovement {
    speed: f32,
}

impl Default for VerticalEnemyMovement {
    fn default() -> Self {
        Self { speed: 150. }
    }
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

#[derive(Debug)]
pub enum EnemyMovement {
    Vertical(VerticalEnemyMovement),
}
