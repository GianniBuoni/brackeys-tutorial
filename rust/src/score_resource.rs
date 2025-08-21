use godot::prelude::*;

#[derive(GodotClass)]
#[class(tool, init, base=Resource)]
pub struct ScoreResource {
    #[var(get, set = set_score)]
    score: i32,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for ScoreResource {
    fn get_property(&self, prop: StringName) -> Option<Variant> {
        match prop.to_string().as_str() {
            "score" => Some(Variant::from(self.score)),
            _ => None,
        }
    }
    fn set_property(&mut self, prop: StringName, val: Variant) -> bool {
        match prop.to_string().as_str() {
            "score" => {
                let Ok(val) = val.try_to::<i32>() else {
                    return false;
                };
                self.set_score(val);
                self.signals().score_changed().emit(val);
            }
            _ => {
                return false;
            }
        };
        true
    }
}

#[godot_api]
impl ScoreResource {
    #[signal]
    pub fn score_changed(val: i32);

    #[func]
    fn set_score(&mut self, value: i32) {
        self.score = value
    }
}
