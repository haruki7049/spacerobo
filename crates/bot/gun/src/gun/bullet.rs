use avian3d::prelude::*;
use bevy::prelude::*;
use spacerobo_commons::{Bullet, Hp};

const BULLET_SIZE: f32 = 1. / 8.;

/// A marker component for a bullet shot by a Gun
#[derive(Component)]
pub struct Common;

impl Bullet for Common {
    /// Shoots a bullet from origin (Vec3) with a force (Vec3).
    fn shoot(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        origin: Vec3,
        force: Vec3,
    ) {
        commands.spawn((
            Transform::from_translation(origin),
            Mesh3d(meshes.add(Sphere::new(BULLET_SIZE).mesh())),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::WHITE,
                ..Default::default()
            })),
            RigidBody::Dynamic,
            Collider::sphere(0.015625),
            LinearVelocity(force),
            Mass(3.0),
            CollisionEventsEnabled,
            Common,
            Hp::ammo(),
        ));
    }

    /// Play gunfire sound. Pow!!
    fn gunfire_sound(commands: &mut Commands, asset_server: &Res<AssetServer>, place: Vec3) {
        commands.spawn((
            Transform::from_translation(place),
            AudioPlayer::new(asset_server.load("SE/shoot.ogg")),
            PlaybackSettings::ONCE.with_spatial(false),
        ));
    }
}

#[cfg(test)]
mod tests {
    mod common_bullet {
        use super::super::Common;
        use avian3d::prelude::*;
        use bevy::prelude::*;
        use spacerobo_commons::Bullet;

        #[test]
        fn shoot_common_bullet() {
            // Create app with MinimalPlugins and AssetPlugin, I use MinimalPlugin because there hasn't to use DefaultPlugins (No GUI).
            // I also use two resources; `Assets<Mesh>` and `Assets<StandardMaterial>`.
            let mut app = App::new();
            app.add_plugins((MinimalPlugins, AssetPlugin::default()));
            app.init_resource::<Assets<Mesh>>();
            app.init_resource::<Assets<StandardMaterial>>();

            // Passes origin place and force vector to fat struct which is derived from bevy::prelude::Resource trait.
            let origin: Origin = Vec3::new(0.0, 0.0, 0.0).into();
            let force: Force = Vec3::new(0.0, 0.0, 0.0).into();

            // Insert them into the app.
            app.insert_resource(origin);
            app.insert_resource(force);
            app.add_systems(Startup, shoot_once);

            // Check the now status of app
            // It should be empty in app
            let world = app.world_mut();
            world
                .query_filtered::<(&Transform, &LinearVelocity, &AngularVelocity), With<Common>>()
                .is_empty(world, world.last_change_tick(), world.read_change_tick());

            // Update app
            app.update();

            // Check the now status of app
            // There should be in app only one
            let world = app.world_mut();
            let mut common_bullet_query = world
                .query_filtered::<(&Transform, &LinearVelocity, &AngularVelocity), With<Common>>();
            let common_bullet_info = common_bullet_query
                .single(world)
                .expect("Common bullet should be spawned at once");

            // Check the each parameters
            assert_eq!(common_bullet_info.0.translation, Vec3::ZERO);
            assert_eq!(**common_bullet_info.1, Vec3::ZERO);
            assert_eq!(**common_bullet_info.2, Vec3::ZERO);

            // Update app
            app.update();

            // Check the now status of app
            // There should be in app only one
            let world = app.world_mut();
            let mut common_bullet_query = world
                .query_filtered::<(&Transform, &LinearVelocity, &AngularVelocity), With<Common>>();
            let common_bullet_info = common_bullet_query
                .single(world)
                .expect("Common bullet should be spawned at once");

            // Check the each parameters
            assert_eq!(common_bullet_info.0.translation, Vec3::ZERO);
            assert_eq!(**common_bullet_info.1, Vec3::ZERO);
            assert_eq!(**common_bullet_info.2, Vec3::ZERO);
        }

        fn shoot_once(
            mut commands: Commands,
            mut meshes: ResMut<Assets<Mesh>>,
            mut materials: ResMut<Assets<StandardMaterial>>,
            origin: Res<Origin>,
            force: Res<Force>,
        ) {
            Common::shoot(
                &mut commands,
                &mut meshes,
                &mut materials,
                **origin,
                **force,
            );
        }

        #[derive(Deref, Resource)]
        struct Origin(Vec3);

        impl From<Vec3> for Origin {
            fn from(from: Vec3) -> Origin {
                Origin { 0: from }
            }
        }

        #[derive(Deref, Resource)]
        struct Force(Vec3);

        impl From<Vec3> for Force {
            fn from(from: Vec3) -> Force {
                Force { 0: from }
            }
        }
    }
}
