use bevy::prelude::*;

#[derive(Debug, States, Default, Hash, Eq, PartialEq, Clone)]
#[states(scoped_entities)]
pub enum GameMode {
    #[default]
    Title,
    ShootingRange,
    VersusMaster,
    VersusGuest,
}
