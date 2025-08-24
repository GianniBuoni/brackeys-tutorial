use super::*;

pub mod prelude {
    pub use super::main_menu_plugin;
}

pub fn main_menu_plugin(app: &mut App) {
    app.add_plugins(MenuPlugin::<MenuAssets<EMainMenu>>::new());
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

impl MenuConstruct for MenuAssets<EMainMenu> {
    fn init(mut menu_assets: ResMut<Self>, mut scene_tree: SceneTreeRef) {
        let Some(root) = scene_tree.get().get_root() else {
            return;
        };
        let nodes = MainMenuTree::from_node(root);
        menu_assets
            .register_button(EMainMenu::Start, nodes.start_button.clone());
        menu_assets.register_button(EMainMenu::Quit, nodes.quit_button.clone());
        info!("Main Menu: node initialized!");
        menu_assets.initialized = true;
    }
}

fn start_game(trigger: Trigger<EMainMenu>, mut scene_tree: SceneTreeRef) {
    if *trigger.event() == EMainMenu::Start {
        info!("Main Menu: starting game.");
        scene_tree
            .get()
            .change_scene_to_file("./scenes/levels/level_01.tscn");
    }
}

fn quit_game(trigger: Trigger<EMainMenu>, mut scene_tree: SceneTreeRef) {
    if *trigger.event() == EMainMenu::Quit {
        info!("Main Menu: quitting game.");
        scene_tree.get().quit();
    }
}
