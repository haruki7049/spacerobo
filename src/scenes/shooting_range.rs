mod entities;
mod health;

use crate::{DeathEvent, GameMode, Hp, KillCounter};
use avian3d::prelude::*;
use bevy::{
    color::palettes::basic::{BLUE, GREEN, RED, WHITE, YELLOW},
    prelude::*,
};

pub struct ShootingRangePlugin;

impl Plugin for ShootingRangePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameMode>();
        app.add_event::<DeathEvent>();
        app.insert_resource(Gravity(Vec3::NEG_Y * 0.));
        app.insert_resource(KillCounter::default());
        app.add_systems(
            OnEnter(GameMode::ShootingRange),
            (
                setup_system,
                entities::player::setup_system,
                entities::player::ui::setup_system,
            ),
        );
        app.add_systems(
            Update,
            (
                // Player
                entities::player::respawn_system,
                entities::player::ui::update_system,
                entities::player::gun::select_fire::full_auto_system,
                entities::player::gun::select_fire::semi_auto_system,
                entities::player::gun::select_fire::toggle_select_fire_system,
                entities::player::gun::bullet::health::update_system,
                entities::player::health::update_system,
                // Bot
                entities::bot::gun::select_fire::full_auto_system,
                entities::bot::gun::bullet::health::update_system,
                entities::bot::health::update_system,
                // Systems
                health::update_system,
                collision_detection_system,
                when_going_outside_system,
            )
                .run_if(in_state(GameMode::ShootingRange)),
        );
        app.add_systems(
            FixedUpdate,
            (
                // Player movement systems
                entities::player::movement::keyboard::update_system,
                entities::player::movement::mouse::update_system,
                entities::player::movement::controller::update_system,
                // Player gun systems
                entities::player::gun::gun_cooling_system,
                // Bot gun systems
                entities::bot::gun::gun_cooling_system,
            )
                .run_if(in_state(GameMode::ShootingRange)),
        );
    }
}

#[derive(Component)]
pub struct Target;

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

    // Bots
    commands
        .spawn((
            StateScoped(GameMode::ShootingRange),
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
            entities::bot::Bot,
            Hp::target(),
        ))
        // Gun
        .with_children(|parent| {
            parent
                .spawn((
                    Transform::from_xyz(0., 0., -0.5),
                    Mesh3d(meshes.add(Extrusion::new(Circle::new(0.125), 1.))),
                    MeshMaterial3d(materials.add(Color::BLACK)),
                    (entities::bot::gun::Gun {
                        interval: entities::bot::gun::Interval {
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
                    entities::bot::gun::Muzzle,
                    RigidBody::Static,
                ));
        });

    // Targets
    for i in 1..5 {
        for j in 1..5 {
            for k in 1..5 {
                commands.spawn((
                    StateScoped(GameMode::ShootingRange),
                    Mesh3d(meshes.add(Sphere::default().mesh())),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color: RED.into(),
                        ..Default::default()
                    })),
                    Transform::from_xyz(10.0 * i as f32, 10.0 * j as f32, 10.0 * k as f32),
                    RigidBody::Static,
                    Collider::sphere(1.0),
                    CollisionEventsEnabled,
                    Mass(1.0),
                    Target,
                    Hp::target(),
                ));

                commands.spawn((
                    StateScoped(GameMode::ShootingRange),
                    Mesh3d(meshes.add(Sphere::default().mesh())),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color: WHITE.into(),
                        ..Default::default()
                    })),
                    Transform::from_xyz(10.0 * i as f32, 10.0 * j as f32, -10.0 * k as f32),
                    RigidBody::Static,
                    Collider::sphere(1.0),
                    CollisionEventsEnabled,
                    Mass(1.0),
                    Target,
                    Hp::target(),
                ));

                commands.spawn((
                    StateScoped(GameMode::ShootingRange),
                    Mesh3d(meshes.add(Sphere::default().mesh())),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color: WHITE.into(),
                        ..Default::default()
                    })),
                    Transform::from_xyz(10.0 * i as f32, -10.0 * j as f32, 10.0 * k as f32),
                    RigidBody::Static,
                    Collider::sphere(1.0),
                    CollisionEventsEnabled,
                    Mass(1.0),
                    Target,
                    Hp::target(),
                ));

                commands.spawn((
                    StateScoped(GameMode::ShootingRange),
                    Mesh3d(meshes.add(Sphere::default().mesh())),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color: GREEN.into(),
                        ..Default::default()
                    })),
                    Transform::from_xyz(10.0 * i as f32, -10.0 * j as f32, -10.0 * k as f32),
                    RigidBody::Static,
                    Collider::sphere(1.0),
                    CollisionEventsEnabled,
                    Mass(1.0),
                    Target,
                    Hp::target(),
                ));

                commands.spawn((
                    StateScoped(GameMode::ShootingRange),
                    Mesh3d(meshes.add(Sphere::default().mesh())),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color: WHITE.into(),
                        ..Default::default()
                    })),
                    Transform::from_xyz(-10.0 * i as f32, 10.0 * j as f32, 10.0 * k as f32),
                    RigidBody::Static,
                    Collider::sphere(1.0),
                    CollisionEventsEnabled,
                    Mass(1.0),
                    Target,
                    Hp::target(),
                ));

                commands.spawn((
                    StateScoped(GameMode::ShootingRange),
                    Mesh3d(meshes.add(Sphere::default().mesh())),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color: YELLOW.into(),
                        ..Default::default()
                    })),
                    Transform::from_xyz(-10.0 * i as f32, 10.0 * j as f32, -10.0 * k as f32),
                    RigidBody::Static,
                    Collider::sphere(1.0),
                    CollisionEventsEnabled,
                    Mass(1.0),
                    Target,
                    Hp::target(),
                ));

                commands.spawn((
                    StateScoped(GameMode::ShootingRange),
                    Mesh3d(meshes.add(Sphere::default().mesh())),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color: BLUE.into(),
                        ..Default::default()
                    })),
                    Transform::from_xyz(-10.0 * i as f32, -10.0 * j as f32, 10.0 * k as f32),
                    RigidBody::Static,
                    Collider::sphere(1.0),
                    CollisionEventsEnabled,
                    Mass(1.0),
                    Target,
                    Hp::target(),
                ));

                commands.spawn((
                    StateScoped(GameMode::ShootingRange),
                    Mesh3d(meshes.add(Sphere::default().mesh())),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color: WHITE.into(),
                        ..Default::default()
                    })),
                    Transform::from_xyz(-10.0 * i as f32, -10.0 * j as f32, -10.0 * k as f32),
                    RigidBody::Static,
                    Collider::sphere(1.0),
                    CollisionEventsEnabled,
                    Mass(1.0),
                    Target,
                    Hp::target(),
                ));
            }
        }
    }
}

/// This system detects the hits between two objects, having Hp, LinearVelocity and Mass Components.
/// This system is created to decrease the hp at contacted objects.
fn collision_detection_system(
    mut collision_event_reader: EventReader<CollisionStarted>,
    mut query: Query<(&mut Hp, &LinearVelocity, &Mass)>,
    mut event_writer: EventWriter<DeathEvent>,
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

                obj1_hp.rest -= damage;
                obj2_hp.rest -= damage;

                debug!("The first object's Hp: {:?}", &obj1_hp);
                debug!("The second object's Hp: {:?}", &obj2_hp);

                if obj1_hp.rest <= 0. {
                    event_writer.write(DeathEvent::new(*entity1));
                }
                if obj2_hp.rest <= 0. {
                    event_writer.write(DeathEvent::new(*entity2));
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
            event_writer.write(DeathEvent::new(entity));
        }
    }
}
