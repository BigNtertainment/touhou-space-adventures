use crate::game_area::GameArea;
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

fn fly(
    mut bullet_query: Query<(&mut Transform, &MoveSpeed, Entity), With<Bullet>>,
    mut commands: Commands,
    time: Res<Time>,
    game_area: Res<GameArea>,
) {
    for (mut bullet_transform, move_speed, bullet_entity) in &mut bullet_query {
        bullet_transform.translation.y += move_speed.0 * time.delta_seconds();

        if bullet_transform.translation.y > game_area.physical_pos().y + game_area.height / 2. {
            commands.entity(bullet_entity).despawn_recursive()
        }
    }
}

fn kill_enemy(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform), With<Bullet>>,
    enemies_query: Query<(&Transform, Entity), With<Enemy>>,
) {
    for (bullet_entity, bullet_transform) in &bullet_query {
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
                commands.entity(bullet_entity).despawn_recursive();
            }
        }
    }
}
