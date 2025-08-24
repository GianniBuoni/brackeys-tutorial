use bevy::prelude::*;
use godot::prelude::*;
use godot_bevy::prelude::*;

use crate::prelude::*;

pub struct MenuPlugin;

// TODO: Decouple plugin from the Main Menu
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
        app.add_observer(quit_game);
        app.add_observer(start_game);
    }
}

#[derive(Debug, PartialEq, Event)]
pub enum MenuEvent {
    Start,
    Quit,
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
        return;
    };
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

fn on_button_press(
    menu_assets: Res<MenuAssets>,
    mut events: EventReader<GodotSignal>,
    mut cmd: Commands,
) {
    events
        .read()
        .filter(|f| f.target.clone().try_get::<Node>().is_some())
        .for_each(|f| {
            let Some(start_button) = menu_assets.start_button.as_ref() else {
                return;
            };
            let Some(quit_button) = menu_assets.quit_button.as_ref() else {
                return;
            };
            match &f.target {
                node if node == start_button => cmd.trigger(MenuEvent::Start),
                node if node == quit_button => cmd.trigger(MenuEvent::Quit),
                _ => {}
            }
        });
}

fn quit_game(trigger: Trigger<MenuEvent>, mut scene_tree: SceneTreeRef) {
    if *trigger.event() == MenuEvent::Quit {
        info!("Main Menu: Quit button pressed.");
        scene_tree.get().quit();
    }
}

fn start_game(trigger: Trigger<MenuEvent>, mut scene_tree: SceneTreeRef) {
    if *trigger.event() == MenuEvent::Start {
        info!("Main Menu: Start button pressed.");
        scene_tree
            .get()
            .change_scene_to_file("./scenes/levels/level_01.tscn");
    }
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
