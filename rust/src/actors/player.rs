use godot::classes::{
    AnimatedSprite2D, CharacterBody2D, CollisionShape2D, Input,
};
use godot_bevy::utils::move_toward;

use crate::prelude::*;

use super::*;

/// Godot representation of the Player Node
#[derive(GodotClass, BevyBundle)]
#[class(init, base = CharacterBody2D)]
#[bevy_bundle((Player), (JumpVelocity: jump_velocity), (Speed: speed))]
pub struct PlayerNode {
    base: Base<CharacterBody2D>,
    #[export]
    #[init(val = -300.)]
    jump_velocity: f32,
    #[export]
    #[init(val = 150.)]
    speed: f32,
}

#[derive(Event, Debug)]
pub struct EPlayerInput {
    pub move_dir: f32,
    pub jump_pressed: bool,
}

#[derive(Event, Debug)]
pub struct EPlayerMovement {
    pub is_moving: bool,
    pub grounded: bool,
    pub facing_left: bool,
}

#[derive(Component, Default)]
struct Player;

pub struct PlayerControllerPlugin;

impl Plugin for PlayerControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EPlayerInput>();
        app.add_event::<EPlayerMovement>();
        app.add_systems(
            PhysicsUpdate,
            (detect_input, set_movement, set_animation)
                .chain()
                .run_if(in_state(GameState::InGame)),
        );
        app.add_systems(
            PhysicsUpdate,
            kill.run_if(in_state(GameState::InGame)),
        );
    }
}

#[main_thread_system]
fn detect_input(
    mut player: Query<&mut GodotNodeHandle, With<Player>>,
    mut input_events: EventWriter<EPlayerInput>,
) {
    // if there is no player node, then no input events are recorded
    let Ok(mut handle) = player.single_mut() else {
        return;
    };
    let Some(_) = handle.try_get::<CharacterBody2D>() else {
        return;
    };
    let input = Input::singleton();
    let move_dir = input.get_axis("move_left", "move_right");
    let jump_pressed = input.is_action_just_pressed("jump");

    input_events.write(EPlayerInput {
        move_dir,
        jump_pressed,
    });
}

#[main_thread_system]
fn set_movement(
    mut player: Query<
        (&mut GodotNodeHandle, &Speed, &JumpVelocity),
        With<Player>,
    >,
    mut input_events: EventReader<EPlayerInput>,
    mut movement_events: EventWriter<EPlayerMovement>,
    physics_delta: Res<PhysicsDelta>,
) {
    let Ok((mut handle, speed, jump_velocity)) = player.single_mut() else {
        return;
    };
    let Some(mut char_body) = handle.try_get::<CharacterBody2D>() else {
        return;
    };

    let mut velocity = char_body.get_velocity();
    let grounded = char_body.is_on_floor();
    let mut is_moving = false;
    let mut dir = 0.0;

    // add gravity
    if !char_body.is_on_floor() {
        velocity += char_body.get_gravity() * physics_delta.delta_seconds;
    }
    // handle input event if there are any
    input_events.read().for_each(|f| {
        // v movement
        if f.jump_pressed && grounded {
            velocity.y = jump_velocity.0;
        }
        // h movement
        match f.move_dir {
            move_dir if move_dir != 0. => {
                velocity.x = f.move_dir * speed.0;
                dir = velocity.x;
                is_moving = true;
            }
            _ => velocity.x = move_toward(velocity.x, 0., speed.0),
        }
    });

    movement_events.write(EPlayerMovement {
        is_moving,
        grounded,
        facing_left: dir < 0.,
    });

    char_body.set_velocity(velocity);
    char_body.move_and_slide();
}

#[main_thread_system]
fn set_animation(
    mut player: Query<&mut GodotNodeHandle, With<Player>>,
    mut movement_events: EventReader<EPlayerMovement>,
) {
    let Ok(mut handle) = player.single_mut() else {
        return;
    };
    let Some(char_body) = handle.try_get::<CharacterBody2D>() else {
        return;
    };
    let Some(mut sprite) =
        char_body.try_get_node_as::<AnimatedSprite2D>("AnimatedSprite2D")
    else {
        return;
    };
    movement_events.read().for_each(|f| {
        if !f.grounded {
            sprite.play_ex().name("jump").done();
            return;
        }
        match f.is_moving {
            true => {
                sprite.play_ex().name("run").done();
                sprite.set_flip_h(f.facing_left);
            }
            false => sprite.play_ex().name("idle").done(),
        }
    });
}

#[main_thread_system]
fn kill(
    mut player: Query<&mut GodotNodeHandle, With<Player>>,
    kill_player: EventReader<EKillPlayer>,
) {
    if kill_player.is_empty() {
        return;
    }
    let Ok(mut handle) = player.single_mut() else {
        return;
    };
    let Some(char_body) = handle.try_get::<CharacterBody2D>() else {
        return;
    };
    let Some(mut collider) =
        char_body.try_get_node_as::<CollisionShape2D>("CollisionShape2D")
    else {
        return;
    };
    collider.set_disabled(true);
}
