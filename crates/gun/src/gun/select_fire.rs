#![allow(clippy::type_complexity)]

use crate::gun::{Gun, Muzzle, bullet::Common};
use avian3d::prelude::*;
use bevy::prelude::*;
use spacerobo_commons::{Bullet, configs::GameConfigs};

/// Select fire setting for Gun component
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum SelectFire {
    /// Semi auto
    #[default]
    Semi,

    /// Full auto
    Full,
}

/// Bullet travel speed, in world units per second, before the owner's own velocity is added.
const BULLET_FORCE: f32 = 500.0;

/// Bullet velocity: `direction` scaled by `BULLET_FORCE`, plus the owner's own velocity.
fn bullet_velocity(direction: Dir3, owner_velocity: Vec3) -> Vec3 {
    direction * BULLET_FORCE + owner_velocity
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
    let bullet_origin: Vec3 = muzzle_transform.translation();
    let direction: Dir3 = muzzle_transform.forward();
    let bullet_vector: Vec3 = bullet_velocity(direction, owner_velocity);

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
#[allow(clippy::too_many_arguments)]
pub fn semi_auto_system(
    mut commands: Commands,
    gun_query: Query<(&Gun, &ChildOf)>,
    muzzle_query: Query<&GlobalTransform, With<Muzzle>>,
    parent_linear_query: Query<&LinearVelocity>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mouse: Res<ButtonInput<MouseButton>>,
    asset_server: Res<AssetServer>,
) {
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
        Query<(&mut Gun, &ChildOf)>,
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
pub fn toggle_select_fire_system(
    mut gun: Query<&mut Gun>,
    keyboard: Res<ButtonInput<KeyCode>>,
    game_configs: Res<GameConfigs>,
) {
    if keyboard.just_pressed(game_configs.player.keyboard.toggle_firemode) {
        let Ok(mut gun) = gun.single_mut() else {
            return;
        };

        match gun.select_fire {
            SelectFire::Semi => gun.fullauto(),
            SelectFire::Full => gun.semiauto(),
        }
    }
}

#[cfg(test)]
mod tests {
    /// `bullet_velocity`'s unit tests
    mod bullet_velocity {
        use super::super::{BULLET_FORCE, bullet_velocity};
        use bevy::prelude::*;

        /// With no owner velocity, the bullet travels along its direction scaled by the force.
        #[test]
        fn zero_owner_velocity_scales_direction_by_force() {
            let result: Vec3 = bullet_velocity(Dir3::NEG_Z, Vec3::ZERO);
            assert_eq!(result, Vec3::NEG_Z * BULLET_FORCE);
        }

        /// The owner's velocity is added on top of the bullet's own travel vector.
        #[test]
        fn owner_velocity_is_added_to_travel_vector() {
            let owner_velocity: Vec3 = Vec3::new(1., 2., 3.);
            let result: Vec3 = bullet_velocity(Dir3::NEG_Z, owner_velocity);
            let expected: Vec3 = Vec3::NEG_Z * BULLET_FORCE + owner_velocity;
            assert_eq!(result, expected);
        }

        /// An owner velocity directly opposing the bullet's travel direction reduces its resultant speed.
        #[test]
        fn opposing_owner_velocity_reduces_resultant_speed() {
            let owner_velocity: Vec3 = Vec3::Z * BULLET_FORCE;
            let result: Vec3 = bullet_velocity(Dir3::NEG_Z, owner_velocity);
            assert_eq!(result, Vec3::ZERO);
        }
    }

    /// `toggle_select_fire_system`'s unit tests
    mod toggle_select_fire_system {
        use super::super::{Gun, SelectFire, toggle_select_fire_system};
        use crate::gun::Interval;
        use bevy::{ecs::system::RunSystemOnce, prelude::*};
        use spacerobo_commons::configs::GameConfigs;

        /// A world with the default-configured toggle key (`T`) pressed.
        fn world_with_key_t_pressed() -> World {
            world_with_key_pressed(KeyCode::KeyT, GameConfigs::default())
        }

        fn world_without_input() -> World {
            let mut world = World::new();
            world.insert_resource(ButtonInput::<KeyCode>::default());
            world.insert_resource(GameConfigs::default());
            world
        }

        fn world_with_key_pressed(key: KeyCode, game_configs: GameConfigs) -> World {
            let mut keyboard = ButtonInput::<KeyCode>::default();
            keyboard.press(key);

            let mut world = World::new();
            world.insert_resource(keyboard);
            world.insert_resource(game_configs);
            world
        }

        fn spawn_gun(world: &mut World, select_fire: SelectFire) {
            world.spawn(Gun {
                owner: Entity::PLACEHOLDER,
                select_fire,
                interval: Interval::default(),
            });
        }

        /// With exactly one gun, pressing T toggles its select-fire setting.
        #[test]
        fn toggles_the_single_guns_select_fire() {
            let mut world = world_with_key_t_pressed();
            spawn_gun(&mut world, SelectFire::Semi);

            world.run_system_once(toggle_select_fire_system).unwrap();

            let select_fire = world.query::<&Gun>().single(&world).unwrap().select_fire;
            assert!(matches!(select_fire, SelectFire::Full));
        }

        /// With no guns, the system returns early instead of panicking on `single_mut`.
        #[test]
        fn does_not_panic_with_no_guns() {
            let mut world = world_with_key_t_pressed();

            world.run_system_once(toggle_select_fire_system).unwrap();
        }

        /// With more than one gun, the system returns early instead of panicking on `single_mut`.
        #[test]
        fn does_not_panic_with_multiple_guns() {
            let mut world = world_with_key_t_pressed();
            spawn_gun(&mut world, SelectFire::Semi);
            spawn_gun(&mut world, SelectFire::Full);

            world.run_system_once(toggle_select_fire_system).unwrap();
        }

        /// Without the toggle key pressed, the gun's select-fire setting is left unchanged.
        #[test]
        fn leaves_select_fire_unchanged_without_input() {
            let mut world = world_without_input();
            spawn_gun(&mut world, SelectFire::Semi);

            world.run_system_once(toggle_select_fire_system).unwrap();

            let select_fire = world.query::<&Gun>().single(&world).unwrap().select_fire;
            assert!(matches!(select_fire, SelectFire::Semi));
        }

        /// Pressing the configured `toggle_firemode` key toggles select-fire, even when it has
        /// been rebound away from the default `T`.
        #[test]
        fn toggles_using_the_configured_key_even_when_rebound() {
            let mut game_configs = GameConfigs::default();
            game_configs.player.keyboard.toggle_firemode = KeyCode::KeyG;
            let mut world = world_with_key_pressed(KeyCode::KeyG, game_configs);
            spawn_gun(&mut world, SelectFire::Semi);

            world.run_system_once(toggle_select_fire_system).unwrap();

            let select_fire = world.query::<&Gun>().single(&world).unwrap().select_fire;
            assert!(matches!(select_fire, SelectFire::Full));
        }

        /// Once `toggle_firemode` is rebound, the old default key (`T`) no longer toggles it.
        #[test]
        fn does_not_toggle_using_the_old_default_key_once_rebound() {
            let mut game_configs = GameConfigs::default();
            game_configs.player.keyboard.toggle_firemode = KeyCode::KeyG;
            let mut world = world_with_key_pressed(KeyCode::KeyT, game_configs);
            spawn_gun(&mut world, SelectFire::Semi);

            world.run_system_once(toggle_select_fire_system).unwrap();

            let select_fire = world.query::<&Gun>().single(&world).unwrap().select_fire;
            assert!(matches!(select_fire, SelectFire::Semi));
        }
    }
}
