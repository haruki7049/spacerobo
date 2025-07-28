use crate::GameMode;
use bevy::prelude::*;

pub fn setup_system(mut commands: Commands) {
    commands.spawn((StateScoped(GameMode::Title), Camera2d));

    commands
        .spawn((
            StateScoped(GameMode::Title),
            Node {
                margin: UiRect::all(Val::Percent(2.)),
                ..default()
            },
            Text::default(),
        ))
        .with_child(TextSpan::new(format!(
            "Spacerobo v{}\n",
            env!("CARGO_PKG_VERSION")
        )))
        .with_child(TextSpan::new(
            "Press space key => Shooting Range\n".to_string(),
        ))
        .with_child(TextSpan::new(
            "Press enter key => Versus <WIP>\n".to_string(),
        ));
}

pub fn input_detection_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut gamemode: ResMut<NextState<GameMode>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        gamemode.set(GameMode::ShootingRange);
    }

    if keyboard.just_pressed(KeyCode::Enter) {
        gamemode.set(GameMode::Versus);
    }
}
