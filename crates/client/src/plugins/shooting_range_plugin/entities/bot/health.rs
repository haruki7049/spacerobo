use super::Bot;
use crate::plugins::commons::DeathEvent;
use bevy::prelude::*;

pub fn update_system(
    mut commands: Commands,
    mut event_reader: EventReader<DeathEvent>,
    query: Query<&Bot>,
) {
    for death_event in event_reader.read() {
        if query.get(**death_event).is_ok() {
            commands.entity(**death_event).despawn();

            debug!("Bot is dead!!");
        }
    }
}
