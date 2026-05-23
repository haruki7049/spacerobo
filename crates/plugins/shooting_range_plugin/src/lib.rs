use avian3d::prelude::*;
use bevy::{
    color::palettes::basic::{BLUE, GREEN, RED, WHITE, YELLOW},
    prelude::*,
};
use spacerobo_commons::{DeathMessage, GameMode, Hp, KillCounter};
use spacerobo_player::PlayerCommonPlugin;
use spacerobo_target::Target;

pub struct ShootingRangePlugin;

impl Plugin for ShootingRangePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerCommonPlugin);
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
                collision_detection_system,
                when_going_outside_system,
                death_system,
            )
                .run_if(in_state(GameMode::InGame)),
        );
    }
}

fn setup_system(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
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
    for i in 1..5 {
        for j in 1..5 {
            for k in 1..5 {
                let i_float = i as f32;
                let j_float = j as f32;
                let k_float = k as f32;

                spawn_target(
                    &mut commands,
                    &mut meshes,
                    &asset_server,
                    &mut materials,
                    RED.into(),
                    Vec3::new(i_float * 10.0, j_float * 10.0, k_float * 10.0),
                );

                spawn_target(
                    &mut commands,
                    &mut meshes,
                    &asset_server,
                    &mut materials,
                    WHITE.into(),
                    Vec3::new(i_float * 10.0, j_float * 10.0, k_float * -10.0),
                );

                spawn_target(
                    &mut commands,
                    &mut meshes,
                    &asset_server,
                    &mut materials,
                    WHITE.into(),
                    Vec3::new(i_float * 10.0, j_float * -10.0, k_float * 10.0),
                );

                spawn_target(
                    &mut commands,
                    &mut meshes,
                    &asset_server,
                    &mut materials,
                    GREEN.into(),
                    Vec3::new(i_float * 10.0, j_float * -10.0, k_float * -10.0),
                );

                spawn_target(
                    &mut commands,
                    &mut meshes,
                    &asset_server,
                    &mut materials,
                    WHITE.into(),
                    Vec3::new(i_float * -10.0, j_float * 10.0, k_float * 10.0),
                );

                spawn_target(
                    &mut commands,
                    &mut meshes,
                    &asset_server,
                    &mut materials,
                    YELLOW.into(),
                    Vec3::new(i_float * -10.0, j_float * 10.0, k_float * -10.0),
                );

                spawn_target(
                    &mut commands,
                    &mut meshes,
                    &asset_server,
                    &mut materials,
                    BLUE.into(),
                    Vec3::new(i_float * -10.0, j_float * -10.0, k_float * 10.0),
                );

                spawn_target(
                    &mut commands,
                    &mut meshes,
                    &asset_server,
                    &mut materials,
                    WHITE.into(),
                    Vec3::new(i_float * -10.0, j_float * -10.0, k_float * -10.0),
                );
            }
        }
    }
}

fn spawn_target(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    asset_server: &Res<AssetServer>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    base_color: Color,
    vec3: Vec3,
) {
    commands.spawn((
        DespawnOnExit(GameMode::InGame),
        Mesh3d(meshes.add(Sphere::default().mesh())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color,
            ..Default::default()
        })),
        Transform::from_translation(vec3),
        RigidBody::Static,
        Collider::sphere(1.0),
        CollisionEventsEnabled,
        Mass(1.0),
        Target,
        Hp::robo(Some(asset_server.load("SE/kill.ogg"))),
    ));
}

/// This system detects the hits between two objects, having Hp, LinearVelocity and Mass Components.
/// This system is created to decrease the hp at contacted objects.
fn collision_detection_system(
    mut collision_event_reader: MessageReader<CollisionStart>,
    mut query: Query<(&mut Hp, &LinearVelocity, &Mass)>,
    mut event_writer: MessageWriter<DeathMessage>,
) {
    for event in collision_event_reader.read() {
        debug!("Collision!!");

        let entity1 = event.collider1;
        let entity2 = event.collider2;

        let objects = query.get_many_mut([entity1, entity2]).ok();

        match objects {
            Some([mut obj1, mut obj2]) => {
                let obj1_damage: f32 = calc_damage(&obj1);
                let obj2_damage: f32 = calc_damage(&obj2);
                let damage: f32 = obj1_damage + obj2_damage;

                let (ref mut obj1_hp, _obj1_linear, _obj1_mass) = obj1;
                let (ref mut obj2_hp, _obj2_linear, _obj2_mass) = obj2;

                obj1_hp.decrease(damage);
                obj2_hp.decrease(damage);

                debug!("The first object's Hp: {:?}", &obj1_hp);
                debug!("The second object's Hp: {:?}", &obj2_hp);

                if obj1_hp.rest <= 0. {
                    event_writer.write(DeathMessage::new(entity1));
                }
                if obj2_hp.rest <= 0. {
                    event_writer.write(DeathMessage::new(entity2));
                }
            }
            _ => debug!(
                "The collisioned entity, {} or {} is missing Hp, LinearVelocity or Mass",
                entity1, entity2
            ),
        }
    }
}

fn calc_damage(object: &(Mut<'_, Hp>, &LinearVelocity, &Mass)) -> f32 {
    let (_hp, linear, mass) = object;

    let speed: f32 = linear.x + linear.y + linear.z;

    // Speed * Mass = Force
    // By Isaac Newton
    // Probably...
    (speed * ***mass).abs()
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
