use godot::{
    classes::{AnimatedSprite2D, RayCast2D},
    prelude::*,
};

#[derive(GodotClass)]
#[class(init, base=Node2D)]
pub struct Enemy {
    #[export]
    speed: f32,
    #[export]
    direction: f32,
    #[init(node = "GroundCasts")]
    ground_casts: OnReady<Gd<Node2D>>,
    #[init(node = "WallCasts")]
    wall_casts: OnReady<Gd<Node2D>>,
    base: Base<Node2D>,
}

#[godot_api]
impl INode2D for Enemy {
    fn physics_process(&mut self, delta: f64) {
        let wall_collision =
            self.wall_casts.get_children().iter_shared().find(|node| {
                let raycast = node.get_node_as::<RayCast2D>(".");
                raycast.is_colliding()
            });

        let no_ground =
            self.ground_casts.get_children().iter_shared().find(|node| {
                let raycast = node.get_node_as::<RayCast2D>(".");
                !raycast.is_colliding()
            });

        if wall_collision.is_some() || no_ground.is_some() {
            self.direction *= -1.;
            let mut sprite = self
                .base_mut()
                .get_node_as::<AnimatedSprite2D>("EnemySprite");
            let flip_h = !sprite.is_flipped_h();
            sprite.set_flip_h(flip_h);
        }

        let mut position = self.base().get_position();
        position.x += self.speed * self.direction * delta as f32;
        self.base_mut().set_position(position);
    }
}
