use godot::{
    classes::{AnimationPlayer, Area2D, IArea2D},
    prelude::*,
};

#[derive(GodotClass)]
#[class(init, base=Area2D)]
pub struct Coin {
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
        godot_print!("Coin +1");
        self.animation_player.play_ex().name("pickup").done();
    }
}
