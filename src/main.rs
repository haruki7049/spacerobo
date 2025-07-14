use avian3d::prelude::*;
use bevy::{
    color::palettes::basic::{BLACK, BLUE, GREEN, RED, SILVER, YELLOW},
    prelude::*,
    window::{CursorGrabMode, CursorOptions},
};
use clap::Parser;
use spacerobo::{
    CLIArgs, DeathEvent, Hp, player, system,
    target::{self, Target},
};

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
        .add_event::<DeathEvent>()
        .insert_resource(args)
        .insert_resource(Gravity(Vec3::NEG_Y * 0.))
        .insert_resource(Time::<Virtual>::default())
        .add_systems(
            Startup,
            (setup_system, player::setup_system, player::ui::setup_system),
        )
        .add_systems(
            Update,
            (
                player::movement::keyboard::update_system,
                player::movement::mouse::update_system,
                player::movement::controller::update_system,
                player::ui::ui_system,
                player::ui::exit_system,
                player::ui::time_pause_system,
                player::gun::gun_shoot_system,
                player::gun::toggle_select_fire_system,
            ),
        )
        .add_systems(FixedUpdate, player::gun::gun_cooling_system)
        .add_systems(
            Update,
            (
                system::collision_detection_system,
                target::health_system,
                player::system::health_system,
            ),
        )
        .run();

    Ok(())
}

fn setup_system(
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
        Mass(1.0),
        Target,
        Hp::default(),
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
        Mass(1.0),
        Target,
        Hp::default(),
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
        Mass(1.0),
        Target,
        Hp::default(),
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
        Mass(1.0),
        Target,
        Hp::default(),
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
        Mass(1.0),
        Target,
        Hp::default(),
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
        Mass(1.0),
        Target,
        Hp::default(),
    ));
}
