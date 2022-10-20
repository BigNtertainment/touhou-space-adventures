use crate::character::MoveSpeed;
use crate::GameState;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Playing).with_system(fly));
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

fn fly(mut bullet_query: Query<(&mut Transform, &MoveSpeed), With<Bullet>>, time: Res<Time>) {
    for (mut bullet_transform, move_speed) in &mut bullet_query {
        bullet_transform.translation.y += move_speed.0 * time.delta_seconds();
    }
}
