use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct GameAssets {
    pub projectile: Handle<Scene>,
    pub tank_body: Handle<Scene>,
    pub tank_turret: Handle<Scene>,
}

pub fn load_game_assets(mut commands: Commands, assets_server: Res<AssetServer>) {
    let game_assets = GameAssets {
        projectile: assets_server.load("FancyTank/projectile.gltf#Scene0"),
        tank_body: assets_server.load("FancyTank/body.gltf#Scene0"),
        tank_turret: assets_server.load("FancyTank/turret.gltf#Scene0"),
    };

    commands.insert_resource(game_assets);
}
