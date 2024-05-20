use bevy::prelude::*;

use crate::schedules::InGameSet;

const DESPAWN_DISTANCE: f32 = 100.0;

pub struct DespwanerPlugin;
impl Plugin for DespwanerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            off_screen_despwaner.in_set(InGameSet::DespwanEntities),
        );
    }
}

// TODO this causes "Could not despawn entity" warnings
fn off_screen_despwaner(mut commands: Commands, query: Query<(Entity, &GlobalTransform)>) {
    for (entity, transform) in query.iter() {
        if transform.translation().distance(Vec3::ZERO) > DESPAWN_DISTANCE {
            commands.entity(entity).despawn_recursive();
        }
    }
}
