use super::Target;
use bevy::prelude::*;
use spacerobo_commons::DeathEvent;

pub fn update_system(
    mut commands: Commands,
    mut event_reader: EventReader<DeathEvent>,
    query: Query<&Target>,
) {
    for death_event in event_reader.read() {
        if query.get(**death_event).is_ok() {
            commands.entity(**death_event).despawn();

            debug!("Target is dead!!");
        }
    }
}
