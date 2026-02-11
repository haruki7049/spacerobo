use avian3d::prelude::*;
use bevy::{
    color::palettes::basic::{BLUE, GREEN, RED, WHITE, YELLOW},
    prelude::*,
};
use spacerobo_commons::{DeathEvent, GameMode, Hp, KillCounter};
use spacerobo_entity::entity::{bot, EntityPlugins};
use spacerobo_target::Target;

pub struct ShootingRangePlugin;

impl Plugin for ShootingRangePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EntityPlugins);
        app.add_event::<DeathEvent>();
        app.insert_resource(Gravity(Vec3::NEG_Y * 0.));
        app.insert_resource(KillCounter::default());
        app.add_systems(OnEnter(GameMode::InGame), setup_system);
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

    // Bots
    commands
        .spawn((
            StateScoped(GameMode::InGame),
            Mesh3d(meshes.add(Sphere::default().mesh())),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: RED.into(),
                ..Default::default()
            })),
            Transform::from_xyz(100.0, 0.0, 0.0),
            RigidBody::Static,
            Collider::sphere(1.0),
            CollisionEventsEnabled,
            Mass(1.0),
            bot::Bot,
            Hp::robo(Some(asset_server.load("SE/kill.ogg"))),
        ))
        // Gun
        .with_children(|parent| {
            parent
                .spawn((
                    Transform::from_xyz(0., 0., -0.5),
                    Mesh3d(meshes.add(Extrusion::new(Circle::new(0.125), 1.))),
                    MeshMaterial3d(materials.add(Color::BLACK)),
                    (bot::gun::Gun {
                        interval: bot::gun::Interval {
                            limit: 0.1,
                            rest: 0.0,
                            amount: 0.005,
                        },
                    }),
                ))
                // Spot light
                .with_child((
                    SpotLight {
                        intensity: 100_000_000.0,
                        range: 100_000_000.0,
                        outer_angle: std::f32::consts::FRAC_PI_4 / 2.0,
                        shadows_enabled: true,
                        ..default()
                    },
                    Transform::from_xyz(0., 0., -1.).looking_to(Vec3::NEG_Z, Vec3::ZERO),
                ))
                // Muzzle
                .with_child((
                    Transform::from_xyz(0., 0., -1.),
                    bot::gun::Muzzle,
                    RigidBody::Static,
                ));
        });

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
        StateScoped(GameMode::InGame),
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
    mut collision_event_reader: EventReader<CollisionStarted>,
    mut query: Query<(&mut Hp, &LinearVelocity, &Mass)>,
    mut event_writer: EventWriter<DeathEvent>,
    asset_server: Res<AssetServer>,
) {
    for CollisionStarted(entity1, entity2) in collision_event_reader.read() {
        debug!("Collision!!");

        let objects = query.get_many_mut([*entity1, *entity2]).ok();

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
                    event_writer.write(DeathEvent::new(
                        *entity1,
                        Some(asset_server.load("SE/kill.ogg")),
                    ));
                }
                if obj2_hp.rest <= 0. {
                    event_writer.write(DeathEvent::new(
                        *entity2,
                        Some(asset_server.load("SE/kill.ogg")),
                    ));
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
    mut event_writer: EventWriter<DeathEvent>,
    asset_server: Res<AssetServer>,
) {
    for (transform, entity) in query.iter_mut() {
        if transform.translation.x > 2000.0
            || transform.translation.y > 2000.0
            || transform.translation.z > 2000.0
            || transform.translation.x < -2000.0
            || transform.translation.y < -2000.0
            || transform.translation.z < -2000.0
        {
            debug!("Creating DeathEvent by area outside...");
            event_writer.write(DeathEvent::new(
                entity,
                Some(asset_server.load("SE/kill.ogg")),
            ));
        }
    }
}

pub fn death_system(
    mut commands: Commands,
    mut event_reader: EventReader<DeathEvent>,
    query: Query<&Hp>,
) {
    for death_event in event_reader.read() {
        if query.get(death_event.entity).is_ok() {
            commands.entity(death_event.entity).despawn();
            if let Some(handle) = death_event.sound.clone() {
                commands.spawn(AudioPlayer::new(handle));
            }

            debug!("{:?} which has Hp component is dead!!", death_event.entity);
        }
    }
}
