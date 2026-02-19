#![allow(clippy::type_complexity)]

use crate::gun::{Gun, Muzzle, Ownable, bullet::Common};
use avian3d::prelude::*;
use bevy::prelude::*;
use spacerobo_commons::Bullet;

/// Select fire setting for Gun component
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum SelectFire {
    /// Semi auto
    #[default]
    Semi,

    /// Full auto
    Full,
}

/// Semi auto
pub fn semi_auto_system(
    mut commands: Commands,
    querys: (
        Query<&Gun>,
        Query<(&GlobalTransform, &LinearVelocity), With<Muzzle>>,
        Query<Entity, With<Ownable>>,
    ),
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mouse: Res<ButtonInput<MouseButton>>,
    asset_server: Res<AssetServer>,
) {
    // Unpacking querys
    let (gun_query, muzzle_query, owner_query) = querys;

    for gun in gun_query.iter() {
        if mouse.just_pressed(MouseButton::Left) && gun.select_fire == SelectFire::Semi {
            debug!("Mouse Left clicked");

            for e in owner_query.iter() {
                if gun.owner != e {
                    return;
                }
            }

            for (global_transform, muzzle_linear) in muzzle_query.iter() {
                // Shoot!!
                let bullet_origin: Vec3 = global_transform.translation();
                let direction: Vec3 = global_transform.rotation() * Vec3::NEG_Z;
                let bullet_force: Vec3 = direction * 500.0 + **muzzle_linear;
                debug!("bullet_force: {}", bullet_force);

                // ray_origin debugging by spawning a sphere
                Common::shoot(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    bullet_origin,
                    bullet_force,
                );

                Common::gunfire_sound(&mut commands, &asset_server, bullet_origin);
            }
        }
    }
}

/// Full auto
pub fn full_auto_system(
    mut commands: Commands,
    mut querys: (
        Query<&mut Gun, With<Gun>>,
        Query<(&GlobalTransform, &LinearVelocity), With<Muzzle>>,
    ),
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mouse: Res<ButtonInput<MouseButton>>,
    asset_server: Res<AssetServer>,
) {
    // Unpacking querys
    let (ref mut gun_query, muzzle_query) = querys;

    // Get muzzle's GlobalTransform
    for (global_transform, muzzle_linear) in muzzle_query.iter() {
        for mut gun in gun_query.iter_mut() {
            if mouse.pressed(MouseButton::Left) && gun.select_fire == SelectFire::Full {
                debug!("Mouse Left clicked");

                if gun.interval.rest >= 0. {
                    debug!("Full auto shoot aborted because of the gun's interval");
                    return;
                }

                // Full auto interval
                gun.interval.rest = gun.interval.limit;

                // Shoot!!
                let bullet_origin: Vec3 = global_transform.translation();
                let direction: Vec3 = global_transform.rotation() * Vec3::NEG_Z;
                let bullet_force: Vec3 = direction * 500.0 + **muzzle_linear;
                debug!("bullet_force: {}", bullet_force);

                // ray_origin debugging by spawning a sphere
                Common::shoot(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    bullet_origin,
                    bullet_force,
                );

                Common::gunfire_sound(&mut commands, &asset_server, bullet_origin);
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
