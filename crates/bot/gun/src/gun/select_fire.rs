#![allow(clippy::type_complexity)]

use crate::gun::{Gun, Muzzle, bullet::Common};
use avian3d::prelude::*;
use bevy::prelude::*;
use spacerobo_commons::Bullet;

/// Full auto forever system
pub fn full_auto_forever_system(
    mut commands: Commands,
    mut querys: (
        Query<&mut Gun, With<Gun>>,
        Query<(&GlobalTransform, &LinearVelocity), With<Muzzle>>,
    ),
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Unpacking querys
    let (ref mut gun_query, muzzle_query) = querys;

    // Get muzzle's GlobalTransform
    for (global_transform, muzzle_linear) in muzzle_query.iter() {
        for mut gun in gun_query.iter_mut() {
            // If the parent entity is not bot, Do nothing and return
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
