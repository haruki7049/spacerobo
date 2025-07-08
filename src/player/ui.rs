use bevy::prelude::*;
use crate::player::Player;

#[derive(Component)]
pub struct HeadingIndicator;

#[derive(Component)]
pub struct CoordinatesIndicator;

pub fn ui_system(
    mut spans: ParamSet<(
        Query<&mut TextSpan, With<HeadingIndicator>>,
        Query<&mut TextSpan, With<CoordinatesIndicator>>,
    )>,
    player_query: Query<&mut Transform, With<Player>>,
) {
    for mut span in &mut spans.p0() {
        for transform in &player_query {
            let rot: Vec3 = transform.rotation.xyz();
            **span = format!("({rot:.2})\n");
        }
    }

    for mut span in &mut spans.p1() {
        for transform in &player_query {
            **span = format!("[{:.2}]\n", transform.translation);
        }
    }
}

pub fn exit_system(mut exit: EventWriter<AppExit>, keyboard: Res<ButtonInput<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
}
