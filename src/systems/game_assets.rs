use std::collections::HashMap;

use bevy::{asset::LoadState, prelude::*};

use crate::AppState;

#[derive(Resource, Default)]
pub struct GameAssets {
    assets: HashMap<GameAssetName, Handle<Scene>>,
}

impl GameAssets {
    fn all_assets_loaded(&self, asset_server: Res<AssetServer>) -> bool {
        for asset in self.assets.values() {
            match asset_server.get_load_state(asset) {
                Some(LoadState::NotLoaded) => todo!(),
                Some(LoadState::Loading) => return false,
                Some(LoadState::Loaded) => continue,
                Some(LoadState::Failed(error)) => {
                    panic!(
                        "An asset {} failed to load: {}",
                        asset.path().unwrap(),
                        error
                    )
                }
                None => todo!(),
            }
        }

        true
    }

    pub fn get_asset(&self, asset_name: GameAssetName) -> Handle<Scene> {
        match self.assets.get(&asset_name) {
            Some(asset) => asset.clone(),
            None => panic!("Asset had not been loaded"),
        }
    }
}

#[derive(PartialEq, Eq, Hash)]
pub enum GameAssetName {
    Projectile,
    TankBody,
    TankTurret,
}

pub fn loading_assets(
    game_assets: Res<GameAssets>,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if game_assets.all_assets_loaded(asset_server) {
        println!("Done Loading Assets...",);
        next_state.set(AppState::Game);
    } else {
        println!("Loading Assets... ");
    }
}

pub fn load_game_assets(mut commands: Commands, assets_server: Res<AssetServer>) {
    let game_assets = GameAssets {
        assets: HashMap::from([
            (
                GameAssetName::Projectile,
                assets_server.load("FancyTank/projectile.gltf#Scene0"),
            ),
            (
                GameAssetName::TankBody,
                assets_server.load("FancyTank/body.gltf#Scene0"),
            ),
            (
                GameAssetName::TankTurret,
                assets_server.load("FancyTank/turret.gltf#Scene0"),
            ),
        ]),
    };

    commands.insert_resource(game_assets);
}
