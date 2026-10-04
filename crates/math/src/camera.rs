//! Mouse-driven camera rotation.

use bevy_math::{Quat, Vec2, Vec3};

/// Angular velocity added by a mouse-driven camera rotation.
///
/// `mouse_delta` is the raw accumulated mouse motion for the frame. `x_axis`/`y_axis` are the
/// local axes that the mouse's horizontal/vertical motion rotates around, each flipped when the
/// matching reverse flag is set, rotated into world space by `rotation`, then scaled by the
/// matching `x_force`/`y_force` thruster force.
#[allow(clippy::too_many_arguments)]
pub fn rotate_camera(
    mouse_delta: Vec2,
    rotation: Quat,
    x_axis: Vec3,
    y_axis: Vec3,
    x_reverse: bool,
    y_reverse: bool,
    x_force: f32,
    y_force: f32,
) -> Vec3 {
    if mouse_delta == Vec2::ZERO {
        return Vec3::ZERO;
    }

    let mouse: Vec2 = Vec2::new(-mouse_delta.x / 100., -mouse_delta.y / 100.);

    let x_direction: Vec3 = if x_reverse { -x_axis } else { x_axis };
    let y_direction: Vec3 = if y_reverse { -y_axis } else { y_axis };

    mouse.x * x_force * (rotation * x_direction) + mouse.y * y_force * (rotation * y_direction)
}

#[cfg(test)]
mod tests {
    use super::rotate_camera;
    use bevy_math::{Quat, Vec2, Vec3};
    use std::f32::consts::FRAC_PI_2;

    /// No mouse movement yields no rotation, regardless of the other parameters.
    #[test]
    fn zero_delta_yields_zero_vector() {
        let result: Vec3 = rotate_camera(
            Vec2::ZERO,
            Quat::IDENTITY,
            Vec3::Z,
            Vec3::X,
            false,
            false,
            1.,
            1.,
        );
        assert_eq!(result, Vec3::ZERO);
    }

    /// With no rotation and no reverse flags, each mouse axis scales its own world axis by its force.
    #[test]
    fn identity_rotation_scales_axes_by_force() {
        let result: Vec3 = rotate_camera(
            Vec2::new(100., -100.),
            Quat::IDENTITY,
            Vec3::Z,
            Vec3::X,
            false,
            false,
            2.,
            3.,
        );

        // mouse = (-delta.x / 100, -delta.y / 100) = (-1, 1)
        let expected: Vec3 = -2. * Vec3::Z + 3. * Vec3::X;
        assert_eq!(result, expected);
    }

    /// The reverse flags flip the axis each mouse component rotates around.
    #[test]
    fn reverse_flags_flip_axes() {
        let result: Vec3 = rotate_camera(
            Vec2::new(100., -100.),
            Quat::IDENTITY,
            Vec3::Z,
            Vec3::X,
            true,
            true,
            2.,
            3.,
        );

        let expected: Vec3 = -2. * -Vec3::Z + 3. * -Vec3::X;
        assert_eq!(result, expected);
    }

    /// Axes are rotated into world space before being scaled by force.
    #[test]
    fn rotation_rotates_axes_before_scaling() {
        // A 90-degree rotation around Y maps Z to X.
        let rotation: Quat = Quat::from_rotation_y(FRAC_PI_2);
        let result: Vec3 = rotate_camera(
            Vec2::new(100., 0.),
            rotation,
            Vec3::Z,
            Vec3::X,
            false,
            false,
            2.,
            3.,
        );

        let expected: Vec3 = Vec3::new(-2., 0., 0.);
        assert!(
            (result - expected).length() < 1e-4,
            "expected {expected:?}, got {result:?}"
        );
    }
}
