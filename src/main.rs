use bevy::{
    color::palettes::basic::BLACK,
    color::palettes::basic::SILVER,
    prelude::*,
};


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, window_controls)
        .add_systems(Update, rotate_light)
        .run();
}

#[derive(Component)]
struct Shape;

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
            MeshMaterial3d(materials.add(Color::from(BLACK))),
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
    ));
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

fn rotate_light(
    mut query: Query<&mut Transform, With<Shape>>,
    time: Res<Time>,
) {
    for mut transform in &mut query {
        transform.rotate_x(time.delta_secs());
        transform.rotate_y(time.delta_secs());
        transform.rotate_z(time.delta_secs());
    }
}
