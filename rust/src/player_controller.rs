use godot::{
    classes::{AnimatedSprite2D, CharacterBody2D, ICharacterBody2D, Input},
    global::move_toward,
    prelude::*,
};

use crate::prelude::*;

#[derive(GodotClass)]
#[class(init, base=CharacterBody2D)]
pub struct PlayerController {
    #[init(node = "AnimatedSprite2D")]
    sprite: OnReady<Gd<AnimatedSprite2D>>,
    base: Base<CharacterBody2D>,
}

#[godot_api]
impl ICharacterBody2D for PlayerController {
    fn physics_process(&mut self, delta: f32) {
        let base = self.base();
        let mut velocity = self.base().get_velocity();

        // add gravity
        if !base.is_on_floor() {
            velocity += base.get_gravity() * delta;
        }
        // handle jump
        if Input::singleton().is_action_just_pressed("jump")
            && base.is_on_floor()
        {
            velocity.y = PLAYER_JUMP;
        }
        // direction -1, 0, 1
        let direction = Input::singleton().get_axis("move_left", "move_right");

        self.set_flip(direction);
        self.set_animation(direction);
        self.set_velocity(direction, &mut velocity);
    }
}

impl PlayerController {
    fn set_animation(&mut self, direction: f32) {
        if !self.base().is_on_floor() {
            self.sprite.set_animation("jump");
            return;
        }
        match direction {
            -1. | 1. => {
                self.sprite.set_animation("run");
            }
            _ => {
                self.sprite.set_animation("idle");
            }
        }
    }
    fn set_flip(&mut self, direction: f32) {
        match direction {
            -1. => {
                self.sprite.set_flip_h(true);
            }
            1. => {
                self.sprite.set_flip_h(false);
            }
            _ => {}
        }
    }
    fn set_velocity(&mut self, direction: f32, velocity: &mut Vector2) {
        match direction {
            dir if dir != 0. => velocity.x = direction * PLAYER_SPEED,
            _ => {
                velocity.x =
                    move_toward(velocity.x as f64, 0., PLAYER_SPEED as f64)
                        as f32
            }
        }
        let mut base = self.base_mut();
        base.set_velocity(*velocity);
        base.move_and_slide();
    }
}
