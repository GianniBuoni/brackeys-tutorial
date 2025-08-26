use godot::classes::{Area2D, Engine};

use crate::GameState;

use super::*;

#[derive(GodotClass, BevyBundle)]
#[class(init, base=Area2D)]
#[bevy_bundle((Killplane))]
pub struct KillPlaneNode {
    base: Base<Area2D>,
}

#[derive(Component, Default)]
struct Killplane;

#[derive(Resource, Default, PartialEq)]
struct RKillplanes(bool);

#[derive(Resource, PartialEq)]
struct RKillTimer {
    active: bool,
    timer: Timer,
}

impl Default for RKillTimer {
    fn default() -> Self {
        Self {
            active: false,
            timer: Timer::from_seconds(0.6, TimerMode::Once),
        }
    }
}

pub struct KillplanePlugin;

impl Plugin for KillplanePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RKillplanes>();
        app.init_resource::<RKillTimer>();
        app.add_event::<EKillPlayer>();
        app.add_event::<EResetScene>();
        app.add_systems(OnEnter(GameState::InGame), reset);
        app.add_systems(
            Update,
            connect_signals
                .run_if(not_init)
                .run_if(in_state(GameState::InGame)),
        );
        app.add_systems(
            PhysicsUpdate,
            (detect_collision, activate_timer, tick_timer, kill)
                .run_if(initialized)
                .run_if(in_state(GameState::InGame))
                .chain(),
        );
    }
}

fn reset(mut killplane_assets: ResMut<RKillplanes>) {
    killplane_assets.set_if_neq(RKillplanes::default());
}

#[main_thread_system]
fn connect_signals(
    mut killplanes: Query<&mut GodotNodeHandle, With<Killplane>>,
    mut killplane_assets: ResMut<RKillplanes>,
    signals: GodotSignals,
) {
    let killplanes = killplanes.iter_mut();
    let len = killplanes.len();

    killplanes.for_each(|mut node| {
        signals.connect(&mut node, "body_entered");
    });
    killplane_assets.0 = true;
    info!("Killplane: {len} initialized");
}

fn not_init(killplane_assets: Res<RKillplanes>) -> bool {
    !killplane_assets.0
}

fn initialized(killplane_assets: Res<RKillplanes>) -> bool {
    killplane_assets.0
}

#[main_thread_system]
fn detect_collision(
    killplanes: Query<&GodotNodeHandle, With<Killplane>>,
    mut signal_events: EventReader<GodotSignal>,
    mut kill_player: EventWriter<EKillPlayer>,
) {
    if signal_events.is_empty() || killplanes.is_empty() {
        return;
    }
    let targets = signal_events
        .read()
        .filter(|f| f.name == "body_entered")
        .map(|f| f.target.clone())
        .collect::<Vec<GodotNodeHandle>>();

    if targets.len() > 1 {
        info!("{}", targets.len());
    }

    if killplanes.iter().any(|f| targets.contains(f)) {
        kill_player.write(EKillPlayer);
        info!("Player death triggered.");
    };
}

#[main_thread_system]
fn activate_timer(
    mut kill_timer: ResMut<RKillTimer>,
    mut kill_player: EventReader<EKillPlayer>,
) {
    for _ in kill_player.read() {
        Engine::singleton().set_time_scale(0.5);
        kill_timer.active = true;
        info!("Player died.");
    }
}

fn tick_timer(mut kill_timer: ResMut<RKillTimer>, delta: Res<PhysicsDelta>) {
    if kill_timer.active {
        kill_timer.timer.tick(delta.delta());
    }
}

fn kill(
    mut kill_timer: ResMut<RKillTimer>,
    mut state: ResMut<NextState<GameState>>,
    mut reset_scene: EventWriter<EResetScene>,
) {
    if kill_timer.timer.just_finished() {
        kill_timer.set_if_neq(RKillTimer::default());
        state.set(GameState::Loading);
        reset_scene.write(EResetScene);
        info!("Kill timer finished: reload triggered.")
    }
}
