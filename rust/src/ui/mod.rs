use bevy::prelude::*;
use godot_bevy::prelude::*;

mod main_menu;

pub fn plugin(app: &mut App) {
    app.add_plugins(main_menu::plugin);
}
