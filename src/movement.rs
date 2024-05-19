use bevy::prelude::*;

use crate::{collision_detection::Collider, game_state::GameState};

#[derive(Component, Debug)]
pub struct Velocity {
    pub value: Vec3,
}

#[derive(Component, Debug)]
pub struct Acceleration {
    pub value: Vec3,
}

#[derive(Bundle)]
pub struct MovingObjectBundle {
    pub velocity: Velocity,
    pub acceleration: Acceleration,
    pub model: SceneBundle,
    pub collider: Collider,
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_velocity, update_position));
    }
}

fn update_velocity(
    mut query: Query<(&Acceleration, &mut Velocity)>,
    time: Res<Time>,
    game_state: Res<GameState>,
) {
    if !game_state.is_playing {
        return;
    }

    for (acceleration, mut velocity) in query.iter_mut() {
        velocity.value += acceleration.value * time.delta_seconds();
    }
}

fn update_position(
    mut query: Query<(&mut Transform, &Velocity)>,
    time: Res<Time>,
    game_state: Res<GameState>,
) {
    if !game_state.is_playing {
        return;
    }

    for (mut transform, velocity) in query.iter_mut() {
        transform.translation += velocity.value * time.delta_seconds();
    }
}
