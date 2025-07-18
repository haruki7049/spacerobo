//! # Target Component

use crate::DeathEvent;
use bevy::prelude::*;

#[derive(Component)]
pub struct Target;

pub fn health_system(
    mut commands: Commands,
    mut death_reader: EventReader<DeathEvent>,
    query: Query<&Target>,
    asset_server: Res<AssetServer>,
) {
    for death_event in death_reader.read() {
        if query.get(death_event.entity).is_ok() {
            commands.spawn((
                AudioPlayer::new(asset_server.load("SE/kill.ogg")),
                PlaybackSettings::ONCE.with_spatial(false),
            ));

            // Despawn the target
            commands.entity(death_event.entity).despawn();
        }
    }
}
