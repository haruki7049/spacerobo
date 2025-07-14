//! # System utils

use crate::Hp;
use avian3d::prelude::*;
use bevy::prelude::*;

/// This system detects the hits between a bullet and a target
pub fn collision_detection_system(
    mut commands: Commands,
    mut collision_event_reader: EventReader<CollisionStarted>,
    mut query: Query<(&Hp, &LinearVelocity, &Mass)>,
    asset_server: Res<AssetServer>,
) {
    for CollisionStarted(entity1, entity2) in collision_event_reader.read() {
        debug!("Collision!!");

        let object1 = query.get(*entity1);
        let object2 = query.get(*entity2);

        dbg!(object1);
        dbg!(object2);
        todo!()
    }
}
