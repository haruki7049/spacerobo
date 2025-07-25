//! # Player systems, Compoments & etc...

#![allow(clippy::type_complexity)]

pub mod config;
pub mod gun;
pub mod health;
pub mod movement;
pub mod ui;

use crate::{
    cli::CLIArgs, configs::GameConfigs, GameMode, Hp,
    player::{
        config::Config,
        gun::{Gun, Interval, Muzzle, select_fire::SelectFire},
    },
};
use avian3d::prelude::*;
use bevy::prelude::*;

/// Player Component
#[derive(Component)]
pub struct Player {
    config: Config,
}

/// setup system to spawn player entity
pub fn setup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    cli_args: Res<CLIArgs>,
) {
    let configs: GameConfigs = confy::load_path(&cli_args.config_file).unwrap_or_else(|_| {
        info!("Running Spacerobo with default GameConfigs...");
        GameConfigs::default()
    });
    let gap = 4.0;

    debug!("Your GameConfigs: {:?}", configs);

    // Camera
    commands
        .spawn((
            StateScoped(GameMode::ShootingRange),
            Camera3d::default(),
            Transform::from_xyz(0., 0., 0.),
            RigidBody::Dynamic,
            GravityScale(0.2),
            Collider::sphere(1.0),
            Mass(5.0),
            AngularVelocity(Vec3::ZERO),
            SpatialListener::new(gap),
            (Player {
                config: configs.player,
            }),
            Hp::default(),
        ))
        // Gun
        .with_children(|parent| {
            parent
                .spawn((
                    Transform::from_xyz(1., -1., -3.),
                    Mesh3d(meshes.add(Extrusion::new(Circle::new(0.125), 2.))),
                    MeshMaterial3d(materials.add(Color::BLACK)),
                    (Gun {
                        select_fire: SelectFire::Full,
                        interval: Interval {
                            limit: 0.1,
                            rest: 0.0,
                            amount: 0.01,
                        },
                    }),
                ))
                // Muzzle
                .with_child((Transform::from_xyz(1., -1., -4.3), Muzzle));
        });
}
