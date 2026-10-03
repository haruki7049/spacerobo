#![allow(clippy::type_complexity)]

use crate::gun::{Gun, Muzzle, bullet::Common};
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

/// Spawn a bullet from one muzzle, combining its travel speed with the owner's velocity.
fn fire(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    asset_server: &Res<AssetServer>,
    muzzle_transform: &GlobalTransform,
    owner_velocity: Vec3,
    owner: Entity,
) {
    const BULLET_FORCE: f32 = 500.0;

    let bullet_origin: Vec3 = muzzle_transform.translation();
    let direction: Dir3 = muzzle_transform.forward();
    let bullet_vector: Vec3 = direction * BULLET_FORCE + owner_velocity;

    Common::shoot(
        commands,
        meshes,
        materials,
        bullet_origin,
        bullet_vector,
        owner,
    );
    Common::gunfire_sound(commands, asset_server, bullet_origin);
}

/// Semi auto
pub fn semi_auto_system(
    mut commands: Commands,
    mut querys: (
        Query<(&Gun, &ChildOf), With<Gun>>,
        Query<&GlobalTransform, With<Muzzle>>,
        Query<&LinearVelocity>,
    ),
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mouse: Res<ButtonInput<MouseButton>>,
    asset_server: Res<AssetServer>,
) {
    // Unpacking querys
    let (ref mut gun_query, muzzle_query, parent_linear_query) = querys;

    for (gun, child_of) in gun_query.iter() {
        if !(mouse.just_pressed(MouseButton::Left) && gun.select_fire == SelectFire::Semi) {
            continue;
        }

        debug!("Mouse Left clicked");

        if child_of.parent() != gun.owner {
            debug!("Semi auto shooting is abandoned");
            continue;
        }

        let Ok(player_linear_velocity) = parent_linear_query.get(child_of.parent()) else {
            continue;
        };

        for global_transform in muzzle_query.iter() {
            fire(
                &mut commands,
                &mut meshes,
                &mut materials,
                &asset_server,
                global_transform,
                **player_linear_velocity,
                gun.owner,
            );
        }
    }
}

/// Full auto
pub fn full_auto_system(
    mut commands: Commands,
    mut querys: (
        Query<(&mut Gun, &ChildOf), With<Gun>>,
        Query<&GlobalTransform, With<Muzzle>>,
        Query<&LinearVelocity>,
    ),
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mouse: Res<ButtonInput<MouseButton>>,
    asset_server: Res<AssetServer>,
) {
    // Unpacking querys
    let (ref mut gun_query, muzzle_query, parent_linear_query) = querys;

    // Get muzzle's GlobalTransform
    for global_transform in muzzle_query.iter() {
        for (mut gun, child_of) in gun_query.iter_mut() {
            let Ok(player_linear_velocity) = parent_linear_query.get(child_of.parent()) else {
                continue;
            };

            if !(mouse.pressed(MouseButton::Left) && gun.select_fire == SelectFire::Full) {
                continue;
            }

            debug!("Mouse Left clicked");

            if child_of.parent() != gun.owner {
                debug!("Full auto shooting is abandoned");
                continue;
            }

            if gun.interval.rest >= 0. {
                debug!("Full auto shoot aborted because of the gun's interval");
                continue;
            }

            // Full auto interval
            gun.interval.rest = gun.interval.limit;

            fire(
                &mut commands,
                &mut meshes,
                &mut materials,
                &asset_server,
                global_transform,
                **player_linear_velocity,
                gun.owner,
            );
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
