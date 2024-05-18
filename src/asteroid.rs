use std::ops::Range;

use bevy::prelude::*;
use rand::Rng;

use crate::{
    asset_loader::SceneAssets,
    game_state::GameState,
    movement::{Acceleration, MovingObjectBundle, Velocity},
    utils::random,
};

const SPAWN_INTERVAL_SECONDS: f32 = 0.6;

const SPAWN_RANGE_X: Range<f32> = -25.0..25.0;
const SPAWN_RANGE_Z: Range<f32> = 0.0..25.0;
const VELOCITY_SCALAR: f32 = 5.0;
const ACCELERATION_SCALAR: f32 = 1.0;

#[derive(Component)]
pub struct Asteroid;

pub struct AsteroidPlugin;
impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer {
            timer: Timer::from_seconds(SPAWN_INTERVAL_SECONDS, TimerMode::Repeating),
        })
        .add_systems(Update, spawn_asteroid);
    }
}

#[derive(Resource)]
pub struct SpawnTimer {
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

    let translation = Vec3 {
        x: rng.gen_range(SPAWN_RANGE_X),
        z: 0.0,
        y: rng.gen_range(SPAWN_RANGE_Z),
    };
    let mut transform = Transform::from_translation(translation);
    transform.rotation = random::rotation();
    transform.scale *= rng.gen_range(0.5..1.5);

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
                transform: transform,
                ..default()
            },
        },
        Asteroid,
    ));
}
