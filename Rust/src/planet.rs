use godot::{classes::Node2D, obj::Base, prelude::*};

#[derive(GodotClass)]
#[class(base=Node2D, init)]
struct Planet {
    base: Base<Node2D>,
}

#[godot_api]
impl Planet {
    #[signal]
    fn planet_connected(center: Vector2);
}