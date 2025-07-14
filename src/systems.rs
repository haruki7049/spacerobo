//! # System utils

use bevy::prelude::*;
use avian3d::prelude::*;
use crate::{
    target::Target,
    player::gun::Bullet,
};

/// This system detects the hits between a bullet and a target
pub fn bullet_hit_detection_system(
    mut commands: Commands,
    mut collision_event_reader: EventReader<CollisionStarted>,
    query: (Query<Entity, With<Target>>, Query<Entity, With<Bullet>>),
    asset_server: Res<AssetServer>,
) {
    for CollisionStarted(entity1, entity2) in collision_event_reader.read() {
        debug!("Collision!!");

        let (targets, bullets) = query;

        if targets.contains(*entity1) && targets.contains(*entity2) {
            return;
        }

        if bullets.contains(*entity1) && bullets.contains(*entity2) {
            commands.entity(*entity1).despawn();
            commands.entity(*entity2).despawn();
        }

        if targets.contains(*entity1) && bullets.contains(*entity2) {
            commands.spawn((
                AudioPlayer::new(asset_server.load("SE/kill.ogg")),
                PlaybackSettings::ONCE.with_spatial(false),
            ));

            commands.entity(*entity1).despawn();
            commands.entity(*entity2).despawn();
        }

        if bullets.contains(*entity1) && targets.contains(*entity2) {
            commands.spawn((
                AudioPlayer::new(asset_server.load("SE/kill.ogg")),
                PlaybackSettings::ONCE.with_spatial(false),
            ));

            commands.entity(*entity1).despawn();
            commands.entity(*entity2).despawn();
        }
    }
}
