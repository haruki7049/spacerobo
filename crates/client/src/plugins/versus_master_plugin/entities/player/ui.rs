//! # UI systems, components & etc...

use super::Player;
use crate::plugins::commons::{GameMode, Hp, KillCounter};
use bevy::prelude::*;

#[derive(Component)]
pub struct HeadingIndicator;

#[derive(Component)]
pub struct CoordinatesIndicator;

#[derive(Component)]
pub struct KillCounterUI;

#[derive(Component)]
pub struct HpUI;

pub fn setup_system(mut commands: Commands) {
    // Heading Indicator
    commands
        .spawn((Text::default(), StateScoped(GameMode::VersusMaster)))
        .with_child((
            TextSpan::default(),
            (TextFont {
                font_size: 21.0,
                ..default()
            }),
            HeadingIndicator,
        ))
        .with_child((
            TextSpan::default(),
            (TextFont {
                font_size: 21.0,
                ..default()
            }),
            CoordinatesIndicator,
        ))
        .with_child((
            TextSpan::new("\n"),
            (TextFont {
                font_size: 21.0,
                ..default()
            }),
        ))
        .with_child((
            TextSpan::default(),
            (TextFont {
                font_size: 21.0,
                ..default()
            }),
            HpUI,
        ))
        .with_child((
            TextSpan::default(),
            (TextFont {
                font_size: 21.0,
                ..default()
            }),
            KillCounterUI,
        ));
}

pub fn update_system(
    mut spans: ParamSet<(
        Query<&mut TextSpan, With<HeadingIndicator>>,
        Query<&mut TextSpan, With<CoordinatesIndicator>>,
        Query<&mut TextSpan, With<HpUI>>,
        Query<&mut TextSpan, With<KillCounterUI>>,
    )>,
    player_query: Query<(&Transform, &Hp), With<Player>>,
    kill_counter: Res<KillCounter>,
) {
    for (transform, hp) in player_query.iter() {
        for mut span in &mut spans.p0() {
            let rot: Vec3 = transform.rotation.xyz();
            **span = format!("({rot:.2})\n");
        }

        for mut span in &mut spans.p1() {
            **span = format!("[{:.2}]\n", transform.translation);
        }

        for mut span in &mut spans.p2() {
            **span = format!("Hp: {:.2}/{:.2}\n", hp.rest(), hp.maximum());
        }
    }

    for mut span in &mut spans.p3() {
        **span = format!("Kill Counter: {:.2}\n", **kill_counter);
    }
}
