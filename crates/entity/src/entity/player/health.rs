use super::Player;
use bevy::prelude::*;
use spacerobo_commons::DeathEvent;

pub fn update_system(
    mut commands: Commands,
    mut event_reader: EventReader<DeathEvent>,
    asset_server: Res<AssetServer>,
    query: Query<&Player>,
) {
    for death_event in event_reader.read() {
        if query.get(**death_event).is_ok() {
            commands.entity(**death_event).despawn();
            commands.spawn(AudioPlayer::new(asset_server.load("SE/kill.ogg")));

            debug!("Player is dead!!");
        }
    }
}
