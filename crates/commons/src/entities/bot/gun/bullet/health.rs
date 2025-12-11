use super::super::bullet::Bullet;
use crate::DeathEvent;
use bevy::prelude::*;

pub fn update_system(
    mut commands: Commands,
    mut death_reader: EventReader<DeathEvent>,
    query: Query<&Bullet>,
) {
    for death_event in death_reader.read() {
        if query.get(**death_event).is_ok() {
            // Despawn the bullet
            commands.entity(**death_event).despawn();
        }
    }
}
