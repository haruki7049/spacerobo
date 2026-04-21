//! # Player systems, Compoments & etc...

use avian3d::prelude::*;
use bevy::prelude::*;
use spacerobo_commons::{Controllable, GameMode, Hp, KillCounter, Playable, configs::GameConfigs};
use spacerobo_gun::{Gun, Interval, Muzzle, select_fire::SelectFire};

pub fn spawn_player(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    kill_counter: &mut ResMut<KillCounter>,
    asset_server: Res<AssetServer>,
) {
    // Reset KillCounter
    kill_counter.reset();

    let gap = 4.0;

    // Camera
    commands
        .spawn((
            DespawnOnExit(GameMode::InGame),
            Camera3d::default(),
            Transform::from_xyz(0., 0., 0.),
            RigidBody::Dynamic,
            GravityScale(0.2),
            Collider::sphere(1.0),
            Mass(5.0),
            AngularVelocity(Vec3::ZERO),
            SpatialListener::new(gap),
            Hp::robo(Some(asset_server.load("SE/kill.ogg"))),
            Controllable,
            Playable,
        ))
        // Gun
        .with_children(|parent| {
            parent
                .spawn((
                    Transform::from_xyz(1., -1., -3.),
                    Mesh3d(meshes.add(Extrusion::new(Circle::new(0.125), 2.))),
                    MeshMaterial3d(materials.add(Color::BLACK)),
                    (Gun {
                        owner: parent.target_entity(),
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

            debug!("Gun's parent.target_entity(): {:?}", parent.target_entity());
        });
}

/// setup system to spawn player entity
pub fn setup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut kill_counter: ResMut<KillCounter>,
    asset_server: Res<AssetServer>,
) {
    spawn_player(
        &mut commands,
        &mut meshes,
        &mut materials,
        &mut kill_counter,
        asset_server,
    );
}

#[allow(clippy::too_many_arguments)]
pub fn respawn_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut kill_counter: ResMut<KillCounter>,
    query: Query<&Playable>,
    game_configs: Res<GameConfigs>,
    keyboard: Res<ButtonInput<KeyCode>>,
    asset_server: Res<AssetServer>,
) {
    if !query.is_empty() {
        return;
    }

    if keyboard.just_pressed(game_configs.player.keyboard.respawn) {
        info!("Respawning player...");

        spawn_player(
            &mut commands,
            &mut meshes,
            &mut materials,
            &mut kill_counter,
            asset_server,
        );
    }
}
