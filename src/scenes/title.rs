use crate::GameMode;
use bevy::prelude::*;

pub struct TitlePlugin;

impl Plugin for TitlePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameMode>();
        app.add_systems(OnEnter(GameMode::Title), setup_system);
        app.add_systems(
            Update,
            (input_detection_system).run_if(in_state(GameMode::Title)),
        );
    }
}

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
        .with_child(TextSpan::new("Press M key => Versus Master\n".to_string()))
        .with_child(TextSpan::new("Press G key => Versus Guest\n".to_string()))
        .with_child(TextSpan::new("Press escape key => Exit...\n".to_string()));
}

pub fn input_detection_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut gamemode: ResMut<NextState<GameMode>>,
    mut exit: EventWriter<AppExit>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        gamemode.set(GameMode::ShootingRange);
    }

    if keyboard.just_pressed(KeyCode::KeyM) {
        gamemode.set(GameMode::VersusMaster);
    }

    if keyboard.just_pressed(KeyCode::KeyG) {
        gamemode.set(GameMode::VersusGuest);
    }

    if keyboard.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
}
