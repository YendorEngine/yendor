use bevy::prelude::*;
use serde::Deserialize;
use yendor_bevy_asset::prelude::*;
use yendor_lib::prelude as yendor;

/// Example of an airplane asset.
#[derive(YendorBevyAsset, TypeUlid, Deserialize, Debug, HasLoadProgress)]
#[ulid = "01GNT26ATV1QWAAYP2PA3M5EFT"]
// This allows us to load the asset from files with `.meta.json` and `.meta.yaml` extensions.
#[asset_id = "meta"]
pub struct GameMeta {
    pub title: String,
    #[asset(deserialize_only)]
    pub info: GameInfo,
    pub players: Vec<yendor::Handle<PlayerMeta>>,
}

#[derive(serde::Deserialize, Debug, HasLoadProgress)]
pub struct GameInfo {
    pub description: String,
    pub authors: Vec<String>,
}

#[derive(YendorBevyAsset, TypeUlid, Deserialize, Debug)]
#[ulid = "01GNT6APZEBGYFJ8SAKX2Q7TX2"]
#[asset_id = "player"]
pub struct PlayerMeta {
    pub name: String,
}

#[derive(Resource)]
pub struct GameMetaHandle(pub Handle<GameMeta>);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_yendor_asset::<GameMeta>()
        .add_yendor_asset::<PlayerMeta>()
        .add_startup_system(|mut commands: Commands, asset_server: Res<AssetServer>| {
            let handle = asset_server.load("game.meta.yaml");
            commands.insert_resource(GameMetaHandle(handle));
        })
        .add_system(
            |mut done: Local<bool>,
             player_assets: Res<Assets<PlayerMeta>>,
             game_meta_assets: Res<Assets<GameMeta>>,
             game_meta_handle: Option<Res<GameMetaHandle>>,
             loading_resources: LoadingResources| {
                if *done {
                    return;
                }

                let Some(game_meta_handle) = game_meta_handle else {
                    return;
                };

                let game_meta = if let Some(game_meta) = game_meta_assets.get(&game_meta_handle.0) {
                    game_meta
                } else {
                    return;
                };

                let game_progress = game_meta.load_progress(&loading_resources);
                let total_percent = game_progress.as_percent() / 3.0 * 100.0;

                // Wait until assets are loaded to start game
                if total_percent >= 1.0 {
                    *done = true;
                }

                let player_bevy_handles =
                    game_meta.players.iter().map(|x| x.get_bevy_handle()).collect::<Vec<_>>();

                if player_bevy_handles.iter().all(|x| player_assets.get(x).is_some()) {
                    *done = true;
                    dbg!(&game_meta);

                    for player_handle in &game_meta.players {
                        let handle = player_handle.get_bevy_handle();

                        let player_meta = player_assets.get(&handle);

                        dbg!(&player_meta);
                    }
                }
            },
        )
        .run();
}
