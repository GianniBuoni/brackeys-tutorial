use godot::classes::{Area2D, Input};

use super::*;

#[derive(GodotClass, BevyBundle)]
#[class(init, base = Area2D)]
#[bevy_bundle((DialogMarker))]
pub struct DialogMarkerNode {
    base: Base<Area2D>,
}

#[godot_api]
impl DialogMarkerNode {
    #[signal]
    fn trigger_dialog();
}

#[derive(Component, Default)]
pub struct DialogMarker;

pub struct DialogMarkerPlugin;

impl Plugin for DialogMarkerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, detect_input);
    }
}

#[main_thread_system]
fn detect_input(
    mut dialog_markers: Query<
        (&mut GodotNodeHandle, &Collisions),
        (With<Area2DMarker>, With<DialogMarker>),
    >,
) {
    dialog_markers
        .iter_mut()
        .for_each(|(mut node, collisions)| {
            if collisions.colliding().is_empty() {
                return;
            }
            let marker = node.get::<DialogMarkerNode>();
            if Input::singleton().is_action_just_pressed("ui_accept") {
                marker.signals().trigger_dialog().emit();
            }
        });
}
