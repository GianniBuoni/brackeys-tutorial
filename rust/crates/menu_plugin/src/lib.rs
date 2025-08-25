use std::{
    collections::BTreeMap, fmt::Display, marker::PhantomData, sync::Arc,
};

use bevy::prelude::*;
use godot::{classes::Button, prelude::*};
use godot_bevy::prelude::*;

pub mod prelude {
    pub use super::{MenuAssets, MenuGetSet, MenuPlugin};
}

/// Plugin struct that adds a barebones menu to the app
/// where R is a Resource that implements the two traits exported
/// by this crate.
///
/// The user is responsible for supplying an init function to
/// map the menu to Godot's scene tree, and schedule it to run on
/// the Update group.
///
/// Example usage:
///
///```
///pub fn main_menu_plugin(app: &mut App) {
///     app.add_plugins(MenuPlugin::<MenuAssets<EMainMenu>, GameState>::new());
///     app.add_systems(Update, init.run_if(MenuAssets::<EMainMenu>::not_init));
///}
///```
pub struct MenuPlugin<R, S>
where
    R: MenuGetSet,
    S: States + Default + Copy,
{
    name: Arc<str>,
    state: S,
    _marker: PhantomData<R>,
}

impl<R, S> MenuPlugin<R, S>
where
    R: MenuGetSet,
    S: States + Default + Copy,
{
    pub fn with_state(&mut self, state: S) -> &mut Self {
        self.state = state;
        self
    }
    pub fn with_name(&mut self, name: impl Display) -> &mut Self {
        self.name = name.to_string().into();
        self
    }
}

impl<R, S> Default for MenuPlugin<R, S>
where
    R: MenuGetSet,
    S: States + Default + Copy,
{
    fn default() -> Self {
        Self {
            name: Arc::<str>::default(),
            state: S::default(),
            _marker: PhantomData,
        }
    }
}

impl<R, S> Plugin for MenuPlugin<R, S>
where
    R: MenuGetSet,
    S: States + Default + Copy,
{
    fn build(&self, app: &mut App) {
        info!("Iniializing resource: {}", self.name);
        let mut r = R::default();
        r.set_name(self.name.clone());

        app.insert_resource(r);
        app.add_systems(OnEnter(self.state), R::reset);
        app.add_systems(
            Update,
            (
                R::connect_signals.run_if(R::no_connections),
                R::handle_events.run_if(R::full_init),
            )
                .run_if(in_state(self.state)),
        );
    }
}

/// Trait responsible for getting and setting the plugin's
/// resource. If the menu resource is defined as a MenuAsset,
/// this trait is already implemented.
pub trait MenuGetSet
where
    Self: bevy::prelude::Resource + Default + PartialEq,
{
    // getters
    fn not_init(menu_assets: Res<Self>) -> bool;
    fn no_connections(menu_assets: Res<Self>) -> bool;
    fn full_init(menu_assets: Res<Self>) -> bool;

    // setters
    fn set_name(&mut self, name: Arc<str>);
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
    // provided setters
    fn reset(mut menu_assets: ResMut<Self>) {
        menu_assets.set_if_neq(Self::default());
    }
}

/// ESC representation of a menu with data
/// for initialization and signal connection checks.
/// This is the provided resource for the MenuPlugin.
///
/// The user is responsible for properly initalizing the
/// resource in the plugin setup!
#[derive(PartialEq, Resource)]
pub struct MenuAssets<E>
where
    E: Event + Ord + Copy,
{
    pub name: Arc<str>,
    pub buttons: BTreeMap<E, GodotNodeHandle>,
    pub initialized: bool,
    signals_connected: bool,
}

impl<E> Default for MenuAssets<E>
where
    E: Event + Ord + Copy,
{
    fn default() -> Self {
        Self {
            name: Arc::<str>::default(),
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
    fn set_name(&mut self, name: Arc<str>) {
        self.name = name
    }
    fn reset(mut menu_assets: ResMut<Self>) {
        menu_assets.buttons = BTreeMap::default();
        menu_assets.initialized = false;
        menu_assets.signals_connected = false;
    }
    fn connect_signals(mut menu_assets: ResMut<Self>, signals: GodotSignals) {
        menu_assets
            .buttons
            .values_mut()
            .enumerate()
            .for_each(|(i, f)| {
                signals.connect(f, "pressed");
                if i == 0 {
                    f.get::<Button>().grab_focus();
                }
            });
        info!("{}: signals connected!", menu_assets.name);
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
    pub fn register_button(&mut self, event: E, node_handle: GodotNodeHandle) {
        self.buttons.entry(event).or_insert(node_handle);
    }
}
