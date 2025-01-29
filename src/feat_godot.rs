use godot::{prelude::*, builtin::Vector3, classes::{INode3D, Node3D}};

pub struct OrbitExtension;

#[gdextension]
unsafe impl ExtensionLibrary for OrbitExtension {}


#[derive(GodotClass)]
#[class(base=Node3D)]
struct TestBody {
	spin_speed: f64,
	base: Base<Node3D>,
}
#[godot_api]
impl INode3D for TestBody {
	fn init(base: Base<Node3D>) -> Self {
		godot_print!("Initializing test orbital body");
		Self{ spin_speed: 1.0, base }
	}

	fn physics_process(&mut self, delta: f64) {
		let rotation = (self.spin_speed * delta) as f32;
		self.base_mut().rotate(Vector3::UP, rotation);
	}
}
