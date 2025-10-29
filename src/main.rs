use bevy::prelude::*;

mod game;
mod help;

// TODO use SubApp for Doc to save resource
fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(bevy::render::RenderPlugin {
                render_creation: bevy::render::settings::WgpuSettings {
                    power_preference: bevy::render::settings::PowerPreference::LowPower,
                    ..Default::default()
                }
                .into(),
                ..Default::default()
            }),
        )
        .init_state::<help::AppState>()
        .add_plugins(help::HelpPlugin)
        .add_plugins(game::GamePlugin)
        .run();
}
