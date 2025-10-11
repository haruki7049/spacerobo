use super::Opponent;
use crate::DeathEvent;
use bevy::prelude::*;

pub fn update_system(
    mut commands: Commands,
    mut event_reader: EventReader<DeathEvent>,
    query: Query<&Opponent>,
) {
    for death_event in event_reader.read() {
        if query.get(death_event.entity).is_ok() {
            commands.entity(death_event.entity).despawn();
        }
    }
}
