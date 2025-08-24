use std::{collections::BTreeMap, marker::PhantomData};

use bevy::prelude::*;
use godot::prelude::*;
use godot_bevy::prelude::*;

use crate::prelude::*;

mod main_menu;

pub mod prelude {
    pub use super::main_menu::prelude::*;
}

/// Plugin struct that adds a barebones menu to the app
/// where R is a Resource that implements the two traits exported
/// by this crate.
///
/// Example usage:
///
///```
///pub fn main_menu_plugin(app: &mut App) {
///     app.add_plugins(MenuPlugin::<MenuAssets<EMainMenu>>::new());
///}
///```
///
struct MenuPlugin<R>
where
    R: MenuConstruct + MenuGetSet,
{
    _marker: PhantomData<R>,
}

impl<R> MenuPlugin<R>
where
    R: MenuConstruct + MenuGetSet,
{
    fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<R> Plugin for MenuPlugin<R>
where
    R: MenuConstruct + MenuGetSet,
{
    fn build(&self, app: &mut App) {
        app.init_resource::<R>();
        app.add_systems(OnEnter(GameState::MainMenu), R::reset);
        app.add_systems(
            Update,
            (
                R::init.run_if(R::not_init),
                R::connect_signals.run_if(R::no_connections),
                R::handle_events.run_if(R::full_init),
            )
                .run_if(in_state(GameState::MainMenu)),
        );
    }
}

/// Trait responsible for the construction/initialization of the plugin's
/// MenuAsset resource.
/// Typical useage requires the user to implement this trait manually.
trait MenuConstruct
where
    Self: bevy::prelude::Resource + Default,
{
    /// Initlializies the MenuAsset resource. Typically a [`NodeTree`]
    /// is use/compared against the resource to get the available NodeHandles.
    /// in the Godot scene tree.
    fn init(menu_assets: ResMut<Self>, scene_tree: SceneTreeRef);
}

/// Trait responsible for getting and resseting the plugin's
/// resource. If the menu resource is defined as a MenuAsset,
/// this trait is already implemented.
trait MenuGetSet
where
    Self: bevy::prelude::Resource + Default + PartialEq,
{
    // getters
    fn not_init(menu_assets: Res<Self>) -> bool;
    fn no_connections(menu_assets: Res<Self>) -> bool;
    fn full_init(menu_assets: Res<Self>) -> bool;

    // provided setters
    fn reset(mut menu_assets: ResMut<Self>) {
        menu_assets.set_if_neq(Self::default());
    }
    /// After the resource is properly initialized, Godot signals are connected
    /// to the assigned Note Handles. Typically, the signal needed is "pressed,
    /// but the trait offers the flexibity to connect any signal that the
    /// target node supports.
    fn connect_signals(menu_assets: ResMut<Self>, signals: GodotSignals);
    /// Evenet listener, command is available to trigger any observer system
    /// defined by the user.
    fn handle_events(
        menu_assets: Res<Self>,
        events: EventReader<GodotSignal>,
        cmd: Commands,
    );
}

/// ESC representation of a menu with data
/// for initialization and signal connection checks.
/// This is the provided resource for the MenuPlugin.
#[derive(PartialEq, Resource)]
struct MenuAssets<E>
where
    E: Event + Ord + Copy,
{
    buttons: BTreeMap<E, GodotNodeHandle>,
    initialized: bool,
    signals_connected: bool,
}

impl<E> Default for MenuAssets<E>
where
    E: Event + Ord + Copy,
{
    fn default() -> Self {
        Self {
            buttons: BTreeMap::default(),
            initialized: false,
            signals_connected: false,
        }
    }
}

impl<E> MenuGetSet for MenuAssets<E>
where
    Self: bevy::prelude::Resource + Default,
    E: Event + Ord + Copy,
{
    // getters
    fn not_init(menu_assets: Res<Self>) -> bool {
        !menu_assets.initialized
    }
    fn no_connections(menu_assets: Res<Self>) -> bool {
        menu_assets.initialized && !menu_assets.signals_connected
    }
    fn full_init(menu_assets: Res<Self>) -> bool {
        menu_assets.initialized && menu_assets.signals_connected
    }
    // setters
    fn reset(mut menu_assets: ResMut<Self>) {
        menu_assets.set_if_neq(Self::default());
    }
    fn connect_signals(mut menu_assets: ResMut<Self>, signals: GodotSignals) {
        menu_assets
            .buttons
            .values_mut()
            .for_each(|f| signals.connect(f, "pressed"));
        info!("Main Menu: signals connected!");
        menu_assets.signals_connected = true;
    }
    fn handle_events(
        menu_assets: Res<Self>,
        mut events: EventReader<GodotSignal>,
        mut cmd: Commands,
    ) {
        events
            .read()
            .filter(|f| f.target.clone().try_get::<Node>().is_some())
            .for_each(|signal| {
                let Some(event) = menu_assets
                    .buttons
                    .iter()
                    .find(|(_, v)| **v == signal.target)
                else {
                    return;
                };
                cmd.trigger(*event.0);
            });
    }
}

impl<E> MenuAssets<E>
where
    E: Event + Ord + Copy,
{
    fn register_button(&mut self, event: E, node_handle: GodotNodeHandle) {
        self.buttons.entry(event).or_insert(node_handle);
    }
}
