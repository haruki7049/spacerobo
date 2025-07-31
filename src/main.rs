use avian3d::prelude::*;
use bevy::{
    prelude::*,
    window::{CursorGrabMode, CursorOptions},
};
use clap::Parser;
use spacerobo::{DeathEvent, GameMode, cli::CLIArgs, configs::GameConfigs, scenes};

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
                scenes::shooting_range::player::setup_system,
                scenes::shooting_range::player::ui::setup_system,
            ),
        )
        .add_systems(
            Update,
            (
                // Player
                scenes::shooting_range::player::respawn_system,
                scenes::shooting_range::player::ui::update_system,
                scenes::shooting_range::player::gun::select_fire::full_auto_system,
                scenes::shooting_range::player::gun::select_fire::semi_auto_system,
                scenes::shooting_range::player::gun::select_fire::toggle_select_fire_system,
                // Systems
                scenes::shooting_range::player::gun::bullet::health::update_system,
                scenes::shooting_range::player::health::update_system,
                scenes::shooting_range::health::update_system,
                scenes::shooting_range::collision_detection_system,
            )
                .run_if(in_state(GameMode::ShootingRange)),
        )
        .add_systems(
            FixedUpdate,
            (
                // Player movement systems
                scenes::shooting_range::player::movement::keyboard::update_system,
                scenes::shooting_range::player::movement::mouse::update_system,
                scenes::shooting_range::player::movement::controller::update_system,
                // Player gun systems
                scenes::shooting_range::player::gun::gun_cooling_system,
            )
                .run_if(in_state(GameMode::ShootingRange)),
        )
        // Versus
        .add_systems(
            OnEnter(GameMode::Versus),
            (
                scenes::versus::setup_system,
                scenes::versus::player::setup_system,
                scenes::versus::player::ui::setup_system,
            ),
        )
        .add_systems(
            Update,
            (
                // Player
                scenes::versus::player::ui::update_system,
                scenes::versus::player::gun::select_fire::full_auto_system,
                scenes::versus::player::gun::select_fire::semi_auto_system,
                scenes::versus::player::gun::select_fire::toggle_select_fire_system,
                // Systems
                scenes::versus::player::gun::bullet::health::update_system,
                scenes::versus::player::health::update_system,
                scenes::versus::health::update_system,
                scenes::versus::collision_detection_system,
            )
                .run_if(in_state(GameMode::Versus)),
        )
        .add_systems(
            FixedUpdate,
            (
                // Player movement systems
                scenes::versus::player::movement::keyboard::update_system,
                scenes::versus::player::movement::mouse::update_system,
                scenes::versus::player::movement::controller::update_system,
                // Player gun systems
                scenes::versus::player::gun::gun_cooling_system,
            )
                .run_if(in_state(GameMode::Versus)),
        )
        .run();

    Ok(())
}
