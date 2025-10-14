use aeronet_webtransport::{client::WebTransportClientPlugin, server::WebTransportServerPlugin};
use avian3d::prelude::*;
use bevy::{
    prelude::*,
    window::{CursorGrabMode, CursorOptions},
};
use clap::Parser;
use spacerobo::{
    GameMode,
    cli::CLIArgs,
    configs::GameConfigs,
    scenes::{
        shooting_range::ShootingRangePlugin, title::TitlePlugin, versus_guest::VersusGuestPlugin,
        versus_master::VersusMasterPlugin,
    },
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: CLIArgs = CLIArgs::parse();

    let configs: GameConfigs = confy::load_path(&args.config_file()).unwrap_or_else(|_| {
        info!("Running Spacerobo with default GameConfigs...");
        GameConfigs::default()
    });

    debug!("Your GameConfigs: {:?}", configs);

    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    cursor_options: CursorOptions {
                        visible: false,
                        grab_mode: CursorGrabMode::Locked,
                        ..default()
                    },
                    title: format!("spacerobo {}", env!("CARGO_PKG_VERSION")),
                    ..default()
                }),
                ..default()
            }),
            PhysicsPlugins::default(),
            WebTransportClientPlugin,
            WebTransportServerPlugin,
            TitlePlugin,
            ShootingRangePlugin,
            VersusMasterPlugin,
            VersusGuestPlugin,
        ))
        .init_state::<GameMode>()
        .insert_resource(configs)
        .run();

    Ok(())
}
