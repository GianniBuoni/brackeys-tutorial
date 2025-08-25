use bevy::prelude::*;
use godot::prelude::*;
use godot_bevy::prelude::*;

mod player;

pub fn plugin(app: &mut App) {
    app.add_plugins(player::PlayerControllerPlugin);
}

#[derive(Component, Debug)]
struct JumpVelocity(f32);

#[derive(Component, Debug)]
struct Speed(f32);
