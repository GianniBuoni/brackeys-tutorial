use godot::classes::Label;

use super::*;

#[derive(NodeTreeView)]
struct HudTree {
    #[node("*/Hud/MarginContainer/HBoxContainer/CoinLabel")]
    label: GodotNodeHandle,
}

#[derive(Resource, Default)]
struct RHudReady {
    label: Option<GodotNodeHandle>,
}

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RHudReady>();
        app.add_systems(OnEnter(GameState::InGame), reset);
        app.add_systems(
            Update,
            init.run_if(not_ready).run_if(in_state(GameState::InGame)),
        );
        // Trigger label update on ready for when score isn't 0.
        app.add_systems(
            Update,
            update_label
                .run_if(ready)
                .run_if(resource_changed::<RHudReady>)
                .run_if(in_state(GameState::InGame)),
        );
        app.add_systems(
            Update,
            update_label
                .run_if(ready)
                .run_if(resource_changed::<PlayerScore>)
                .run_if(in_state(GameState::InGame)),
        );
    }
}

fn reset(mut hud_ready: ResMut<RHudReady>) {
    hud_ready.label = None;
}

fn init(mut scene_tree: SceneTreeRef, mut hud_ready: ResMut<RHudReady>) {
    let Some(root) = scene_tree.get().get_root() else {
        error_once!("{}", NodeError::not_found("Hud", "root"));
        return;
    };
    let hud = HudTree::from_node(root);
    hud_ready.label = Some(hud.label.clone());
    info!("Hud: label ready!");
}

fn ready(hud_ready: Res<RHudReady>) -> bool {
    hud_ready.label.is_some()
}

fn not_ready(hud_ready: Res<RHudReady>) -> bool {
    hud_ready.label.is_none()
}

#[main_thread_system]
fn update_label(hud_ready: Res<RHudReady>, score: Res<PlayerScore>) {
    let Some(mut label) = hud_ready.label.clone() else {
        error_once!("Hud: update system called, but not ready.");
        return;
    };
    let Some(mut label) = label.try_get::<Label>() else {
        error_once!("{}", NodeError::not_found("Hud", "Label"));
        return;
    };
    label.set_text(score.0.to_string().as_str());
}
