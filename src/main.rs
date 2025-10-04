use avian3d::prelude::*;
use bevy::{
    prelude::*,
    window::{CursorGrabMode, CursorOptions},
};
use clap::Parser;
use spacerobo::{
    DeathEvent, GameMode, KillCounter, cli::CLIArgs, configs::GameConfigs, entities, scenes,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: CLIArgs = CLIArgs::parse();

    let configs: GameConfigs = confy::load_path(&args.config_file).unwrap_or_else(|_| {
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
        ))
        .init_state::<GameMode>()
        .add_event::<DeathEvent>()
        .insert_resource(configs)
        .insert_resource(Gravity(Vec3::NEG_Y * 0.))
        .insert_resource(Time::<Virtual>::default())
        .insert_resource(KillCounter::default())
        // Title
        .add_systems(OnEnter(GameMode::Title), scenes::title::setup_system)
        .add_systems(
            Update,
            (scenes::title::input_detection_system).run_if(in_state(GameMode::Title)),
        )
        // Shooting range
        .add_systems(
            OnEnter(GameMode::ShootingRange),
            (
                scenes::shooting_range::setup_system,
                entities::player::setup_system,
                entities::player::ui::setup_system,
            ),
        )
        .add_systems(
            Update,
            (
                // Player
                entities::player::respawn_system,
                entities::player::ui::update_system,
                entities::player::gun::select_fire::full_auto_system,
                entities::player::gun::select_fire::semi_auto_system,
                entities::player::gun::select_fire::toggle_select_fire_system,
                entities::player::gun::bullet::health::update_system,
                entities::player::health::update_system,
                // Bot
                scenes::shooting_range::bot::gun::select_fire::full_auto_system,
                // Systems
                scenes::shooting_range::health::update_system,
                scenes::shooting_range::collision_detection_system,
                scenes::shooting_range::when_going_outside_system,
            )
                .run_if(in_state(GameMode::ShootingRange)),
        )
        .add_systems(
            FixedUpdate,
            (
                // Player movement systems
                entities::player::movement::keyboard::update_system,
                entities::player::movement::mouse::update_system,
                entities::player::movement::controller::update_system,
                // Player gun systems
                entities::player::gun::gun_cooling_system,
                // Bot gun systems
                scenes::shooting_range::bot::gun::gun_cooling_system,
            )
                .run_if(in_state(GameMode::ShootingRange)),
        )
        .run();

    Ok(())
}
