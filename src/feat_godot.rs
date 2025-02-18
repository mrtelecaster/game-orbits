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
impl INode for GodotPlanetDatabase {
	fn init(_base: Base<Node>) -> Self {
		Self{ database: Database::default() }
	}
}
#[godot_api]
impl GodotPlanetDatabase {
	#[func]
	pub fn add_solar_system(&mut self) {
		self.database.add_sol();
	}
	#[func]
	pub fn relative_position(&self, origin: i64, relative: i64, time: f32) -> Vector3 {
		vec_nalgebra_to_godot(self.database.relative_position(&origin, &relative, time).unwrap())
	}
	#[func]
	pub fn radius_soi(&self, handle: i64) -> f32 {
		self.database.radius_soi(&handle)
	}
}
