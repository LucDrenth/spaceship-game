use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct SceneAssets {
    pub asteroid: Handle<Scene>,
    pub spaceship: Handle<Scene>,
    pub missiles: Handle<Scene>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneAssets>()
            .add_systems(PreStartup, load_assets);
    }
}

fn load_assets(mut scene_assets: ResMut<SceneAssets>, asset_server: Res<AssetServer>) {
    let scene_suffix = "#Scene0".to_owned();

    *scene_assets = SceneAssets {
        asteroid: asset_server.load(format!("Asteroid.glb{scene_suffix}")),
        spaceship: asset_server.load(format!("Spaceship.glb{scene_suffix}")),
        missiles: asset_server.load(format!("Missiles.glb{scene_suffix}")),
    }
}
