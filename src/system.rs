//! # System utils

use crate::{DeathEvent, Hp, player::Player, target::Target};
use avian3d::prelude::*;
use bevy::{platform::thread::sleep, prelude::*};
use std::time::Duration;

/// This system detects the hits between two objects, having Hp, LinearVelocity and Mass Components.
/// This system is created to decrease the hp at contacted objects.
pub fn collision_detection_system(
    mut collision_event_reader: EventReader<CollisionStarted>,
    mut query: Query<(&mut Hp, &LinearVelocity, &Mass)>,
    mut event_writer: EventWriter<DeathEvent>,
) {
    for CollisionStarted(entity1, entity2) in collision_event_reader.read() {
        debug!("Collision!!");

        let objects = query.get_many_mut([*entity1, *entity2]).ok();

        match objects {
            Some([mut obj1, mut obj2]) => {
                let obj1_damage: f32 = calc_damage(&obj1);
                let obj2_damage: f32 = calc_damage(&obj2);
                let damage: f32 = obj1_damage + obj2_damage;

                let (ref mut obj1_hp, _obj1_linear, _obj1_mass) = obj1;
                let (ref mut obj2_hp, _obj2_linear, _obj2_mass) = obj2;

                obj1_hp.rest -= damage;
                obj2_hp.rest -= damage;

                debug!("The first object's Hp: {:?}", &obj1_hp);
                debug!("The second object's Hp: {:?}", &obj2_hp);

                if obj1_hp.rest <= 0. {
                    event_writer.write(DeathEvent::new(*entity1));
                }
                if obj2_hp.rest <= 0. {
                    event_writer.write(DeathEvent::new(*entity2));
                }
            }
            _ => debug!(
                "The collisioned entity, {} or {} is missing Hp, LinearVelocity or Mass",
                entity1, entity2
            ),
        }
    }
}

pub fn gameover_system(
    players: Query<&Player>,
    targets: Query<&Target>,
    mut exit: EventWriter<AppExit>,
) {
    if targets.is_empty() {
        info!("No targets detected. GAMECLEAR!!");

        let duration: Duration = Duration::from_secs(3);
        sleep(duration);
        exit.write(AppExit::Success);
    }

    if players.is_empty() {
        info!("No targets detected. GAMEOVER...");
        exit.write(AppExit::Success);
    }
}

fn calc_damage(object: &(Mut<'_, Hp>, &LinearVelocity, &Mass)) -> f32 {
    let (_hp, linear, mass) = object;

    let speed: f32 = linear.x + linear.y + linear.z;

    // Speed * Mass = Force
    // By Isaac Newton
    // Probably...
    (speed * ***mass).abs()
}
