use super::Controllable;
use crate::{GameMode, configs::GameConfigs};
use avian3d::prelude::*;
use bevy::prelude::*;
use spacerobo_math::apply_thrust;

pub fn update_system(
    mut gamemode: ResMut<NextState<GameMode>>,
    mut query: Query<(&Transform, &mut AngularVelocity, &mut LinearVelocity), With<Controllable>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    game_configs: Res<GameConfigs>,
) {
    for (transform, mut angular, mut linear) in query.iter_mut() {
        // Hovering
        if keyboard.pressed(game_configs.player.keyboard.hover) {
            let damping: Vec3 = Vec3::splat(game_configs.player.robo.hover_damping);
            angular.0 *= damping;
            linear.0 *= damping;
        }

        // Exit spacerobo
        if keyboard.just_pressed(game_configs.player.keyboard.quit) {
            gamemode.set(GameMode::Title);
        }

        let rotation: Quat = transform.rotation;

        // Accelerate
        {
            let force: f32 = game_configs.player.robo.thruster.force.accelerate;

            if keyboard.pressed(game_configs.player.keyboard.forward) {
                linear.0 += apply_thrust(rotation, Vec3::NEG_Z, force);
            }

            if keyboard.pressed(game_configs.player.keyboard.left) {
                linear.0 += apply_thrust(rotation, Vec3::NEG_X, force);
            }

            if keyboard.pressed(game_configs.player.keyboard.back) {
                linear.0 += apply_thrust(rotation, Vec3::Z, force);
            }

            if keyboard.pressed(game_configs.player.keyboard.right) {
                linear.0 += apply_thrust(rotation, Vec3::X, force);
            }
        }

        // Dash
        if keyboard.pressed(game_configs.player.keyboard.dash) {
            let force: f32 = game_configs.player.robo.thruster.force.dash;

            if keyboard.pressed(game_configs.player.keyboard.forward) {
                linear.0 += apply_thrust(rotation, Vec3::NEG_Z, force);
            }

            if keyboard.pressed(game_configs.player.keyboard.left) {
                linear.0 += apply_thrust(rotation, Vec3::NEG_X, force);
            }

            if keyboard.pressed(game_configs.player.keyboard.back) {
                linear.0 += apply_thrust(rotation, Vec3::Z, force);
            }

            if keyboard.pressed(game_configs.player.keyboard.right) {
                linear.0 += apply_thrust(rotation, Vec3::X, force);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    /// `update_system`'s unit tests
    mod update_system {
        use super::super::update_system;
        use crate::{Controllable, GameMode, configs::GameConfigs};
        use avian3d::prelude::*;
        use bevy::{ecs::system::RunSystemOnce, prelude::*};

        fn world_with_hover_pressed(hover_damping: f32) -> World {
            let mut game_configs = GameConfigs::default();
            game_configs.player.robo.hover_damping = hover_damping;

            let mut keyboard = ButtonInput::<KeyCode>::default();
            keyboard.press(game_configs.player.keyboard.hover);

            let mut world = World::new();
            world.insert_resource(keyboard);
            world.insert_resource(game_configs);
            world.insert_resource(NextState::<GameMode>::default());
            world
        }

        /// Holding the hover key scales linear and angular velocity by the configured damping,
        /// not a hardcoded value.
        #[test]
        fn hover_damping_comes_from_game_configs() {
            let mut world = world_with_hover_pressed(0.5);
            let entity = world
                .spawn((
                    Transform::default(),
                    AngularVelocity(Vec3::splat(2.0)),
                    LinearVelocity(Vec3::splat(4.0)),
                    Controllable,
                ))
                .id();

            world.run_system_once(update_system).unwrap();

            let angular = world.get::<AngularVelocity>(entity).unwrap();
            let linear = world.get::<LinearVelocity>(entity).unwrap();
            assert_eq!(angular.0, Vec3::splat(1.0));
            assert_eq!(linear.0, Vec3::splat(2.0));
        }
    }
}
