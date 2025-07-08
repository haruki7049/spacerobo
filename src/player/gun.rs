use bevy::prelude::*;
use avian3d::prelude::*;
use crate::target::Target;

#[derive(Component)]
pub struct Gun;

#[derive(Component)]
pub struct Muzzle;

#[derive(Component)]
pub struct Bullet;

pub fn gun_shoot_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    muzzle: Query<&GlobalTransform, With<Muzzle>>,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        debug!("Mouse Left clicked");

        let global_transform = muzzle.single().unwrap();
        let bullet_origin: Vec3 = global_transform.translation();

        let direction: Vec3 = global_transform.rotation() * Vec3::NEG_Z;
        let bullet_force: Vec3 = direction * 10000.0;
        debug!("bullet_force: {}", bullet_force);

        // ray_origin debugging by spawning a sphere
        commands.spawn((
            Transform::from_translation(bullet_origin),
            Mesh3d(meshes.add(Sphere::new(0.125).mesh())),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::WHITE,
                ..Default::default()
            })),
            RigidBody::Dynamic,
            Collider::sphere(0.125),
            LinearVelocity(bullet_force),
            CollisionEventsEnabled,
            Bullet,
        ));
    }
}

pub fn bullet_hit_detection_system(
    mut commands: Commands,
    mut collision_event_reader: EventReader<CollisionStarted>,
    targets: Query<Entity, With<Target>>,
) {
    for CollisionStarted(entity1, entity2) in collision_event_reader.read() {
        debug!("Collision!!");

        match targets.get(*entity1) {
            Ok(entity) => {
                commands.entity(entity).despawn();
                debug!("Despawned the target");
            }
            Err(e) => {
                debug!("Error by collision: {}", e);
            }
        }

        match targets.get(*entity2) {
            Ok(entity) => {
                commands.entity(entity).despawn();
                debug!("Despawned the target");
            }
            Err(e) => {
                debug!("Error by collision: {}", e);
            }
        }
    }
}
