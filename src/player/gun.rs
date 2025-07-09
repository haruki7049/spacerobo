use crate::target::Target;
use avian3d::prelude::*;
use bevy::prelude::*;

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
        let bullet_force: Vec3 = direction * 200.0;
        debug!("bullet_force: {}", bullet_force);

        // ray_origin debugging by spawning a sphere
        commands.spawn((
            Transform::from_translation(bullet_origin),
            Mesh3d(meshes.add(Sphere::new(0.015625).mesh())),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::WHITE,
                ..Default::default()
            })),
            RigidBody::Dynamic,
            Collider::sphere(0.015625),
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
    bullets: Query<Entity, With<Bullet>>,
) {
    for CollisionStarted(entity1, entity2) in collision_event_reader.read() {
        debug!("Collision!!");

        if targets.contains(*entity1) && targets.contains(*entity2) {
            return;
        }

        if bullets.contains(*entity1) && bullets.contains(*entity2) {
            commands.entity(*entity1).despawn();
            commands.entity(*entity2).despawn();
        }

        if targets.contains(*entity1) && bullets.contains(*entity2) {
            commands.entity(*entity1).despawn();
            commands.entity(*entity2).despawn();
        }

        if bullets.contains(*entity1) && targets.contains(*entity2) {
            commands.entity(*entity1).despawn();
            commands.entity(*entity2).despawn();
        }
    }
}
