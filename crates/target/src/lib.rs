//! # Target systems, Compoments & etc...

use avian3d::prelude::*;
use bevy::prelude::*;
use spacerobo_commons::{GameMode, Hp, Target};

/// Target Component
#[derive(Component)]
pub struct Common;

impl Target for Common {
    fn spawn(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        asset_server: &Res<AssetServer>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        base_color: Color,
        position: Vec3,
    ) {
        commands.spawn((
            DespawnOnExit(GameMode::InGame),
            Mesh3d(meshes.add(Sphere::default().mesh())),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color,
                ..Default::default()
            })),
            Transform::from_translation(position),
            RigidBody::Static,
            Collider::sphere(1.0),
            CollisionEventsEnabled,
            Mass(1.0),
            Self,
            Hp::robo(Some(asset_server.load("SE/kill.ogg"))),
        ));
    }
}
