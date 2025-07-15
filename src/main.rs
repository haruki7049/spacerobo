use avian3d::prelude::*;
use bevy::{
    color::palettes::basic::{BLACK, BLUE, GREEN, RED, SILVER, YELLOW},
    prelude::*,
    window::{CursorGrabMode, CursorOptions},
};
use clap::Parser;
use spacerobo::{
    CLIArgs, DeathEvent, GameMode, Hp, player, system,
    target::{self, Target},
    title,
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
        .init_state::<GameMode>()
        .add_event::<DeathEvent>()
        .insert_resource(args)
        .insert_resource(Gravity(Vec3::NEG_Y * 0.))
        .insert_resource(Time::<Virtual>::default())
        .add_systems(
            OnEnter(GameMode::Title),
            title::setup_system.run_if(in_state(GameMode::Title)),
        )
        .add_systems(
            Update,
            (title::input_detection_system).run_if(in_state(GameMode::Title)),
        )
        .add_systems(
            OnEnter(GameMode::ShootingRange),
            (setup_system, player::setup_system, player::ui::setup_system),
        )
        .add_systems(
            Update,
            (
                // Player
                player::movement::keyboard::update_system,
                player::movement::mouse::update_system,
                player::movement::controller::update_system,
                player::ui::update_system,
                player::gun::select_fire::full_auto_system,
                player::gun::select_fire::semi_auto_system,
                player::gun::select_fire::toggle_select_fire_system,
                // Systems
                system::gameover_system,
                target::health_system,
                player::system::health_system,
            )
                .run_if(in_state(GameMode::ShootingRange)),
        )
        .add_systems(
            FixedUpdate,
            (
                // Player
                player::gun::gun_cooling_system,
                // Systems
                system::collision_detection_system,
            )
                .run_if(in_state(GameMode::ShootingRange)),
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
        StateScoped(GameMode::ShootingRange),
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
        StateScoped(GameMode::ShootingRange),
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
        StateScoped(GameMode::ShootingRange),
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
        StateScoped(GameMode::ShootingRange),
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
        StateScoped(GameMode::ShootingRange),
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
        StateScoped(GameMode::ShootingRange),
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
