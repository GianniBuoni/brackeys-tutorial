use bevy::prelude::*;
use godot::{classes::Engine, prelude::*};
use godot_bevy::prelude::*;

use crate::prelude::*;

mod killplane;

pub mod prelude {
    pub use super::EKillPlayer;
}

pub fn plugin(app: &mut App) {
    app.add_plugins(killplane::KillplanePlugin);
    app.add_systems(OnEnter(GameState::Reloading), reset_scene);
}

#[derive(Event)]
pub struct EKillPlayer;

#[main_thread_system]
fn reset_scene(
    mut scene_tree: SceneTreeRef,
    mut state: ResMut<NextState<GameState>>,
) {
    scene_tree.get().reload_current_scene();
    Engine::singleton().set_time_scale(1.);
    state.set(GameState::InGame);
    info!("Scene Reloaded");
}
