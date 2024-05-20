use std::ops::Range;

use bevy::prelude::*;
use rand::Rng;

use crate::{
    asset_loader::SceneAssets,
    collision_detection::Collider,
    movement::{Acceleration, MovingObjectBundle, Velocity},
    schedules::InGameSet,
    spaceship::SpaceshipMissile,
    utils::random,
};

const SPAWN_INTERVAL_SECONDS: f32 = 0.6;

const SPAWN_RANGE_X: Range<f32> = -25.0..25.0;
const SPAWN_RANGE_Z: Range<f32> = 0.0..25.0;
const VELOCITY_SCALAR: f32 = 5.0;
const ACCELERATION_SCALAR: f32 = 1.0;
const COLLISION_SIZE: f32 = 2.0;

#[derive(Component)]
pub struct Asteroid;

pub struct AsteroidPlugin;
impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer {
            timer: Timer::from_seconds(SPAWN_INTERVAL_SECONDS, TimerMode::Repeating),
        })
        .add_systems(
            Update,
            remove_on_missile_collision.in_set(InGameSet::CollisionDetection),
        )
        .add_systems(Update, spawn_asteroid.in_set(InGameSet::EntitySpwaning));
    }
}

#[derive(Resource)]
struct SpawnTimer {
    timer: Timer,
}

fn spawn_asteroid(
    mut commands: Commands,
    mut spawn_timer: ResMut<SpawnTimer>,
    time: Res<Time>,
    scene_assets: Res<SceneAssets>,
) {
    spawn_timer.timer.tick(time.delta());

    if !spawn_timer.timer.just_finished() {
        return;
    }

    let mut rng = rand::thread_rng();

    let translation = Vec3::new(
        rng.gen_range(SPAWN_RANGE_X),
        0.0,
        rng.gen_range(SPAWN_RANGE_Z),
    );
    let mut transform = Transform::from_translation(translation);
    transform.rotation = random::rotation();
    let scale = rng.gen_range(0.5..1.5);
    transform.scale *= scale;

    let velocity = random::direction_2d() * VELOCITY_SCALAR;
    let acceleration = random::direction_2d() * ACCELERATION_SCALAR;

    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity { value: velocity },
            acceleration: Acceleration {
                value: acceleration,
            },
            model: SceneBundle {
                scene: scene_assets.asteroid.clone(),
                transform,
                ..default()
            },
            collider: Collider::new(COLLISION_SIZE * scale),
        },
        Asteroid,
    ));
}

fn remove_on_missile_collision(
    mut commands: Commands,
    query: Query<(Entity, &Collider), With<Asteroid>>,
    missiles_query: Query<Entity, With<SpaceshipMissile>>,
) {
    struct MissileEntry {
        entity: Entity,
        removed: bool,
    }

    let mut missiles: Vec<MissileEntry> = vec![];
    for missile_entity in missiles_query.iter() {
        missiles.push(MissileEntry {
            entity: missile_entity,
            removed: false,
        });
    }

    for (asteroid_entity, asteroid_collider) in query.iter() {
        for collided_entity in &asteroid_collider.colliding_entities {
            let mut colliding_missile_entry_index: Option<usize> = None;

            for i in 0..missiles.len() {
                if missiles[i].removed {
                    continue;
                }

                if missiles[i].entity == *collided_entity {
                    colliding_missile_entry_index = Some(i);
                    break;
                }
            }

            match colliding_missile_entry_index {
                Some(missile_entry_index) => {
                    commands.entity(asteroid_entity).despawn_recursive();

                    if !missiles[missile_entry_index].removed {
                        missiles[missile_entry_index].removed = true;
                        commands
                            .entity(missiles[missile_entry_index].entity)
                            .despawn_recursive();
                    }

                    break;
                }
                None => (),
            }
        }
    }
}
