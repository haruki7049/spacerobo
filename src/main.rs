use avian3d::prelude::*;
use bevy::{
    prelude::*,
    window::{CursorGrabMode, CursorOptions},
};
use clap::Parser;
use spacerobo::{CLIArgs, DeathEvent, GameMode, player, scenes, system, target};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: CLIArgs = CLIArgs::parse();

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
        .insert_resource(args)
        .insert_resource(Gravity(Vec3::NEG_Y * 0.))
        .insert_resource(Time::<Virtual>::default())
        .add_systems(OnEnter(GameMode::Title), scenes::title::setup_system)
        .add_systems(
            Update,
            (scenes::title::input_detection_system).run_if(in_state(GameMode::Title)),
        )
        .add_systems(
            OnEnter(GameMode::ShootingRange),
            (
                scenes::shooting_range::setup_system,
                player::setup_system,
                player::ui::setup_system,
            ),
        )
        .add_systems(
            Update,
            (
                // Player
                player::ui::update_system,
                player::gun::select_fire::full_auto_system,
                player::gun::select_fire::semi_auto_system,
                player::gun::select_fire::toggle_select_fire_system,
                // Systems
                system::gameover_system,
                target::health::update_system,
                player::gun::bullet::health::update_system,
                player::health::update_system,
            )
                .run_if(in_state(GameMode::ShootingRange)),
        )
        .add_systems(
            FixedUpdate,
            (
                // Player movement systems
                player::movement::keyboard::update_system,
                player::movement::mouse::update_system,
                player::movement::controller::update_system,
            )
                .run_if(in_state(GameMode::ShootingRange)),
        )
        //.add_systems(
        //    OnEnter(GameMode::Versus),
        //    (
        //        scenes::versus::setup_system,
        //        player::setup_system,
        //        player::ui::setup_system,
        //    ),
        //)
        //.add_systems(
        //    Update,
        //    (
        //        // Player
        //        player::movement::keyboard::update_system,
        //        player::movement::mouse::update_system,
        //        player::movement::controller::update_system,
        //        player::ui::update_system,
        //        player::gun::select_fire::full_auto_system,
        //        player::gun::select_fire::semi_auto_system,
        //        player::gun::select_fire::toggle_select_fire_system,
        //        // Systems
        //        scenes::versus::multi_player_system,
        //        target::health::update_system,
        //        player::health::update_system,
        //    )
        //        .run_if(in_state(GameMode::Versus)),
        //)
        .add_systems(
            FixedUpdate,
            (
                // Player
                player::gun::gun_cooling_system,
                // Systems
                system::collision_detection_system,
            ),
        )
        .run();

    Ok(())
}
