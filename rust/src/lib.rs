#![allow(unexpected_cfgs)]
use bevy::{prelude::*, state::app::StatesPlugin};
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};
use godot::prelude::*;
use godot_bevy::prelude::*;

mod actors;
mod ui;

pub mod prelude {
    pub use super::GameState;
}

#[bevy_app]
fn build_app(app: &mut App) {
    app.add_plugins(GodotDefaultPlugins);
    app.add_plugins(StatesPlugin)
        .init_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::MainMenu),
        );
    app.add_plugins(ui::plugin);
    app.add_plugins(actors::plugin);
    app.add_systems(Startup, hello_world);
}

fn hello_world() {
    info!("Hello from bevy!")
}

#[allow(dead_code)]
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Loading,
    MainMenu,
    PauseMenu,
    InGame,
}
