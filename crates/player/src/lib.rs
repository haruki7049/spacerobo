//! # Player systems, Compoments & etc...

pub mod ui;

use avian3d::prelude::*;
use bevy::prelude::*;
use spacerobo_commons::{
    Controllable, DeathMessage, GameMode, Hp, KillCounter, Player, Weapon, configs::GameConfigs,
};
use spacerobo_gun::{Gun, GunPlugin};

/// Player Common Component
#[derive(Component)]
pub struct Common;

impl Player for Common {
    fn spawn(
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
                Common,
            ))
            // Gun
            .with_children(|parent| {
                let origin = Vec3::new(1.0, -1.0, -3.0);
                Gun::spawn_as_child(parent, meshes, materials, origin);

                debug!("Gun's parent.target_entity(): {:?}", parent.target_entity());
            });
    }
}

pub struct PlayerCommonPlugin;

impl Plugin for PlayerCommonPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GunPlugin);
        app.add_message::<DeathMessage>();
        app.insert_resource(KillCounter::default());
        app.add_systems(OnEnter(GameMode::InGame), (setup_system, ui::setup_system));
        app.add_systems(
            Update,
            (respawn_system, ui::update_system).run_if(in_state(GameMode::InGame)),
        );
    }
}

/// setup system to spawn player entity
pub fn setup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut kill_counter: ResMut<KillCounter>,
    asset_server: Res<AssetServer>,
) {
    Common::spawn(
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
    query: Query<&Common>,
    game_configs: Res<GameConfigs>,
    keyboard: Res<ButtonInput<KeyCode>>,
    asset_server: Res<AssetServer>,
) {
    if !query.is_empty() {
        return;
    }

    if keyboard.just_pressed(game_configs.player.keyboard.respawn) {
        info!("Respawning player...");

        Common::spawn(
            &mut commands,
            &mut meshes,
            &mut materials,
            &mut kill_counter,
            asset_server,
        );
    }
}
