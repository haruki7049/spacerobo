use avian3d::prelude::*;
use bevy::{
    prelude::*,
    window::{CursorGrabMode, CursorOptions},
};
use bevy_embedded_assets::{EmbeddedAssetPlugin, PluginMode};
use clap::Parser;
use spacerobo_client::cli::CLIArgs;
use spacerobo_commons::{ControllablePlugin, GameMode, configs::GameConfigs};
use spacerobo_shooting_range_plugin::ShootingRangePlugin;
use spacerobo_title_plugin::TitlePlugin;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: CLIArgs = CLIArgs::parse();

    let configs: GameConfigs = confy::load_path(args.config_file()).unwrap_or_else(|e| {
        // The logger is not installed before `App::new()`, so `warn!` would be lost here.
        eprintln!(
            "warning: failed to load {}: {e}. Running Spacerobo with default GameConfigs...",
            args.config_file().display()
        );
        GameConfigs::default()
    });

    debug!("Your GameConfigs: {:?}", configs);

    App::new()
        .add_plugins((
            // `ReplaceDefault` serves the embedded assets through the default asset source, so
            // `asset_server.load("SE/shoot.ogg")` works without an `assets/` directory.
            // It must be registered before `DefaultPlugins`, which builds the `AssetPlugin`.
            EmbeddedAssetPlugin {
                mode: PluginMode::ReplaceDefault,
            },
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: format!("spacerobo {}", env!("CARGO_PKG_VERSION")),
                    ..default()
                }),
                primary_cursor_options: Some(CursorOptions {
                    visible: false,
                    grab_mode: CursorGrabMode::Locked,
                    ..default()
                }),
                ..default()
            }),
            PhysicsPlugins::default(),
            TitlePlugin,
            ShootingRangePlugin,
            ControllablePlugin,
        ))
        .init_state::<GameMode>()
        .insert_resource(configs)
        .run();

    Ok(())
}
