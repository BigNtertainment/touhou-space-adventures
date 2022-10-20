use crate::GameState;
use crate::{character::MoveSpeed, enemy::Enemy};
use bevy::sprite::collide_aabb::collide;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(fly)
                .with_system(kill_enemy),
        );
    }
}

#[derive(Component)]
pub struct Bullet;

#[derive(Bundle)]
pub struct BulletBundle {
    #[bundle]
    pub mesh: MaterialMesh2dBundle<ColorMaterial>,
    pub bullet: Bullet,
    pub move_speed: MoveSpeed,
}

impl Default for BulletBundle {
    fn default() -> Self {
        Self {
            mesh: MaterialMesh2dBundle::default(),
            bullet: Bullet,
            move_speed: MoveSpeed(500.),
        }
    }
}

fn fly(mut bullet_query: Query<(&mut Transform, &MoveSpeed), With<Bullet>>, time: Res<Time>) {
    for (mut bullet_transform, move_speed) in &mut bullet_query {
        bullet_transform.translation.y += move_speed.0 * time.delta_seconds();
    }
}

fn kill_enemy(
    mut commands: Commands,
    bullet_query: Query<&Transform, With<Bullet>>,
    enemies_query: Query<(&Transform, Entity), With<Enemy>>,
) {
    for bullet_transform in &bullet_query {
        for (enemy_transform, enemy_entity) in &enemies_query {
            if collide(
                bullet_transform.translation,
                Vec2::splat(10.),
                enemy_transform.translation,
                Vec2::splat(50.),
            )
            .is_some()
            {
                commands.entity(enemy_entity).despawn_recursive();
            }
        }
    }
}
