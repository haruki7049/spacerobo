use avian3d::prelude::*;
use bevy::prelude::*;
use spacerobo_commons::{GameMode, Hp, Information, OpponentResource};

/// Bullet Component
#[derive(Component)]
pub struct Bullet;

/// update system to manage opponent's bullet
pub fn update_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    opponent_resource: Res<OpponentResource>,
    bullet_query: Query<Entity, With<Bullet>>,
) {
    let Some(info): Option<Information> = opponent_resource.get() else {
        return;
    };

    if !bullet_query.is_empty() {
        for bullet in bullet_query.iter() {
            commands.entity(bullet).despawn();
        }
    }

    // Bullet
    for bullet in info.bullets {
        commands.spawn((
            StateScoped(GameMode::VersusGuest),
            Mesh3d(meshes.add(Sphere::new(1.0 / 8.0).mesh())),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::WHITE,
                ..Default::default()
            })),
            bullet.transform,
            RigidBody::Kinematic,
            GravityScale(0.2),
            Collider::sphere(0.015625),
            Mass(3.0),
            bullet.angular,
            bullet.linear,
            Hp::ammo(),
            Bullet,
            CollisionEventsEnabled,
        ));
    }
}
