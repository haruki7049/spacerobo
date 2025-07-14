use crate::{DeathEvent, player::Player};
use bevy::prelude::*;

pub fn health_system(
    mut commands: Commands,
    mut event_reader: EventReader<DeathEvent>,
    query: Query<&Player>,
) {
    for death_event in event_reader.read() {
        if query.get(death_event.entity).is_ok() {
            commands.entity(death_event.entity).despawn();
        }
    }
}
