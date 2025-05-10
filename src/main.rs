use bevy::prelude::*;
use clap::Parser;

fn main() {
    let _args: CLIArgs = CLIArgs::parse();

    App::new().add_plugins(DefaultPlugins).run();
}

#[derive(Debug, Parser)]
#[command(version, about)]
struct CLIArgs;
