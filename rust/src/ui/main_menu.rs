use crate::GameState;

use super::*;
use menu_plugin::prelude::*;

pub fn plugin(app: &mut App) {
    let mut plugin = MenuPlugin::<MenuAssets<EMainMenu>, GameState>::default();
    plugin.with_state(GameState::MainMenu);
    plugin.with_name("Main Menu");

    app.add_plugins(plugin);
    app.add_systems(
        Update,
        init.run_if(MenuAssets::<EMainMenu>::not_init)
            .run_if(in_state(GameState::MainMenu)),
    );
    app.add_observer(start_game);
    app.add_observer(quit_game);
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

#[derive(Event, Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub enum EMainMenu {
    Start,
    Quit,
}

fn init(
    mut menu_assets: ResMut<MenuAssets<EMainMenu>>,
    mut scene_tree: SceneTreeRef,
) {
    let Some(root) = scene_tree.get().get_root() else {
        return;
    };
    let nodes = MainMenuTree::from_node(root);
    menu_assets.register_button(EMainMenu::Start, nodes.start_button.clone());
    menu_assets.register_button(EMainMenu::Quit, nodes.quit_button.clone());
    info!("{}: node initialized!", menu_assets.name);
    menu_assets.initialized = true;
}

fn start_game(
    trigger: Trigger<EMainMenu>,
    mut scene_tree: SceneTreeRef,
    mut state: ResMut<NextState<GameState>>,
) {
    if *trigger.event() == EMainMenu::Start {
        info!("Main Menu: starting game.");
        scene_tree
            .get()
            .change_scene_to_file("./scenes/levels/level_01.tscn");
        state.set(GameState::InGame);
    }
}

fn quit_game(trigger: Trigger<EMainMenu>, mut scene_tree: SceneTreeRef) {
    if *trigger.event() == EMainMenu::Quit {
        info!("Main Menu: quitting game.");
        scene_tree.get().quit();
    }
}
