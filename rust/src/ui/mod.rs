use bevy::prelude::*;
use godot::{classes::Button, prelude::*};
use godot_bevy::prelude::*;

use crate::prelude::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MenuAssets>();
        app.add_systems(OnEnter(GameState::MainMenu), reset);
        app.add_systems(
            Update,
            (
                init_menu_assets.run_if(not_init),
                connect_buttons.run_if(no_connections),
                on_button_press.run_if(full_init),
            )
                .run_if(in_state(GameState::MainMenu)),
        );
    }
}
/// Describes the hierarchy of interactable elements
/// in the scene tree.
#[derive(NodeTreeView)]
pub struct MainMenuTree {
    #[node("*/Control/Options/StartButton")]
    start_button: GodotNodeHandle,
    #[node("*/Control/Options/QuitButton")]
    quit_button: GodotNodeHandle,
}
/// ESC representation of MainMenuTree with data
/// for initialization and signal connection checks.
#[derive(Resource, Default)]
pub struct MenuAssets {
    start_button: Option<GodotNodeHandle>,
    quit_button: Option<GodotNodeHandle>,
    initialized: bool,
    signals_connected: bool,
}

fn reset(mut menu_assets: ResMut<MenuAssets>) {
    menu_assets.start_button = None;
    menu_assets.quit_button = None;
    menu_assets.initialized = false;
    menu_assets.signals_connected = false;
}

#[main_thread_system]
fn init_menu_assets(
    mut menu_assets: ResMut<MenuAssets>,
    mut scene_tree: SceneTreeRef,
) {
    let Some(root) = scene_tree.get().get_root() else {
        // TODO handle error
        return;
    };
    // TODO: from_node() can panic?
    let menu_tree = MainMenuTree::from_node(root);
    info!("MainMenu: Found menu nodes.");
    menu_assets.start_button = Some(menu_tree.start_button);
    menu_assets.quit_button = Some(menu_tree.quit_button);
    menu_assets.initialized = true;
}

fn connect_buttons(mut menu_assets: ResMut<MenuAssets>, signals: GodotSignals) {
    let menu_assets = menu_assets.as_mut();
    let Some(start_button) = menu_assets.start_button.as_mut() else {
        return;
    };
    let Some(quit_button) = menu_assets.quit_button.as_mut() else {
        return;
    };
    signals.connect(start_button, "pressed");
    signals.connect(quit_button, "pressed");
    menu_assets.signals_connected = true;
}

// TODO: rewrite to use observer events?
fn on_button_press(
    mut events: EventReader<GodotSignal>,
    menu_assets: Res<MenuAssets>,
    mut _app_state: ResMut<NextState<GameState>>,
    // add event writer if needed
) {
    events
        .read()
        .filter(|f| f.target.clone().try_get::<Node>().is_some())
        .for_each(|f| {
            let Some(start_button) = &menu_assets.start_button else {
                return;
            };
            if &f.target == start_button {
                info!("Main Menu: Start button pressed.");
                // load game here
            }
            let Some(quit_button) = &menu_assets.quit_button else {
                return;
            };
            if &f.target == quit_button {
                info!("Main Menu: Quit button pressed.");
                if let Some(button) = f.target.clone().try_get::<Button>()
                    && let Some(mut tree) = button.get_tree()
                {
                    tree.quit();
                }
            }
        });
}

// Helper functions for system running
fn not_init(menu_assets: Res<MenuAssets>) -> bool {
    !menu_assets.initialized
}
fn no_connections(menu_assets: Res<MenuAssets>) -> bool {
    !menu_assets.signals_connected
}
fn full_init(menu_assets: Res<MenuAssets>) -> bool {
    menu_assets.initialized && menu_assets.signals_connected
}
