use godot::{
    classes::{Area2D, CollisionShape2D, Engine, IArea2D, Timer},
    prelude::*,
};

use crate::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Area2D)]
pub struct KillPlane {
    #[init(val = OnReady::from_loaded(SCORE_RESOURCE))]
    score_resource: OnReady<Gd<Resource>>,
    #[init(node = "Timer")]
    reset_timer: OnReady<Gd<Timer>>,
    base: Base<Area2D>,
}

#[godot_api]
impl IArea2D for KillPlane {
    fn ready(&mut self) {
        self.signals()
            .body_entered()
            .connect_self(Self::kill_player);

        self.reset_timer
            .signals()
            .timeout()
            .connect_other(self, Self::reload_game);
    }
}

#[godot_api]
impl KillPlane {
    #[signal]
    fn kill();

    #[func]
    fn kill_player(&mut self, body: Gd<Node2D>) {
        godot_print!("You died.");
        Engine::singleton().set_time_scale(0.5);
        body.get_node_as::<CollisionShape2D>("CollisionShape2D")
            .queue_free();
        self.reset_timer.start();
    }

    #[func]
    fn reload_game(&mut self) {
        if let Some(mut scene) = self.reset_timer.get_tree() {
            scene.reload_current_scene();
            self.score_resource.set("score", &Variant::from(0));
            Engine::singleton().set_time_scale(1.);
        }
    }
}
