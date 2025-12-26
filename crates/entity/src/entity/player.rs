//! # Player systems, Compoments & etc...

pub mod gun;
pub mod health;
pub mod movement;
pub mod ui;

use avian3d::prelude::*;
use bevy::prelude::*;
use gun::{Gun, Interval, Muzzle, select_fire::SelectFire};
use spacerobo_commons::{DeathEvent, GameMode, Hp, KillCounter, configs::GameConfigs};

/// Player Component
#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DeathEvent>();
        app.insert_resource(KillCounter::default());
        app.add_systems(
            OnEnter(GameMode::InGame),
            (setup_system, ui::setup_system),
        );
        app.add_systems(
            Update,
            (
                respawn_system,
                ui::update_system,
                gun::select_fire::full_auto_system,
                gun::select_fire::semi_auto_system,
                gun::select_fire::toggle_select_fire_system,
                gun::bullet::health::update_system,
                health::update_system,
            )
                .run_if(in_state(GameMode::InGame)),
        );
        app.add_systems(
            FixedUpdate,
            (
                // movement systems
                movement::keyboard::update_system,
                movement::mouse::update_system,
                movement::controller::update_system,
                // gun systems
                gun::gun_cooling_system,
            )
                .run_if(in_state(GameMode::InGame)),
        );
    }
}

/// setup system to spawn player entity
pub fn setup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut kill_counter: ResMut<KillCounter>,
) {
    // Reset KillCounter
    kill_counter.reset();

    let gap = 4.0;

    // Camera
    commands
        .spawn((
            StateScoped(GameMode::InGame),
            Camera3d::default(),
            Transform::from_xyz(0., 0., 0.),
            RigidBody::Dynamic,
            GravityScale(0.2),
            Collider::sphere(1.0),
            Mass(5.0),
            AngularVelocity(Vec3::ZERO),
            SpatialListener::new(gap),
            Hp::player(),
            Player,
        ))
        // Gun
        .with_children(|parent| {
            parent
                .spawn((
                    Transform::from_xyz(1., -1., -3.),
                    Mesh3d(meshes.add(Extrusion::new(Circle::new(0.125), 2.))),
                    MeshMaterial3d(materials.add(Color::BLACK)),
                    (Gun {
                        select_fire: SelectFire::Full,
                        interval: Interval {
                            limit: 0.1,
                            rest: 0.0,
                            amount: 0.01,
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
                    Transform::from_xyz(1., -1., -4.3).looking_to(Vec3::NEG_Z, Vec3::ZERO),
                ))
                // Muzzle
                .with_child((
                    Transform::from_xyz(1., -1., -4.3),
                    Muzzle,
                    RigidBody::Static,
                ));
        });
}

pub fn respawn_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut kill_counter: ResMut<KillCounter>,
    query: Query<&Player>,
    game_configs: Res<GameConfigs>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if !query.is_empty() {
        return;
    }

    if keyboard.just_pressed(game_configs.player().keyboard().respawn()) {
        info!("Respawning player...");

        // Reset KillCounter
        kill_counter.reset();

        let gap = 4.0;

        // Camera
        commands
            .spawn((
                StateScoped(GameMode::InGame),
                Camera3d::default(),
                Transform::from_xyz(0., 0., 0.),
                RigidBody::Dynamic,
                GravityScale(0.2),
                Collider::sphere(1.0),
                Mass(5.0),
                AngularVelocity(Vec3::ZERO),
                SpatialListener::new(gap),
                Hp::player(),
                Player,
            ))
            // Gun
            .with_children(|parent| {
                parent
                    .spawn((
                        Transform::from_xyz(1., -1., -3.),
                        Mesh3d(meshes.add(Extrusion::new(Circle::new(0.125), 2.))),
                        MeshMaterial3d(materials.add(Color::BLACK)),
                        (Gun {
                            select_fire: SelectFire::Full,
                            interval: Interval {
                                limit: 0.1,
                                rest: 0.0,
                                amount: 0.01,
                            },
                        }),
                    ))
                    // Muzzle
                    .with_child((
                        Transform::from_xyz(1., -1., -4.3),
                        Muzzle,
                        RigidBody::Static,
                    ));
            });
    }
}
