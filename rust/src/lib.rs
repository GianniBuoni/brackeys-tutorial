use godot::prelude::*;

mod killplane;
mod pickup_component;
mod player_controller;

pub mod prelude {
    pub use super::constants::*;
}
pub mod constants {
    pub const PLAYER_SPEED: f32 = 130.;
    pub const PLAYER_JUMP: f32 = -300.;
}

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
