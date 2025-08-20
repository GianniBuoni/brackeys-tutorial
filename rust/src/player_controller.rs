use godot::{
    classes::{CharacterBody2D, ICharacterBody2D, Input},
    global::move_toward,
    prelude::*,
};

use crate::prelude::*;

#[derive(GodotClass)]
#[class(init, base=CharacterBody2D)]
pub struct PlayerController {
    base: Base<CharacterBody2D>,
}

#[godot_api]
impl ICharacterBody2D for PlayerController {
    fn physics_process(&mut self, delta: f32) {
        let mut base = self.base_mut();
        let mut velocity = base.get_velocity();

        // add gravity
        if !base.is_on_floor() {
            velocity += base.get_gravity() * delta;
        }
        // handle jump
        if Input::singleton().is_action_just_pressed("ui_accept")
            && base.is_on_floor()
        {
            velocity.y = PLAYER_JUMP;
        }
        // handle input
        let direction = Input::singleton().get_axis("ui_left", "ui_right");
        match direction {
            dir if dir != 0. => velocity.x = direction * PLAYER_SPEED,
            _ => {
                velocity.x =
                    move_toward(velocity.x as f64, 0., PLAYER_SPEED as f64)
                        as f32
            }
        }
        base.set_velocity(velocity);
        base.move_and_slide();
    }
}
