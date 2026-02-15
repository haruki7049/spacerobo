use avian3d::prelude::*;
use bevy::prelude::*;
use spacerobo_commons::Hp;

const BULLET_SIZE: f32 = 1. / 8.;

/// A marker component for a bullet shot by a Gun
#[derive(Component)]
pub struct Bullet;

impl Bullet {
    pub fn shoot(
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
            Bullet,
            Hp::ammo(),
        ));
    }

    pub fn gunfire(commands: &mut Commands, asset_server: &Res<AssetServer>, place: Vec3) {
        commands.spawn((
            Transform::from_translation(place),
            AudioPlayer::new(asset_server.load("SE/shoot.ogg")),
            PlaybackSettings::ONCE.with_spatial(false),
        ));
    }
}
