use super::{Target, entities::bot::Bot};
use crate::{DeathEvent, KillCounter};
use bevy::prelude::*;

pub fn update_system(
    mut commands: Commands,
    mut death_reader: EventReader<DeathEvent>,
    query: Query<Entity, Or<(With<Target>, With<Bot>)>>,
    mut kill_counter: ResMut<KillCounter>,
    asset_server: Res<AssetServer>,
) {
    for death_event in death_reader.read() {
        if query.get(death_event.entity).is_ok() {
            kill_counter.increment();

            commands.spawn((
                AudioPlayer::new(asset_server.load("SE/kill.ogg")),
                PlaybackSettings::ONCE.with_spatial(false),
            ));

            // Despawn the target
            commands.entity(death_event.entity).despawn();
        }
    }
}
