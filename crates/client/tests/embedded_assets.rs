//! `EmbeddedAssetPlugin` in `ReplaceDefault` mode must serve the game assets through the default
//! asset source, so that `AssetServer::load("SE/shoot.ogg")` needs no `assets/` directory.

use bevy::{
    asset::io::{AssetSourceId, Reader},
    prelude::*,
    tasks::block_on,
};
use bevy_embedded_assets::{EmbeddedAssetPlugin, PluginMode};
use std::path::Path;

fn read_default_source(app: &App, path: &str) -> Vec<u8> {
    let server = app.world().resource::<AssetServer>();
    let source = server
        .get_source(AssetSourceId::Default)
        .expect("the default asset source should exist");

    block_on(async {
        let mut reader = source
            .reader()
            .read(Path::new(path))
            .await
            .unwrap_or_else(|e| panic!("{path} should be embedded: {e}"));
        let mut bytes = Vec::new();
        reader
            .read_to_end(&mut bytes)
            .await
            .unwrap_or_else(|e| panic!("failed to read {path}: {e}"));
        bytes
    })
}

#[test]
fn game_assets_are_served_from_the_binary() {
    let mut app = App::new();
    app.add_plugins((
        EmbeddedAssetPlugin {
            mode: PluginMode::ReplaceDefault,
        },
        MinimalPlugins,
        AssetPlugin::default(),
    ));

    for path in ["SE/shoot.ogg", "SE/kill.ogg", "branding/icon.png"] {
        assert!(
            !read_default_source(&app, path).is_empty(),
            "{path} should not be empty"
        );
    }
}
