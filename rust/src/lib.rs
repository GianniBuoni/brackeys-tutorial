#![allow(unexpected_cfgs)]
#![allow(clippy::type_complexity)]
use std::{fmt::Display, sync::Arc};

use bevy::{prelude::*, state::app::StatesPlugin};
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};
use godot::prelude::*;
use godot_bevy::prelude::*;

mod actors;
mod gameplay;
mod ui;

pub mod prelude {
    pub use super::gameplay::*;
    pub use super::{GameState, NodeError};
    pub use anyhow;
    pub use anyhow::Ok as Aok;
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
    app.add_plugins(gameplay::plugin);
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

#[derive(Debug)]
pub enum NodeError {
    /// (root node, path)
    NotFound(Arc<str>, Arc<str>),
}

impl std::error::Error for NodeError {}

impl std::fmt::Display for NodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound(root, path) => {
                write!(f, "{root}: {path} not found.")
            }
        }
    }
}

impl NodeError {
    pub fn not_found(root_name: impl Display, path: impl Display) -> Self {
        Self::NotFound(root_name.to_string().into(), path.to_string().into())
    }
}
