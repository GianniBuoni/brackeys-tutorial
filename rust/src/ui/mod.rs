use std::{collections::BTreeMap, marker::PhantomData};

use bevy::prelude::*;
use godot_bevy::prelude::*;

use crate::prelude::*;

mod main_menu;

pub mod prelude {
    pub use super::main_menu::prelude::*;
}

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

trait MenuConstruct
where
    Self: bevy::prelude::Resource + Default,
{
    // setters and constructors
    fn init(menu_assets: ResMut<Self>, scene_tree: SceneTreeRef);
    fn connect_signals(menu_assets: ResMut<Self>, signals: GodotSignals);

    // handler
    fn handle_events(
        menu_assets: Res<Self>,
        events: EventReader<GodotSignal>,
        cmd: Commands,
    );
}

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
}

/// ESC representation of MainMenuTree with data
/// for initialization and signal connection checks.
#[derive(Default, PartialEq, Resource)]
struct MenuAssets<E>
where
    E: Event + Default + Ord,
{
    buttons: BTreeMap<E, GodotNodeHandle>,
    initialized: bool,
    signals_connected: bool,
}

impl<E> MenuGetSet for MenuAssets<E>
where
    Self: bevy::prelude::Resource + Default + PartialEq,
    E: Event + Default + Ord,
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
}

impl<E> MenuAssets<E>
where
    E: Event + Default + Ord,
{
    fn register_button(&mut self, event: E, node_handle: GodotNodeHandle) {
        self.buttons.entry(event).or_insert(node_handle);
    }
}
