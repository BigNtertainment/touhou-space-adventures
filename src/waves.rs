use crate::util::despawn;
use bevy::prelude::*;
use bevy_rapier2d::prelude::Collider;

use crate::{
    enemy::{Enemy, EnemyBundle, EnemyMovement},
    game_area::GameArea,
    loading::TextureAssets,
    GameState,
};

pub struct WavesPlugin;

impl Plugin for WavesPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_waves_manager))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(spawn_waves));
    }
}

#[derive(Debug)]
struct EnemyData {
    spawn_position: Vec2,
    movement: EnemyMovement,
}

struct Wave {
    enemies: Vec<EnemyData>,
    timeout: f32,
}

#[derive(Component)]
struct WavesManager {
    pub timer: Timer,
    pub waves: Vec<Wave>,
    pub current: usize,
}

fn spawn_waves_manager(mut commands: Commands) {
    commands.spawn().insert(WavesManager {
        timer: Timer::from_seconds(3., false),
        waves: vec![
            Wave {
                enemies: vec![
                    EnemyData {
                        spawn_position: Vec2::new(-0.5, 1.),
                        movement: EnemyMovement::default(),
                    },
                    EnemyData {
                        spawn_position: Vec2::new(0., 1.),
                        movement: EnemyMovement::default(),
                    },
                    EnemyData {
                        spawn_position: Vec2::new(0.5, 1.),
                        movement: EnemyMovement::default(),
                    },
                ],
                timeout: 7.,
            },
            Wave {
                enemies: vec![
                    EnemyData {
                        spawn_position: Vec2::new(-0.5, 1.),
                        movement: EnemyMovement::Sin {
                            speed: 150.,
                            amplitude: 35.,
                        },
                    },
                    EnemyData {
                        spawn_position: Vec2::new(0., 1.),
                        movement: EnemyMovement::Sin {
                            speed: 150.,
                            amplitude: 35.,
                        },
                    },
                    EnemyData {
                        spawn_position: Vec2::new(0.5, 1.),
                        movement: EnemyMovement::Sin {
                            speed: 150.,
                            amplitude: 35.,
                        },
                    },
                    EnemyData {
                        spawn_position: Vec2::new(1., 1.),
                        movement: EnemyMovement::Sin {
                            speed: 150.,
                            amplitude: 35.,
                        },
                    },
                ],
                timeout: 7.,
            },
        ],
        current: 0,
    });
}

fn spawn_waves(
    mut commands: Commands,
    mut waves_manager: Query<&mut WavesManager>,
    time: Res<Time>,
    textures: Res<TextureAssets>,
    game_area: Res<GameArea>,
    images: Res<Assets<Image>>,
) {
    let mut waves_manager = waves_manager.single_mut();

    if waves_manager.timer.tick(time.delta()).finished() {
        waves_manager.current += 1;
        waves_manager.current %= waves_manager.waves.len();

        let current_wave = &waves_manager.waves[waves_manager.current];

        let texture_size = images
            .get(&textures.enemy_texture)
            .unwrap()
            .texture_descriptor
            .size;

        for enemy_data in &current_wave.enemies {
            commands
                .spawn_bundle(EnemyBundle {
                    sprite_bundle: SpriteBundle {
                        texture: textures.enemy_texture.clone(),
                        transform: Transform::from_translation(
                            game_area
                                .relative_to_absolute(enemy_data.spawn_position)
                                .extend(99.),
                        )
                        .with_scale(Vec3::new(0.75, 0.75, 1.)),
                        ..default()
                    },
                    enemy: Enemy,
                    collider: Collider::cuboid(
                        texture_size.width as f32,
                        texture_size.height as f32,
                    ),
                })
                .insert(enemy_data.movement.clone());
        }

        waves_manager.timer = Timer::from_seconds(current_wave.timeout, false);
    }
}
