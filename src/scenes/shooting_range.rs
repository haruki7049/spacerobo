use bevy::{
    color::palettes::basic::{BLACK, BLUE, GREEN, RED, SILVER, YELLOW},
    prelude::*,
};
use avian3d::prelude::*;
use crate::{Hp, target::Target, GameMode};

pub fn setup_system(
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
