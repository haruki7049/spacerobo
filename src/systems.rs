//! # System utils

use crate::Hp;
use avian3d::prelude::*;
use bevy::prelude::*;

/// This system detects the hits between a bullet and a target
pub fn collision_detection_system(
    mut commands: Commands,
    mut collision_event_reader: EventReader<CollisionStarted>,
    mut query: Query<(&mut Hp, &LinearVelocity, &Mass)>,
    asset_server: Res<AssetServer>,
) {
    for CollisionStarted(entity1, entity2) in collision_event_reader.read() {
        debug!("Collision!!");

        let mut object1 = query.get_mut(*entity1).ok();
        let mut object2 = query.get_mut(*entity2).ok();

        match (object1, object2) {
            (Some(obj1), Some(obj2)) => {
                let obj1_damage: f32 = calc_damage(obj1);
                let obj2_damage: f32 = calc_damage(obj2);

                dbg!(&obj1_damage);
                dbg!(&obj2_damage);

                let damage: f32 = obj1_damage + obj2_damage;

                todo!()
            }
            (None, Some(obj)) | (Some(obj), None) => {
                let damage: f32 = calc_damage(obj);

                dbg!(damage);
                todo!();
            }
            (None, None) => (),
        }
    }
}

fn calc_damage(object: (Mut<'_, Hp>, &LinearVelocity, &Mass)) -> f32 {
    let (hp, linear, mass) = object;

    let speed: f32 = linear.x + linear.y + linear.z;

    // Speed * Mass = Force
    // By Isaac Newton
    // Probably...
    (speed * **mass).abs()
}
