use godot::{
    classes::{ILabel, Label},
    prelude::*,
};

use crate::prelude::*;
use crate::score_resource::ScoreResource;

#[derive(GodotClass)]
#[class(init,base=Label)]
pub struct ScoreLabel {
    #[init(val = OnReady::from_loaded(SCORE_RESOURCE))]
    score_resource: OnReady<Gd<ScoreResource>>,
    base: Base<Label>,
}

#[godot_api]
impl ILabel for ScoreLabel {
    fn ready(&mut self) {
        self.score_resource
            .signals()
            .score_changed()
            .connect_other(self, Self::update_score_text);
    }
}

#[godot_api]
impl ScoreLabel {
    #[func]
    fn update_score_text(&mut self, val: i32) {
        let text = format!("WOW!\n You got\n {val} coins.");
        self.base_mut().set_text(&text);
    }
}
