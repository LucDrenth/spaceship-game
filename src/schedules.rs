pub use bevy::prelude::*;

use crate::game_state::GameState;

#[derive(SystemSet, Hash, PartialEq, Eq, Clone, Debug)]
pub enum InGameSet {
    UserInput,
    EntitySpwaning,
    EntityUpdates,
    CollisionDetection,
    DespwanEntities,
}

pub struct SchedulesPlugin;

impl Plugin for SchedulesPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                InGameSet::DespwanEntities,
                InGameSet::UserInput,
                InGameSet::EntitySpwaning,
                InGameSet::EntityUpdates,
                InGameSet::CollisionDetection,
            )
                // TODO add `.chain()`. It will result in a bug regarding collisions though.
                .run_if(in_state(GameState::InGame)),
        );
    }
}
