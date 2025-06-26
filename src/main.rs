use bevy::{
    color::palettes::basic::RED,
    color::palettes::basic::SILVER,
    prelude::*,
};


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, window_controls)
        .add_systems(Update, control_position)
        .run();
}


/// A marker component for our shapes so we can query them separately from the ground plane
#[derive(Component)]
struct Shape;

/// A marker component for the camera
#[derive(Component)]
struct Player;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let extrusions = [
        meshes.add(Extrusion::new(Triangle2d::default(), 1.)),
    ];

    for shape in extrusions.into_iter() {
        commands.spawn((
            Mesh3d(shape),
            MeshMaterial3d(materials.add(Color::from(RED))),
            Transform::from_xyz(0.0, 2.0, 0.0),
            Shape,
        ));
    }

    // Light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            intensity: 10_000_000.,
            range: 100.0,
            shadow_depth_bias: 0.2,
            ..default()
        },
        Transform::from_xyz(8.0, 16.0, 8.0),
    ));

    // Ground plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0).subdivisions(10))),
        MeshMaterial3d(materials.add(Color::from(SILVER))),
    ));

    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 7., 14.0).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
        Player,
    ));
}

fn control_position(
    mut query: Query<&mut Transform, With<Shape>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.pressed(KeyCode::ArrowRight) {
        for mut transform in &mut query {
            transform.translation.x += 0.1;
        }
    }

    if input.pressed(KeyCode::ArrowLeft) {
        for mut transform in &mut query {
            transform.translation.x -= 0.1;
        }
    }

    if input.pressed(KeyCode::ArrowDown) {
        for mut transform in &mut query {
            transform.translation.z += 0.1;
        }
    }

    if input.pressed(KeyCode::ArrowUp) {
        for mut transform in &mut query {
            transform.translation.z -= 0.1;
        }
    }
}

fn window_controls(
    mut commands: Commands,
    focused_windows: Query<(Entity, &Window)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for (window, focus) in focused_windows.iter() {
        if !focus.focused {
            continue;
        }

        if input.just_pressed(KeyCode::Escape) {
            commands.entity(window).despawn();
        }
    }
}
