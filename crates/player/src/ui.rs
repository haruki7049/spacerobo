//! # UI systems, components & etc...

#![allow(clippy::type_complexity)]

use super::Common as PlayerCommon;
use bevy::prelude::*;
use spacerobo_commons::{GameMode, Hp, KillCounter};

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
        .spawn((Text::default(), DespawnOnExit(GameMode::InGame)))
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
    transform_query: Query<&Transform, (With<PlayerCommon>, Changed<Transform>)>,
    hp_query: Query<&Hp, (With<PlayerCommon>, Changed<Hp>)>,
    kill_counter: Res<KillCounter>,
) {
    if let Ok(transform) = transform_query.single() {
        let rot: Vec3 = transform.rotation.xyz();
        for mut span in &mut spans.p0() {
            **span = format!("({rot:.2})\n");
        }

        for mut span in &mut spans.p1() {
            **span = format!("[{:.2}]\n", transform.translation);
        }
    }

    if let Ok(hp) = hp_query.single() {
        for mut span in &mut spans.p2() {
            **span = format!("Hp: {:.2}/{:.2}\n", hp.rest, hp.maximum);
        }
    }

    if kill_counter.is_changed() {
        for mut span in &mut spans.p3() {
            **span = format!("Kill Counter: {:.2}\n", **kill_counter);
        }
    }
}

#[cfg(test)]
mod tests {
    /// `update_system`'s unit tests
    mod update_system {
        use super::super::{HpUI, KillCounterUI, PlayerCommon, update_system};
        use bevy::prelude::*;
        use spacerobo_commons::{Hp, KillCounter};

        fn app_with_player() -> App {
            let mut app = App::new();
            app.add_systems(Update, update_system);
            app.insert_resource(KillCounter::default());
            app.world_mut()
                .spawn((Transform::default(), Hp::new(50., None), PlayerCommon));
            app.world_mut().spawn((TextSpan::default(), HpUI));
            app.world_mut().spawn((TextSpan::default(), KillCounterUI));
            app
        }

        fn hp_span_text(app: &mut App) -> String {
            app.world_mut()
                .query_filtered::<&TextSpan, With<HpUI>>()
                .single(app.world())
                .unwrap()
                .0
                .clone()
        }

        fn kill_counter_span_text(app: &mut App) -> String {
            app.world_mut()
                .query_filtered::<&TextSpan, With<KillCounterUI>>()
                .single(app.world())
                .unwrap()
                .0
                .clone()
        }

        /// On the first update, the HP span is populated from the player's `Hp`.
        #[test]
        fn sets_the_hp_span_on_the_first_update() {
            let mut app = app_with_player();

            app.update();

            assert_eq!(hp_span_text(&mut app), "Hp: 50.00/50.00\n");
        }

        /// Without an `Hp` change, a later update leaves the HP span untouched.
        #[test]
        fn leaves_the_hp_span_untouched_without_a_change() {
            let mut app = app_with_player();
            app.update();

            // Overwrite with a sentinel the real system would never produce.
            app.world_mut()
                .query_filtered::<&mut TextSpan, With<HpUI>>()
                .single_mut(app.world_mut())
                .unwrap()
                .0 = "sentinel".to_string();

            app.update();

            assert_eq!(hp_span_text(&mut app), "sentinel");
        }

        /// Changing `Hp` causes the next update to refresh the HP span.
        #[test]
        fn refreshes_the_hp_span_after_a_change() {
            let mut app = app_with_player();
            app.update();

            let mut query = app.world_mut().query::<&mut Hp>();
            query.single_mut(app.world_mut()).unwrap().rest = 25.;

            app.update();

            assert_eq!(hp_span_text(&mut app), "Hp: 25.00/50.00\n");
        }

        /// On the first update, the kill-counter span is populated.
        #[test]
        fn sets_the_kill_counter_span_on_the_first_update() {
            let mut app = app_with_player();

            app.update();

            assert_eq!(kill_counter_span_text(&mut app), "Kill Counter: 0\n");
        }

        /// Without a `KillCounter` change, a later update leaves its span untouched.
        #[test]
        fn leaves_the_kill_counter_span_untouched_without_a_change() {
            let mut app = app_with_player();
            app.update();

            app.world_mut()
                .query_filtered::<&mut TextSpan, With<KillCounterUI>>()
                .single_mut(app.world_mut())
                .unwrap()
                .0 = "sentinel".to_string();

            app.update();

            assert_eq!(kill_counter_span_text(&mut app), "sentinel");
        }

        /// Incrementing the kill counter causes the next update to refresh its span.
        #[test]
        fn refreshes_the_kill_counter_span_after_a_change() {
            let mut app = app_with_player();
            app.update();

            app.world_mut().resource_mut::<KillCounter>().increment();

            app.update();

            assert_eq!(kill_counter_span_text(&mut app), "Kill Counter: 1\n");
        }
    }
}
