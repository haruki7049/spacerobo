use super::PlayerSpawnInfo;
use crate::Hp;
use avian3d::prelude::*;
use bevy::{color::palettes::basic::RED, prelude::*};
use bevy_octopus::prelude::*;

#[derive(Component)]
struct Opponent;

pub fn update_system(
    mut commands: Commands,
    mut channel_received: EventReader<ReceiveChannelMessage<PlayerSpawnInfo>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for event in channel_received.read() {
        commands.spawn((
            (event.message.transform),
            Mesh3d(meshes.add(Sphere::default().mesh())),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: RED.into(),
                ..Default::default()
            })),
            RigidBody::Dynamic,
            Collider::sphere(1.0),
            Mass(5.0),
            Hp::player(),
            Opponent,
        ));
    }
}
