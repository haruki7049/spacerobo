//! Shared vector math used by the robo's flight controls and the gun.
//!
//! Every function here is pure (`Vec3`/`Quat`/`Vec2`/`f32` in, `Vec3` out) and depends only on
//! `bevy_math`, not on `bevy`'s ECS or `avian3d`, so it can be edited and tested without
//! recompiling `spacerobo_commons` or `spacerobo_gun`.

mod camera;
mod thrust;

pub use camera::rotate_camera;
pub use thrust::{apply_thrust, bullet_velocity};
