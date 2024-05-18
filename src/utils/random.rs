use rand::Rng;
use std::f32::consts::TAU;

use bevy::prelude::*;

pub fn rotation() -> Quat {
    let mut rng = rand::thread_rng();

    let mut quat = Quat::from_rotation_x(rng.gen_range(0.0..TAU));
    quat *= Quat::from_rotation_y(rng.gen_range(0.0..TAU));
    quat *= Quat::from_rotation_z(rng.gen_range(0.0..TAU));

    quat
}

pub fn direction_2d() -> Vec3 {
    let mut rng = rand::thread_rng();
    Vec3::new(rng.gen_range(-1.0..1.0), 0.0, rng.gen_range(-1.0..1.0))
}
