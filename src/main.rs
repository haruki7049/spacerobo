use avian3d::prelude::*;
use bevy::{color::palettes::basic::SILVER, prelude::*};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .insert_resource(Gravity(Vec3::NEG_Y * 0.))
        .add_systems(Startup, setup)
        .add_systems(Startup, spacerobo::player::setup)
        .add_systems(Update, spacerobo::player::controller_system)
        .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // Spawn ground and generate a collider for the mesh using ColliderConstructor
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(8.0, 8.0))),
        MeshMaterial3d(materials.add(Color::from(SILVER))),
        ColliderConstructor::TrimeshFromMesh,
        RigidBody::Static,
    ));

    // Light
    commands.spawn((
        PointLight {
            intensity: 1_000_000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(2.0, 8.0, 2.0),
    ));
}
