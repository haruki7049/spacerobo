use crate::{
    Hp,
    player::{
        Gun, Muzzle, Player, SelectFire,
        gun::{BULLET_SIZE, Bullet},
    },
};
use avian3d::prelude::*;
use bevy::prelude::*;

/// Semi auto
pub fn semi_auto_system(
    mut commands: Commands,
    querys: (
        Query<&ChildOf, With<Gun>>,
        Query<(&GlobalTransform, &ChildOf), With<Muzzle>>,
        Query<&LinearVelocity, With<Player>>,
    ),
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mouse: Res<ButtonInput<MouseButton>>,
    asset_server: Res<AssetServer>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        debug!("Mouse Left clicked");

        // Unpacking querys
        let (gun_query, muzzle_query, player_query) = querys;

        for childof in gun_query.iter() {
            // If the parent entity is not player, Do nothing and return
            if player_query.get(childof.parent()).is_err() {
                return;
            }

            for (global_transform, childof) in muzzle_query.iter() {
                // If the parent entity is not gun, Do nothing and return
                if gun_query.get(childof.parent()).is_err() {
                    return;
                }

                for player_linear in player_query.iter() {
                    let bullet_origin: Vec3 = global_transform.translation();

                    let direction: Vec3 = global_transform.rotation() * Vec3::NEG_Z;
                    let bullet_force: Vec3 = direction * 200.0 + **player_linear;
                    debug!("bullet_force: {}", bullet_force);

                    // ray_origin debugging by spawning a sphere
                    commands.spawn((
                        Transform::from_translation(bullet_origin),
                        Mesh3d(meshes.add(Sphere::new(BULLET_SIZE).mesh())),
                        MeshMaterial3d(materials.add(StandardMaterial {
                            base_color: Color::WHITE,
                            ..Default::default()
                        })),
                        RigidBody::Dynamic,
                        Collider::sphere(0.015625),
                        LinearVelocity(bullet_force),
                        Mass(3.0),
                        CollisionEventsEnabled,
                        Bullet,
                        Hp::ammo(),
                    ));

                    commands.spawn((
                        Transform::from_translation(global_transform.translation()),
                        AudioPlayer::new(asset_server.load("SE/shoot.ogg")),
                        PlaybackSettings::ONCE.with_spatial(false),
                    ));
                }
            }
        }
    }
}

/// Full auto
pub fn full_auto_system(
    mut commands: Commands,
    mut querys: (
        Query<(&mut Gun, &ChildOf), With<Gun>>,
        Query<(&GlobalTransform, &ChildOf), With<Muzzle>>,
        Query<&LinearVelocity, With<Player>>,
    ),
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mouse: Res<ButtonInput<MouseButton>>,
    asset_server: Res<AssetServer>,
) {
    if mouse.pressed(MouseButton::Left) {
        debug!("Mouse Left clicked");

        // Unpacking querys
        let (ref mut gun_query, muzzle_query, player_query) = querys;

        // Get muzzle's GlobalTransform
        for (global_transform, childof) in muzzle_query.iter() {
            // If the parent entity is not gun, Do nothing and return
            if gun_query.get(childof.parent()).is_err() {
                return;
            }

            for (mut gun, childof) in gun_query.iter_mut() {
                // If the parent entity is not player, Do nothing and return
                if player_query.get(childof.parent()).is_err() {
                    return;
                }

                if gun.interval.rest >= 0. {
                    debug!("Full auto shoot aborted because of the gun's interval");
                    return;
                }

                // Full auto interval
                gun.interval.rest = gun.interval.limit;

                for player_linear in player_query.iter() {
                    let bullet_origin: Vec3 = global_transform.translation();

                    let direction: Vec3 = global_transform.rotation() * Vec3::NEG_Z;
                    let bullet_force: Vec3 = direction * 200.0 + **player_linear;
                    debug!("bullet_force: {}", bullet_force);

                    // ray_origin debugging by spawning a sphere
                    commands.spawn((
                        Transform::from_translation(bullet_origin),
                        Mesh3d(meshes.add(Sphere::new(BULLET_SIZE).mesh())),
                        MeshMaterial3d(materials.add(StandardMaterial {
                            base_color: Color::WHITE,
                            ..Default::default()
                        })),
                        RigidBody::Dynamic,
                        Collider::sphere(0.015625),
                        LinearVelocity(bullet_force),
                        Mass(3.0),
                        CollisionEventsEnabled,
                        Bullet,
                        Hp::ammo(),
                    ));

                    commands.spawn((
                        Transform::from_translation(global_transform.translation()),
                        AudioPlayer::new(asset_server.load("SE/shoot.ogg")),
                        PlaybackSettings::ONCE.with_spatial(false),
                    ));
                }
            }
        }
    }
}

/// Toggle gun's select fire.
/// Full auto <---> Semi auto
pub fn toggle_select_fire_system(mut gun: Query<&mut Gun>, keyboard: Res<ButtonInput<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::KeyT) {
        let mut gun = gun.single_mut().unwrap();

        match gun.select_fire {
            SelectFire::Semi => gun.fullauto(),
            SelectFire::Full => gun.semiauto(),
        }
    }
}
