use avian3d::prelude::*;
use bevy::{
    color::palettes::basic::{BLUE, GREEN, RED, WHITE, YELLOW},
    prelude::*,
};
use spacerobo_commons::{Damage, DeathMessage, GameMode, Hp, KillCounter};
use spacerobo_player::PlayerCommonPlugin;
use spacerobo_target::{Target, TargetPlugin};

pub struct ShootingRangePlugin;

impl Plugin for ShootingRangePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerCommonPlugin);
        app.add_plugins(TargetPlugin);
        app.add_message::<DeathMessage>();
        app.insert_resource(Gravity(Vec3::NEG_Y * 0.));
        app.insert_resource(KillCounter::default());
        app.add_systems(
            OnEnter(GameMode::InGame),
            (setup_system, spawn_boundary_grid).run_if(in_state(GameMode::InGame)),
        );
        app.add_systems(
            Update,
            (
                // Systems
                when_going_outside_system,
                death_system,
            )
                .run_if(in_state(GameMode::InGame)),
        );
        app.add_observer(apply_damage_system);
    }
}

fn setup_system(mut commands: Commands) {
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
    for i in 1..5 {
        for j in 1..5 {
            for k in 1..5 {
                let i_float = i as f32;
                let j_float = j as f32;
                let k_float = k as f32;

                Target::spawn(
                    &mut commands,
                    Vec3::new(i_float * 10.0, j_float * 10.0, k_float * 10.0),
                    RED.into(),
                );

                Target::spawn(
                    &mut commands,
                    Vec3::new(i_float * 10.0, j_float * 10.0, k_float * -10.0),
                    WHITE.into(),
                );

                Target::spawn(
                    &mut commands,
                    Vec3::new(i_float * 10.0, j_float * -10.0, k_float * 10.0),
                    WHITE.into(),
                );

                Target::spawn(
                    &mut commands,
                    Vec3::new(i_float * 10.0, j_float * -10.0, k_float * -10.0),
                    GREEN.into(),
                );

                Target::spawn(
                    &mut commands,
                    Vec3::new(i_float * -10.0, j_float * 10.0, k_float * 10.0),
                    WHITE.into(),
                );

                Target::spawn(
                    &mut commands,
                    Vec3::new(i_float * -10.0, j_float * 10.0, k_float * -10.0),
                    YELLOW.into(),
                );

                Target::spawn(
                    &mut commands,
                    Vec3::new(i_float * -10.0, j_float * -10.0, k_float * 10.0),
                    BLUE.into(),
                );

                Target::spawn(
                    &mut commands,
                    Vec3::new(i_float * -10.0, j_float * -10.0, k_float * -10.0),
                    WHITE.into(),
                );
            }
        }
    }
}

fn when_going_outside_system(
    mut query: Query<(&Transform, Entity), With<Hp>>,
    mut event_writer: MessageWriter<DeathMessage>,
) {
    for (transform, entity) in query.iter_mut() {
        if transform.translation.x > 2000.0
            || transform.translation.y > 2000.0
            || transform.translation.z > 2000.0
            || transform.translation.x < -2000.0
            || transform.translation.y < -2000.0
            || transform.translation.z < -2000.0
        {
            debug!("Creating DeathMessage by area outside...");
            event_writer.write(DeathMessage::new(entity));
        }
    }
}

pub fn death_system(
    mut commands: Commands,
    mut event_reader: MessageReader<DeathMessage>,
    hp_query: Query<&Hp>,
) {
    for death_event in event_reader.read() {
        if let Ok(hp) = hp_query.get(death_event.entity) {
            commands.entity(death_event.entity).despawn();
            if let Some(handle) = hp.death_sound.clone() {
                commands.spawn(AudioPlayer::new(handle));
            }

            debug!("{:?} which has Hp component is dead!!", death_event.entity);
        }
    }
}

pub fn spawn_boundary_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let limit = 2000.0;
    let spacing = 200.0;
    let thickness = 2.0;

    // Mesh for horizontal and vertical lines
    let line_mesh_x = meshes.add(Cuboid::new(limit * 2.0, thickness, thickness));
    let line_mesh_y = meshes.add(Cuboid::new(thickness, limit * 2.0, thickness));

    let faces = [
        (Vec3::new(0.0, 0.0, limit), Quat::IDENTITY),
        (Vec3::new(0.0, 0.0, -limit), Quat::IDENTITY),
        (
            Vec3::new(limit, 0.0, 0.0),
            Quat::from_rotation_y(std::f32::consts::FRAC_PI_2),
        ),
        (
            Vec3::new(-limit, 0.0, 0.0),
            Quat::from_rotation_y(std::f32::consts::FRAC_PI_2),
        ),
        (
            Vec3::new(0.0, limit, 0.0),
            Quat::from_rotation_x(std::f32::consts::FRAC_PI_2),
        ),
        (
            Vec3::new(0.0, -limit, 0.0),
            Quat::from_rotation_x(std::f32::consts::FRAC_PI_2),
        ),
    ];

    for (face_pos, face_rot) in faces {
        // Create a unique material for each face
        let material = materials.add(StandardMaterial {
            base_color: Color::srgba(1.0, 0.0, 0.0, 0.8),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        });

        commands
            .spawn((
                Transform::from_translation(face_pos).with_rotation(face_rot),
                Visibility::default(),
            ))
            .with_children(|parent| {
                let mut i = -limit;
                while i <= limit {
                    // Horizontal line
                    parent.spawn((
                        Mesh3d(line_mesh_x.clone()),
                        MeshMaterial3d(material.clone()),
                        Transform::from_xyz(0.0, i, 0.0),
                    ));
                    // Vertical line
                    parent.spawn((
                        Mesh3d(line_mesh_y.clone()),
                        MeshMaterial3d(material.clone()),
                        Transform::from_xyz(i, 0.0, 0.0),
                    ));
                    i += spacing;
                }
            });
    }
}

fn apply_damage_system(
    damage: On<Damage>,
    mut query: Query<&mut Hp>,
    mut event_writer: MessageWriter<DeathMessage>,
) {
    if let Ok(mut hp) = query.get_mut(damage.target) {
        if hp.rest <= 0. {
            return;
        }

        hp.decrease(damage.amount);

        if hp.rest <= 0. {
            event_writer.write(DeathMessage::new(damage.target));
        }
    }
}
