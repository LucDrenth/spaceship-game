use asset_loader::{AssetLoaderPlugin, SceneAssets};
use asteroid::AsteroidPlugin;
use bevy::prelude::*;
use camera::CameraPlugin;
use collision_detection::CollisionDetectionPlugin;
use despawner::DespwanerPlugin;
use game_state::GameStatePlugin;
use movement::MovementPlugin;
use schedules::SchedulesPlugin;
use spaceship::SpaceshipPlugin;

mod asset_loader;
mod asteroid;
mod camera;
mod collision_detection;
mod despawner;
mod game_state;
mod movement;
mod schedules;
mod spaceship;
mod utils;

fn main() {
    App::new()
        // resources
        .init_resource::<SceneAssets>()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 1000.0,
        })
        // default plugins
        .add_plugins(DefaultPlugins)
        // custom plugins
        .add_plugins(SchedulesPlugin)
        .add_plugins(GameStatePlugin)
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(CollisionDetectionPlugin)
        .add_plugins(DespwanerPlugin)
        .add_plugins(SpaceshipPlugin)
        .add_plugins(AsteroidPlugin)
        // run app
        .run();
}
