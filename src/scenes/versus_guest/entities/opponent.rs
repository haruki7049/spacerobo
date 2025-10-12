//! # Opponent systems, Compoments & etc...

pub mod bullet;
pub mod health;

use crate::{Damage, GameMode, Information, OpponentResource};
use avian3d::prelude::*;
use bevy::prelude::*;
//use gun::{Gun, Interval, Muzzle, select_fire::SelectFire};

/// Opponent Component
#[derive(Component)]
pub struct Opponent;

#[derive(Component)]
pub struct DamageCollector(pub Vec<Damage>);

/// update system to manage opponent entity
pub fn update_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    opponent_resource: Res<OpponentResource>,
    opponent_query: Query<Entity, With<Opponent>>,
) {
    let Some(info): Option<Information> = opponent_resource.get() else {
        return;
    };

    if !opponent_query.is_empty() {
        for opponent in opponent_query.iter() {
            commands.entity(opponent).despawn();
        }
    }

    // Camera
    commands
        .spawn((
            StateScoped(GameMode::VersusMaster),
            Mesh3d(meshes.add(Sphere::new(1.).mesh())),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::WHITE,
                ..Default::default()
            })),
            info.player.transform,
            RigidBody::Kinematic,
            GravityScale(0.2),
            Collider::sphere(1.0),
            Mass(5.0),
            info.player.angular,
            info.player.linear,
            Opponent,
            DamageCollector(info.player.damages),
            CollisionEventsEnabled,
        ))
        // Gun
        .with_children(|parent| {
            parent
                .spawn((
                    Transform::from_xyz(1., -1., -3.),
                    Mesh3d(meshes.add(Extrusion::new(Circle::new(0.125), 2.))),
                    MeshMaterial3d(materials.add(Color::WHITE)),
                    //(Gun {
                    //    select_fire: SelectFire::Full,
                    //    interval: Interval {
                    //        limit: 0.1,
                    //        rest: 0.0,
                    //        amount: 0.01,
                    //    },
                    //}),
                ))
                // Spot light
                .with_child((
                    SpotLight {
                        intensity: 100_000_000.0,
                        range: 100_000_000.0,
                        outer_angle: std::f32::consts::FRAC_PI_4 / 2.0,
                        shadows_enabled: true,
                        ..default()
                    },
                    Transform::from_xyz(1., -1., -4.3).looking_to(Vec3::NEG_Z, Vec3::ZERO),
                ));
            //// Muzzle
            //.with_child((
            //    Transform::from_xyz(1., -1., -4.3),
            //    Muzzle,
            //    RigidBody::Static,
            //));
        });
}
