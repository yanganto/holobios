use bevy::prelude::*;

mod game;
mod help;

// TODO use SubApp for Doc to save resource
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<help::AppState>()
        .add_plugins(help::HelpPlugin)
        .add_plugins(game::GamePlugin)
        .run();
}
