use godot::prelude::*;

mod enemy;
mod killplane;
mod pickup_component;
mod player_controller;
mod score_label;
mod score_resource;

pub mod prelude {
    pub use super::constants::*;
}
pub mod constants {
    pub const PLAYER_SPEED: f32 = 130.;
    pub const PLAYER_JUMP: f32 = -300.;
    pub const SCORE_RESOURCE: &str = "res://resources/score_resource.tres";
}

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
