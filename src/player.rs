#![allow(clippy::type_complexity)]

pub mod config;
pub mod gun;
pub mod movement;
pub mod ui;

use crate::player::{
    config::{Config, ControllerConfig, KeyboardConfig, MouseConfig},
    gun::{Gun, Interval, Muzzle, SelectFire},
};
use avian3d::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    config: Config,
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let gap = 4.0;

    // Camera
    commands
        .spawn((
            Camera3d::default(),
            Transform::from_xyz(0., 0., 0.),
            RigidBody::Dynamic,
            GravityScale(0.2),
            Collider::sphere(1.0),
            AngularVelocity(Vec3::ZERO),
            SpatialListener::new(gap),
            (Player {
                config: Config {
                    keyboard: KeyboardConfig::default(),
                    mouse: MouseConfig::default(),
                    controller: ControllerConfig::default(),
                },
            }),
        ))
        // Gun
        .with_child((
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
}
