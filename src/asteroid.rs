use std::ops::Range;

use bevy::prelude::*;
use rand::Rng;

use crate::{
    asset_loader::SceneAssets,
    collision_detection::Collider,
    game_state::GameState,
    movement::{Acceleration, MovingObjectBundle, Velocity},
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
        .add_systems(Update, remove_on_missle_collision)
        .add_systems(Update, spawn_asteroid);
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
    game_state: Res<GameState>,
) {
    if !game_state.is_playing {
        return;
    }

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

fn remove_on_missle_collision(
    mut commands: Commands,
    query: Query<(Entity, &Collider), With<Asteroid>>,
    missles_query: Query<Entity, With<SpaceshipMissile>>,
    game_state: Res<GameState>,
) {
    if !game_state.is_playing {
        return;
    }

    let mut missle_entities = vec![];
    for missle_entity in missles_query.iter() {
        missle_entities.push(missle_entity);
    }

    for (asteroid_entity, asteroid_collider) in query.iter() {
        for collided_entity in &asteroid_collider.colliding_entities {
            match missle_entities.iter().find(|e| *e == collided_entity) {
                Some(colliding_missle_entity) => {
                    commands.entity(asteroid_entity).despawn_recursive();

                    // TODO also remove entry from missle_entities, so that if a missle collides with
                    // multiple asteroid, we won't get a warning about not being able to despawn the
                    // missle entity due to it already being despawnd by a previous asteroid collision
                    // check.
                    commands
                        .entity(*colliding_missle_entity)
                        .despawn_recursive();
                    break;
                }
                None => (),
            }
        }
    }
}
