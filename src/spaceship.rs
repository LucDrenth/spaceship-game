use std::f32::consts::{PI, TAU};

use bevy::prelude::*;

use crate::{
    asset_loader::SceneAssets,
    collision_detection::Collider,
    game_state::GameState,
    movement::{Acceleration, MovingObjectBundle, Velocity},
};

const SPACESHIP_ROTATION_SPEED: f32 = 2.0;
const SPACESHIP_ROLL_AMOUNT_ON_STEER: f32 = TAU / 16.0;
const SPACESHIP_MOVEMENT_SPEED: f32 = 20.0;
const SPACESHIP_COLLISION_SIZE: f32 = 5.0;

const MISSILE_SPEED: f32 = 50.0;
const MISSILE_COLLISION_SIZE: f32 = 1.0;

#[derive(Component)]
pub struct Spaceship;

#[derive(Component)]
pub struct SpaceshipMissile;

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_spaceship)
            .add_systems(Update, spaceship_movement_controls)
            .add_systems(Update, spaceship_weapon_controls);
    }
}

fn spawn_spaceship(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity { value: Vec3::ZERO },
            acceleration: Acceleration { value: Vec3::ZERO },
            model: SceneBundle {
                scene: scene_assets.spaceship.clone(),
                transform: Transform::from_translation(Vec3::ZERO),
                ..default()
            },
            collider: Collider::new(SPACESHIP_COLLISION_SIZE),
        },
        Spaceship,
    ));
}

fn spaceship_movement_controls(
    mut query: Query<(&mut Transform, &mut Velocity), With<Spaceship>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    game_state: Res<GameState>,
) {
    if !game_state.is_playing {
        return;
    }

    let (mut transform, mut velocity) = query.single_mut();

    let mut rotation_y = 0.0;
    let mut roll = 0.0;
    let mut movement = 0.0;

    if keyboard_input.pressed(KeyCode::KeyA) {
        rotation_y = SPACESHIP_ROTATION_SPEED * time.delta_seconds();
        roll = -SPACESHIP_ROLL_AMOUNT_ON_STEER;
    } else if keyboard_input.pressed(KeyCode::KeyD) {
        rotation_y = -SPACESHIP_ROTATION_SPEED * time.delta_seconds();
        roll = SPACESHIP_ROLL_AMOUNT_ON_STEER;
    }

    if keyboard_input.pressed(KeyCode::KeyW) {
        movement = SPACESHIP_MOVEMENT_SPEED;
    } else if keyboard_input.pressed(KeyCode::KeyS) {
        movement = -SPACESHIP_MOVEMENT_SPEED;
    }

    transform.rotate_y(rotation_y);

    let current_role = transform.rotation.to_euler(EulerRot::XYZ).2;
    transform.rotate_local_z(roll - current_role);

    // TODO For some reason the spaceship is on its back when looking down so we compensate
    // by turning it back up. But there is still a glitch happening when looking exactly left or right.
    if transform.forward().z >= 0.0 {
        transform.rotate_local_z(PI);
    }

    // TODO ensure we do not go off screen, by either moving the camera or constraining movement to the windows
    velocity.value = -transform.forward() * movement;
}

fn spaceship_weapon_controls(
    mut commands: Commands,
    query: Query<&Transform, With<Spaceship>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    scene_assets: Res<SceneAssets>,
    game_state: Res<GameState>,
) {
    if !game_state.is_playing {
        return;
    }

    let spaceship_transform = query.single();

    // TODO spawn missiles independent of framerate
    if keyboard_input.pressed(KeyCode::Space) {
        let mut missile_transform = Transform::from_translation(
            spaceship_transform.translation + -spaceship_transform.forward() * 7.5,
        );
        missile_transform.rotation = spaceship_transform.rotation.clone();
        missile_transform.rotate_local_x(PI / 2.0);

        commands.spawn((
            MovingObjectBundle {
                velocity: Velocity {
                    value: -spaceship_transform.forward() * MISSILE_SPEED,
                },
                acceleration: Acceleration { value: Vec3::ZERO },
                model: SceneBundle {
                    scene: scene_assets.missiles.clone(),
                    transform: missile_transform,
                    ..default()
                },
                collider: Collider::new(MISSILE_COLLISION_SIZE),
            },
            SpaceshipMissile,
        ));
    }
}
