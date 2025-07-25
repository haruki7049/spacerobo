pub mod health;

use crate::{GameMode, Hp};
use avian3d::prelude::*;
use bevy::{
    color::palettes::basic::{BLACK, BLUE, GREEN, RED, SILVER, YELLOW},
    prelude::*,
};

#[derive(Component)]
pub struct Target;

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

    let targets_translation: Vec<(Vec3, Color)> = vec![
        (Vec3::new(10.0, 0.0, 0.0), RED.into()),
        (Vec3::new(0.0, 10.0, 0.0), BLUE.into()),
        (Vec3::new(0.0, 0.0, 10.0), GREEN.into()),
        (Vec3::new(-10.0, 0.0, 0.0), YELLOW.into()),
        (Vec3::new(0.0, -10.0, 0.0), SILVER.into()),
        (Vec3::new(0.0, 0.0, -10.0), BLACK.into()),
    ];

    // Targets
    for (translation, base_color) in targets_translation {
        commands.spawn((
            StateScoped(GameMode::ShootingRange),
            Mesh3d(meshes.add(Sphere::default().mesh())),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color,
                ..Default::default()
            })),
            Transform::from_translation(translation),
            RigidBody::Static,
            Collider::sphere(1.0),
            CollisionEventsEnabled,
            Mass(1.0),
            Target,
            Hp::default(),
        ));
    }
}
