use bevy::prelude::*;
use godot::prelude::*;
use godot_bevy::prelude::*;

mod enemy;
mod player;

pub fn plugin(app: &mut App) {
    app.add_plugins(player::PlayerControllerPlugin);
    app.add_plugins(enemy::EnemyPlugin);
}

#[derive(Component, Debug)]
struct JumpVelocity(f32);

#[derive(Component, Debug)]
struct Speed(f32);

#[derive(Component, Debug)]
struct Direction(f32);

impl Default for Direction {
    fn default() -> Self {
        Self(1.)
    }
}

impl Direction {
    pub fn change(&mut self) {
        self.0 *= -1.;
    }
}
