use bevy::prelude::*;
use godot::{classes::Engine, prelude::*};
use godot_bevy::prelude::*;

use crate::GameState;

mod killplane;

pub mod prelude {
    pub use super::{EKillPlayer, EResetScene};
}

pub fn plugin(app: &mut App) {
    app.add_plugins(killplane::KillplanePlugin);
    app.add_systems(Update, reset_scene.run_if(in_state(GameState::Loading)));
}

#[derive(Event)]
pub struct EKillPlayer;

#[derive(Event)]
pub struct EResetScene;

#[main_thread_system]
fn reset_scene(
    mut reset_event: EventReader<EResetScene>,
    mut scene_tree: SceneTreeRef,
    mut state: ResMut<NextState<GameState>>,
) {
    for _ in reset_event.read() {
        scene_tree.get().reload_current_scene();
        state.set(GameState::InGame);
        Engine::singleton().set_time_scale(1.);
        info!("Scene Reloaded");
    }
}
