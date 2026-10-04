//! Thrust and projectile velocity composition.

use bevy_math::{Dir3, Quat, Vec3};

/// Thrust applied along a local `direction`, rotated into world space by `rotation` and scaled by `force`.
pub fn apply_thrust(rotation: Quat, direction: Vec3, force: f32) -> Vec3 {
    force * (rotation * direction)
}

/// Bullet velocity: `direction` scaled by `force`, plus the owner's own velocity.
pub fn bullet_velocity(direction: Dir3, force: f32, owner_velocity: Vec3) -> Vec3 {
    direction * force + owner_velocity
}

#[cfg(test)]
mod tests {
    /// `apply_thrust`'s unit tests
    mod apply_thrust {
        use super::super::apply_thrust;
        use bevy_math::{Quat, Vec3};
        use std::f32::consts::FRAC_PI_2;

        /// With no rotation, thrust is the direction scaled by force.
        #[test]
        fn identity_rotation_scales_direction() {
            let result: Vec3 = apply_thrust(Quat::IDENTITY, Vec3::NEG_Z, 0.7);
            assert_eq!(result, Vec3::new(0., 0., -0.7));
        }

        /// Zero force yields a zero vector regardless of rotation or direction.
        #[test]
        fn zero_force_yields_zero_vector() {
            let rotation: Quat = Quat::from_rotation_y(FRAC_PI_2);
            let result: Vec3 = apply_thrust(rotation, Vec3::X, 0.);
            assert_eq!(result, Vec3::ZERO);
        }

        /// The direction is rotated into world space before being scaled by force.
        #[test]
        fn rotation_rotates_direction_before_scaling() {
            // A 90-degree rotation around Y maps NEG_Z to NEG_X.
            let rotation: Quat = Quat::from_rotation_y(FRAC_PI_2);
            let result: Vec3 = apply_thrust(rotation, Vec3::NEG_Z, 2.0);
            let expected: Vec3 = Vec3::new(-2., 0., 0.);

            assert!(
                (result - expected).length() < 1e-4,
                "expected {expected:?}, got {result:?}"
            );
        }
    }

    /// `bullet_velocity`'s unit tests
    mod bullet_velocity {
        use super::super::bullet_velocity;
        use bevy_math::{Dir3, Vec3};

        const BULLET_FORCE: f32 = 500.0;

        /// With no owner velocity, the bullet travels along its direction scaled by the force.
        #[test]
        fn zero_owner_velocity_scales_direction_by_force() {
            let result: Vec3 = bullet_velocity(Dir3::NEG_Z, BULLET_FORCE, Vec3::ZERO);
            assert_eq!(result, Vec3::NEG_Z * BULLET_FORCE);
        }

        /// The owner's velocity is added on top of the bullet's own travel vector.
        #[test]
        fn owner_velocity_is_added_to_travel_vector() {
            let owner_velocity: Vec3 = Vec3::new(1., 2., 3.);
            let result: Vec3 = bullet_velocity(Dir3::NEG_Z, BULLET_FORCE, owner_velocity);
            let expected: Vec3 = Vec3::NEG_Z * BULLET_FORCE + owner_velocity;
            assert_eq!(result, expected);
        }

        /// An owner velocity directly opposing the bullet's travel direction reduces its resultant speed.
        #[test]
        fn opposing_owner_velocity_reduces_resultant_speed() {
            let owner_velocity: Vec3 = Vec3::Z * BULLET_FORCE;
            let result: Vec3 = bullet_velocity(Dir3::NEG_Z, BULLET_FORCE, owner_velocity);
            assert_eq!(result, Vec3::ZERO);
        }
    }
}
