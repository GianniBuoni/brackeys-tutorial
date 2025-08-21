use godot::{
    classes::{AnimationPlayer, Area2D, IArea2D},
    prelude::*,
};

use crate::prelude::*;
use crate::score_resource::ScoreResource;

#[derive(GodotClass)]
#[class(init, base=Area2D)]
pub struct Coin {
    #[init(val = OnReady::from_loaded(SCORE_RESOURCE))]
    score_resource: OnReady<Gd<ScoreResource>>,
    #[init(node = "AnimationPlayer")]
    animation_player: OnReady<Gd<AnimationPlayer>>,
    base: Base<Area2D>,
}

#[godot_api]
impl IArea2D for Coin {
    fn ready(&mut self) {
        self.signals().body_entered().connect_self(Self::add_coin);
    }
}

#[godot_api]
impl Coin {
    #[signal]
    fn picked_up();

    #[func]
    fn add_coin(&mut self, _body: Gd<Node2D>) {
        self.animation_player.play_ex().name("pickup").done();

        let Ok(score) = self.score_resource.get("score").try_to::<i32>() else {
            // TODO: handle error
            return;
        };
        let score = Variant::from(score + 1);
        self.score_resource.set("score", &score);
    }
}
