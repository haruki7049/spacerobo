use avian3d::prelude::*;
use bevy::{color::palettes::basic::BLACK, color::palettes::basic::SILVER, prelude::*};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .insert_resource(Gravity(Vec3::NEG_Y * 0.))
        .add_systems(Startup, setup)
        .add_systems(Update, accelerate_angular)
        .run();
}

#[derive(Component)]
struct Cube;

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

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(Color::from(BLACK))),
        Transform::from_xyz(0.0, 2.0, 0.0),
        // RigidBody
        RigidBody::Dynamic,
        GravityScale(0.2),
        // Collider
        Collider::cuboid(1.0, 1.0, 1.0),
        // Velocity
        AngularVelocity(Vec3::ZERO),
        Cube,
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

    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-5.0, 3.5, 5.5).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn accelerate_angular(
    mut query: Query<&mut AngularVelocity, With<Cube>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        for mut angular_velocity in &mut query {
            angular_velocity.x += 0.5;
            angular_velocity.y += 0.5;
            angular_velocity.z += 0.5;
        }
    }
}
