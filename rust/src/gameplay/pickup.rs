use godot::classes::{
    AnimationPlayer, Area2D,
    class_macros::sys::godot_virtual_consts::AnimatedSprite2D,
};

use super::*;

#[derive(GodotClass, BevyBundle)]
#[class(init, base=Area2D)]
#[bevy_bundle((Pickup: value))]
pub struct PickupNode {
    #[export]
    #[init(val = 1)]
    value: i32,
}

#[derive(Component, Debug, Default)]
struct Pickup(i32);

#[derive(Resource, Default)]
struct RPickupReady(bool);

#[derive(Resource, Default)]
struct RPickedUp(Vec<i32>);

pub struct PickupPlugin;

impl Plugin for PickupPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RPickupReady>();
        app.init_resource::<RPickedUp>();
        app.add_systems(OnEnter(GameState::InGame), reset_pickup_ready);
        app.add_systems(
            Update,
            connect_signals
                .run_if(not_ready)
                .run_if(in_state(GameState::InGame)),
        );
        app.add_systems(
            Update,
            (detect_collisons, update_score)
                .run_if(ready)
                .run_if(in_state(GameState::InGame)),
        );
    }
}

fn reset_pickup_ready(mut pickup_asset: ResMut<RPickupReady>) {
    pickup_asset.0 = false;
}

fn not_ready(pickup_asset: Res<RPickupReady>) -> bool {
    !pickup_asset.0
}
fn ready(pickup_asset: Res<RPickupReady>) -> bool {
    pickup_asset.0
}

#[main_thread_system]
fn connect_signals(
    mut pickups: Query<
        &mut GodotNodeHandle,
        (With<Pickup>, With<Area2DMarker>),
    >,
    mut pickup_ready: ResMut<RPickupReady>,
    signals: GodotSignals,
) {
    pickups
        .iter_mut()
        .for_each(|mut f| signals.connect(&mut f, "body_entered"));
    info!("Pickups: {} connected.", pickups.iter().len());
    pickup_ready.0 = true;
}

#[main_thread_system]
fn detect_collisons(
    mut pickups: Query<(&mut GodotNodeHandle, &Pickup), With<Area2DMarker>>,
    mut picked_up: ResMut<RPickedUp>,
    mut signals: EventReader<GodotSignal>,
) {
    info_once!("Pickup: polling for collisions!");
    let targets = signals
        .read()
        .filter(|f| f.name == "body_entered")
        .map(|f| f.target.clone())
        .collect::<Vec<GodotNodeHandle>>();

    let _ = pickups
        .iter_mut()
        .filter(|(f, _)| targets.contains(f))
        .try_for_each(|(mut node, val)| {
            let path = "AnimationPlayer";
            node.get::<Area2D>()
                .try_get_node_as::<AnimationPlayer>(path)
                .ok_or(NodeError::not_found("Pickup", path))?
                .play_ex()
                .name("pickup")
                .done();

            picked_up.0.push(val.0);
            Aok(())
        })
        .map_err(|e| error_once!("{e}"));
}

fn update_score(
    mut picked_up: ResMut<RPickedUp>,
    mut score: ResMut<PlayerScore>,
) {
    picked_up.0.iter().for_each(|f| score.0 += f);
    picked_up.0.clear();
}
