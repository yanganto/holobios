use bevy::prelude::*;

mod game;

// TODO use SubApp for Doc to save resource
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .add_plugins(game::GamePlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, show_help.run_if(in_state(AppState::Help)))
        .add_systems(Update, clear_help.run_if(in_state(AppState::Game)))
        .add_systems(Update, transition_to_game_state)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Text::new("Initializing...."));
}

fn show_help(mut text: Single<&mut Text>) {
    text.clear();
    text.push_str(
        "Use the arrow keys to move the selector.\n\
        Use the C key to drop a puzzle.\n\
        Use the R key to rotate a puzzle.\n\
        Use the H key to show the doc.\n\
        Use the Esc key to close the doc.\n\
        ",
    );
}

fn clear_help(mut commands: Commands, mut text: Single<&mut Text>) {
    text.clear();
}

pub fn transition_to_game_state(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_state.set(AppState::Game);
    }
    if keyboard_input.just_pressed(KeyCode::KeyH) {
        next_state.set(AppState::Help);
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    Help,
    Game,
}
