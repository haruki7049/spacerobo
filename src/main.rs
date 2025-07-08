use avian3d::prelude::*;
use bevy::{
    color::palettes::basic::{BLACK, BLUE, GREEN, RED, SILVER, YELLOW},
    prelude::*,
    window::{CursorGrabMode, CursorOptions},
};
use clap::Parser;
use serde::{Deserialize, Serialize};
use spacerobo::player;
use spacerobo::target::Target;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(version, author, about)]
struct CLIArgs {
    config: Option<PathBuf>,
}

#[derive(Resource, Serialize, Deserialize, Default)]
pub struct GameSettings {}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: CLIArgs = CLIArgs::parse();

    let settings_path: PathBuf = args.config.unwrap_or_else(|| {
        confy::get_configuration_file_path("spacerobo", "config")
            .expect("Failed to get path for spacerobo")
    });
    let settings: GameSettings = confy::load_path(&settings_path)?;

    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    cursor_options: CursorOptions {
                        visible: false,
                        grab_mode: if cfg!(target_os = "macos") {
                            CursorGrabMode::Locked
                        } else {
                            CursorGrabMode::Confined
                        },
                        ..default()
                    },
                    title: format!("spacerobo {}", env!("CARGO_PKG_VERSION")),
                    ..default()
                }),
                ..default()
            }),
            PhysicsPlugins::default(),
        ))
        .insert_resource(Gravity(Vec3::NEG_Y * 0.))
        .insert_resource(settings)
        .add_systems(Startup, setup)
        .add_systems(Startup, player::setup)
        .add_systems(
            Update,
            (
                player::keyboard_mouse_system,
                player::controller_system,
                player::ui::ui_system,
                player::ui::exit_system,
                player::gun::gun_shoot_system,
                player::gun::bullet_hit_detection_system,
            ),
        )
        .run();

    Ok(())
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // Light
    commands.spawn((
        PointLight {
            intensity: 1_000_000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(2.0, 8.0, 2.0),
    ));

    // Targets
    commands.spawn((
        Mesh3d(meshes.add(Sphere::default().mesh())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: RED.into(),
            ..Default::default()
        })),
        Transform::from_xyz(10.0, 0.0, 0.0),
        RigidBody::Static,
        Collider::sphere(1.0),
        CollisionEventsEnabled,
        Target,
    ));

    commands.spawn((
        Mesh3d(meshes.add(Sphere::default().mesh())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: BLUE.into(),
            ..Default::default()
        })),
        Transform::from_xyz(0.0, 10.0, 0.0),
        RigidBody::Static,
        Collider::sphere(1.0),
        CollisionEventsEnabled,
        Target,
    ));

    commands.spawn((
        Mesh3d(meshes.add(Sphere::default().mesh())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: GREEN.into(),
            ..Default::default()
        })),
        Transform::from_xyz(-10.0, 0.0, 0.0),
        RigidBody::Static,
        Collider::sphere(1.0),
        CollisionEventsEnabled,
        Target,
    ));

    commands.spawn((
        Mesh3d(meshes.add(Sphere::default().mesh())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: YELLOW.into(),
            ..Default::default()
        })),
        Transform::from_xyz(0.0, -10.0, 0.0),
        RigidBody::Static,
        Collider::sphere(1.0),
        CollisionEventsEnabled,
        Target,
    ));

    commands.spawn((
        Mesh3d(meshes.add(Sphere::default().mesh())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: SILVER.into(),
            ..Default::default()
        })),
        Transform::from_xyz(0.0, 0.0, 10.0),
        RigidBody::Static,
        Collider::sphere(1.0),
        CollisionEventsEnabled,
        Target,
    ));

    commands.spawn((
        Mesh3d(meshes.add(Sphere::default().mesh())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: BLACK.into(),
            ..Default::default()
        })),
        Transform::from_xyz(0.0, 0.0, -10.0),
        RigidBody::Static,
        Collider::sphere(1.0),
        CollisionEventsEnabled,
        Target,
    ));
}
