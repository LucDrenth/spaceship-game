use asset_loader::{AssetLoaderPlugin, SceneAssets};
use asteroid::AsteroidPlugin;
use bevy::prelude::*;
use camera::CameraPlugin;
use game_state::{GameState, GameStatePlugin};
use movement::MovementPlugin;
use spaceship::SpaceshipPlugin;

mod asset_loader;
mod game_state;

mod utils;

mod asteroid;
mod camera;
mod movement;
mod spaceship;

fn main() {
    App::new()
        // resources
        .init_resource::<GameState>()
        .init_resource::<SceneAssets>()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 1000.0,
        })
        // default plugins
        .add_plugins(DefaultPlugins)
        // custom plugins
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(GameStatePlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(SpaceshipPlugin)
        .add_plugins(AsteroidPlugin)
        // run app
        .run();
}
